use lib_core::model::{ModelManager, department::DepartmentBmc};
use lib_models::department::DepartmentForCreate;

use crate::{
	Result, core::{
		context::ctx_resolve, types::request_res::{RequestError, RequestResult}
	}
};

pub async fn handlers_add_department(
	mm: &ModelManager,
	user_id: i64,
	payload: DepartmentAddPayload,
) -> Result<RequestResult> {
	let DepartmentAddPayload { name, full_address, description } = payload;

	let data = DepartmentForCreate {
		name: name.to_string(),
		full_address: full_address.to_string(),
		description: description.to_string(),
	};

	let ctx = ctx_resolve(user_id).await?.0;

	let _ = DepartmentBmc::create_department(&ctx, &mm, data).await?;

	Ok(RequestResult {
		message: "Department Successfully Added".to_string(),
		status: RequestError::Success,
	})
}

#[derive(Debug)]
pub struct DepartmentAddPayload {
	pub name: String,
	pub full_address: String,
	pub description: String,
}
