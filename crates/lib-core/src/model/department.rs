use lib_models::{
	department::{Department, DepartmentForCreate, DepartmentForInsert}, types::identities::DepartmentIden
};
use modql::{
	field::{HasSeaFields, SeaField, SeaFields}, filter::ListOptions
};
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::{FromRow, postgres::PgRow};

use crate::{
	ctx::Ctx, model::{
		DbBmc, Error, ModelManager, Result, base, filters::DepartmentFilter, prep_fields_for_update
	}
};

#[rustfmt::skip]
pub trait DepartmentBy: HasSeaFields + for<'r>FromRow<'r, PgRow> + Unpin + Send {}

impl DepartmentBy for Department {}

pub struct DepartmentBmc;

impl DbBmc for DepartmentBmc {
	const TABLE: &'static str = "tbl_department";

	fn has_timestamps() -> bool {
		true
	}

	fn has_owner_id() -> bool {
		true
	}
}

impl DepartmentBmc {
	pub async fn create_department(
		ctx: &Ctx,
		mm: &ModelManager,
		data: DepartmentForCreate,
	) -> Result<i64> {
		let DepartmentForCreate { name, full_address, description } = data;

		let department_fi = DepartmentForInsert {
			name: name.to_string(),
			full_address: full_address.to_string(),
			description: description.to_string(),
		};

		let mm = mm.new_with_txn()?;
		mm.dbx().begin_txn().await?;

		let deparment_id = base::create::<Self, _>(ctx, &mm, department_fi)
			.await
			.map_err(|model_err| {
				Error::resolve_unique_violation(
					model_err,
					Some(|table: &str, constraint: &str| {
						if table == "tbl_department"
							&& constraint.contains("name")
						{
							Some(Error::EntityAlreadyExists {
								table_name: "tbl_department".to_string(),
								attribute_name: "name".to_string(),
								tuple: name.clone(),
							})
						} else {
							None
						}
					}),
				)
			})?;

		Ok(deparment_id)
	}

	pub async fn list_department(
		ctx: &Ctx,
		mm: &ModelManager,
		filter: Option<Vec<DepartmentFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<Department>> {
		base::list::<Self, _, _>(ctx, mm, filter, list_options).await
	}

	pub async fn get_deparment<E>(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
	) -> Result<E>
	where
		E: DepartmentBy,
	{
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn remove_department(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
	) -> Result<()> {
		let department: Department = Self::get_deparment(ctx, mm, id).await?;
		let mut fields =
			SeaFields::new(vec![SeaField::new(DepartmentIden::Visible, 1)]);
		prep_fields_for_update::<Self>(&mut fields, department.id);

		let fields = fields.for_sea_update();
		let mut query = Query::update();
		query
			.table(Self::table_ref())
			.values(fields)
			.and_where(Expr::col(DepartmentIden::Id).eq(id));
		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let sqlx_query = sqlx::query_with(&sql, values);
		let _ = mm.dbx().execute(sqlx_query).await?;

		Ok(())
	}
}
