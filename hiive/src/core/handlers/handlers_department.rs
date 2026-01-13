use lib_core::model::{ModelManager, department::DepartmentBmc};
use lib_models::department::DepartmentForCreate;

use crate::{Result, core::context::ctx_resolve};

pub async fn handlers_add_department(
	mm: &ModelManager,
	user_id: i64,
	payload: DepartmentAddPayload,
) -> Result<i64> {
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

#[derive(Debug)]
pub struct DepartmentAddPayload {
	pub name: String,
	pub full_address: String,
	pub description: String,
}
