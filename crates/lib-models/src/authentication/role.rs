use modql::field::Fields;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Role {
	pub id: i64,
	pub name: String,
	pub description: String,
	pub status: i32,
}

#[derive(Deserialize)]
pub struct RoleForCreate {
	pub name: String,
	pub description: String,
}

#[derive(Fields)]
pub struct RoleForInsert {
	pub name: String,
	pub description: String,
}
