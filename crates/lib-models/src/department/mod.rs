use modql::field::Fields;
use serde::Deserialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Clone, FromRow, Fields, Debug)]
pub struct Department {
	pub id: i64,
	pub department_id: Uuid,
	pub name: String,
	pub full_address: String,
	pub description: String,
	pub visible: i8,
}

#[derive(Deserialize)]
pub struct DepartmentForCreate {
	pub name: String,
	pub full_address: String,
	pub description: String,
}

#[derive(Fields)]
pub struct DepartmentForInsert {
	pub name: String,
	pub full_address: String,
	pub description: String,
}

#[derive(Fields, Deserialize, Default)]
pub struct DepartmentForUpdate {
	pub full_address: String,
	pub description: String,
}
