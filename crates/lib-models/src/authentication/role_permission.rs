use modql::field::Fields;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct RolePermission {
	pub id: i64,
	pub module: String,
	pub action: String,
	pub level: String,
}

#[derive(Deserialize)]
pub struct RolePermissionForCreate {
	pub module: String,
	pub action: String,
	pub level: String,
}

#[derive(Fields, Deserialize)]
pub struct RolePermissionForUpdate {
	pub action: String,
	pub level: String,
}
