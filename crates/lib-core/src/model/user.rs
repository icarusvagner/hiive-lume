use lib_auth::pwd::{self, ContentToHash};
use lib_models::{
	authentication::user_account::{
		UserAccount, UserAccountForAuth, UserAccountForCreate, UserAccountForInsert, UserAccountForLogin
	}, types::UserSessionState
};
use modql::{
	field::{HasSeaFields, SeaField, SeaFields}, filter::{
		FilterNodes, ListOptions, OpValsInt64, OpValsString, OpValsValue
	}
};
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::Deserialize;
use sqlx::{FromRow, postgres::PgRow};

use crate::{
	ctx::Ctx, model::{
		Error, ModelManager, Result, base::{self, CommonIden, DbBmc, prep_fields_for_update}, modql_utils::time_to_sea_value
	}
};

#[derive(Iden)]
pub enum UserIden {
	Id,
	Username,
	Password,
	State,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct UserFilter {
	pub id: Option<OpValsInt64>,
	pub user_id: Option<OpValsString>,
	pub username: Option<OpValsString>,
	pub state: Option<OpValsValue>,

	pub cid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub ctime: Option<OpValsValue>,
	pub mid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub mtime: Option<OpValsValue>,
}

pub trait UserBy:
	HasSeaFields + for<'r> FromRow<'r, PgRow> + Unpin + Send
{
}

impl UserBy for UserAccount {}
impl UserBy for UserAccountForLogin {}
impl UserBy for UserAccountForAuth {}

pub struct UserBmc;

impl DbBmc for UserBmc {
	const TABLE: &'static str = "tbl_user_account";
}

impl UserBmc {
	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		user_c: UserAccountForCreate,
	) -> Result<i64> {
		let UserAccountForCreate { user_id, username, password } = user_c;

		// create the user row for insert
		let user_fi =
			UserAccountForInsert { user_id, username: username.to_string() };

		// start the transaction
		let mm = mm.new_with_txn()?;
		mm.dbx().begin_txn().await?;

		let user_id = base::create::<Self, _>(ctx, &mm, user_fi)
			.await
			.map_err(|model_err| {
				Error::resolve_unique_violation(
					model_err,
					Some(|table: &str, constraint: &str| {
						if table == "tbl_user_account"
							&& constraint.contains("username")
						{
							Some(Error::UserAlreadyExists { username })
						} else {
							None
						}
					}),
				)
			})?;

		// update the password
		Self::update_password(ctx, &mm, user_id, &password).await?;

		// commit transaction
		mm.dbx().commit_txn().await?;
		Ok(user_id)
	}

	pub async fn get<E>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
	where
		E: UserBy,
	{
		let mut query = Query::select();
		query
			.from(Self::table_ref())
			.columns(E::sea_column_refs())
			.and_where(Expr::col(CommonIden::Id).eq(id));

		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let sqlx_query = sqlx::query_as_with::<_, E, _>(&sql, values);
		let entity = mm
			.dbx()
			.fetch_optional(sqlx_query)
			.await?
			.ok_or(Error::EntityNotFound { entity: Self::TABLE, id })?;

		Ok(entity)
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filter: Option<Vec<UserFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<UserAccount>> {
		base::list::<Self, _, _>(ctx, mm, filter, list_options).await
	}

	pub async fn first_by_username<E>(
		_ctx: &Ctx,
		mm: &ModelManager,
		username: &str,
	) -> Result<Option<E>>
	where
		E: UserBy,
	{
		let mut query = Query::select();
		query
			.from(Self::table_ref())
			.columns(E::sea_idens())
			.and_where(Expr::col(UserIden::Username).eq(username));

		// execute query
		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let sqlx_query = sqlx::query_as_with::<_, E, _>(&sql, values);
		let entity = mm.dbx().fetch_optional(sqlx_query).await?;

		Ok(entity)
	}

	pub async fn update_password(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		pwd_clear: &str,
	) -> Result<()> {
		let user: UserAccountForLogin = Self::get(ctx, mm, id).await?;
		let pwd = pwd::hash_pwd(ContentToHash {
			content: pwd_clear.to_string(),
			salt: user.pass_salt,
		})
		.await?;

		let mut fields =
			SeaFields::new(vec![SeaField::new(UserIden::Password, pwd)]);
		prep_fields_for_update::<Self>(&mut fields, ctx.user_id());

		// build query
		let fields = fields.for_sea_update();
		let mut query = Query::update();
		query
			.table(Self::table_ref())
			.values(fields)
			.and_where(Expr::col(UserIden::Id).eq(id));

		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let sqlx_query = sqlx::query_with(&sql, values);
		let _count = mm.dbx().execute(sqlx_query).await?;

		Ok(())
	}

	pub async fn update_status(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		state: UserSessionState,
	) -> Result<()> {
		let mut fields =
			SeaFields::new(vec![SeaField::new(UserIden::State, state)]);
		prep_fields_for_update::<Self>(&mut fields, ctx.user_id());

		let fields = fields.for_sea_update();
		let mut query = Query::update();
		query
			.table(Self::table_ref())
			.values(fields)
			.and_where(Expr::col(UserIden::Id).eq(id));

		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let sqlx_query = sqlx::query_with(&sql, values);
		let _count = mm.dbx().execute(sqlx_query).await?;

		Ok(())
	}
}
