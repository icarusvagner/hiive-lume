use lib_core::model::{ModelManager, department::DepartmentBmc};
use lib_models::department::{Department, DepartmentForCreate};

use crate::{Result, core::context::ctx_resolve};

pub async fn handlers_add_department(
	mm: &ModelManager,
	user_id: i64,
	payload: DepartmentAddPayload,
) -> Result<i64> {
	tracing::debug!("{:<12} - {}", "HANDLERS", "handlers_add_department");
	let DepartmentAddPayload { name, full_address, description } = payload;

	let data = DepartmentForCreate {
		name: name.to_string(),
		full_address: full_address.to_string(),
		description: description.to_string(),
	};

	let ctx = ctx_resolve(user_id).await?.0;

	let department_id =
		DepartmentBmc::create_department(&ctx, &mm, data).await?;

	Ok(department_id)
}

pub async fn handlers_get_count(
	mm: &ModelManager,
	user_id: i64,
) -> Result<usize> {
	tracing::debug!("{:<12} - {}", "HANDLERS", "handlers_get_count_department");
	let ctx = ctx_resolve(user_id).await?.0;

	let department_count =
		DepartmentBmc::list_department(&ctx, mm, None, None).await?.len();
	println!("{department_count}");

	Ok(department_count)
}

pub async fn handlers_department_list(
	mm: &ModelManager,
	user_id: i64,
) -> Result<Vec<Department>> {
	tracing::debug!("{:<12} - {}", "HANDLERS", "handlers_department_list");
	let ctx = ctx_resolve(user_id).await?.0;

	let departments =
		DepartmentBmc::list_department(&ctx, mm, None, None).await?;

	Ok(departments)
}

#[derive(Debug)]
pub struct DepartmentAddPayload {
	pub name: String,
	pub full_address: String,
	pub description: String,
}
