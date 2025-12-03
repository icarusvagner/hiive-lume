use lib_models::authentication::{
	role::{Role, RoleForCreate, RoleForInsert}, role_permission::{Permission, PermissionForCreate, PermissionForInsert}
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
		Error, ModelManager, Result, base::{self, CommonIden, DbBmc, prep_fields_for_update}, modql_utils::time_to_sea_value, prep_fields_for_create
	}
};

#[derive(Iden)]
pub enum RoleIden {
	Id,
	Name,
	Description,
	Status,
}

#[derive(Iden)]
pub enum PermissionIden {
	Id,
	Module,
	Action,
	Level,
	Status,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct RoleFilter {
	pub id: Option<OpValsInt64>,
	pub name: Option<OpValsString>,
	pub description: Option<OpValsString>,

	pub cid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub ctime: Option<OpValsValue>,
	pub mid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub mtime: Option<OpValsValue>,
}

pub trait RoleBy:
	HasSeaFields + for<'r> FromRow<'r, PgRow> + Unpin + Send
{
}

impl RoleBy for Role {}
impl RoleBy for Permission {}

pub struct RoleBmc;
pub struct PermissionBmc;

impl DbBmc for RoleBmc {
	const TABLE: &'static str = "tbl_role";

	fn has_timestamps() -> bool {
		true
	}
}

impl DbBmc for PermissionBmc {
	const TABLE: &'static str = "tbl_permission";

	fn has_timestamps() -> bool {
		true
	}
}

impl PermissionBmc {
	pub async fn create_permission(
		ctx: &Ctx,
		mm: &ModelManager,
		data: PermissionForCreate,
	) -> Result<i64> {
		let PermissionForCreate { module, action, level } = data;

		let permission_fi = PermissionForInsert {
			module: module.to_string(),
			action: action.to_string(),
			level: level.to_string(),
		};

		let mm = mm.new_with_txn()?;
		mm.dbx().begin_txn().await?;

		let permission_id = base::create::<Self, _>(ctx, &mm, permission_fi)
			.await
			.map_err(|model_err| {
				Error::resolve_unique_violation(
					model_err,
					Some(|table: &str, constraint: &str| {
						if table == "tbl_permission"
							&& constraint.contains("module")
						{
							Some(Error::PermissionAlreadeyExists {
								module,
								action,
							})
						} else {
							None
						}
					}),
				)
			})?;

		Ok(permission_id)
	}
}

impl RoleBmc {
	pub async fn create_role(
		ctx: &Ctx,
		mm: &ModelManager,
		data: RoleForCreate,
	) -> Result<i64> {
		let RoleForCreate { name, description } = data;

		let mm = mm.new_with_txn()?;
		mm.dbx().begin_txn().await?;

		let role_fi = RoleForInsert {
			name: name.to_string(),
			description: description.to_string(),
		};
		let mut fields = SeaFields::new(vec![
			SeaField::new(RoleIden::Name, role_fi.name.clone()),
			SeaField::new(RoleIden::Description, description),
		]);

		let role_id = base::create::<Self, _>(ctx, &mm, role_fi)
			.await
			.map_err(|model_err| {
				Error::resolve_unique_violation(
					model_err,
					Some(|table: &str, constraint: &str| {
						if table == "tbl_role" && constraint.contains("module")
						{
							Some(Error::RoleAlreadyExists { name })
						} else {
							None
						}
					}),
				)
			})?;

		prep_fields_for_create::<Self>(&mut fields, role_id);

		let (columns, sea_values) = fields.for_sea_insert();
		let mut query = Query::insert();
		query
			.into_table(Self::table_ref())
			.columns(columns)
			.values(sea_values)?
			.returning(Query::returning().columns([CommonIden::Id]));

		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let sqlx_query = sqlx::query_as_with::<_, (i64,), _>(&sql, values);
		let (id,) = mm.dbx().fetch_one(sqlx_query).await?;

		Ok(id)
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filter: Option<Vec<RoleFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<Role>> {
		base::list::<Self, _, _>(ctx, mm, filter, list_options).await
	}

	pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
	where
		E: RoleBy,
	{
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn remove_role(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
	) -> Result<()> {
		let role: Role = Self::get(ctx, mm, id).await?;
		let mut fields =
			SeaFields::new(vec![SeaField::new(RoleIden::Status, 1)]);
		prep_fields_for_update::<Self>(&mut fields, role.id);

		let fields = fields.for_sea_update();
		let mut query = Query::update();
		query
			.table(Self::table_ref())
			.values(fields)
			.and_where(Expr::col(RoleIden::Id).eq(id));

		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let sqlx_query = sqlx::query_with(&sql, values);
		let _ = mm.dbx().execute(sqlx_query).await?;

		Ok(())
	}
}
