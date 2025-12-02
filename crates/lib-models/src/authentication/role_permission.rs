use modql::field::Fields;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Permission {
	pub id: i64,
	pub module: String,
	pub action: String,
	pub level: String,
}

#[derive(Deserialize)]
pub struct PermissionForCreate {
	pub module: String,
	pub action: String,
	pub level: String,
}

#[derive(Fields, Deserialize)]
pub struct PermissionForUpdate {
	pub action: String,
	pub level: String,
}
