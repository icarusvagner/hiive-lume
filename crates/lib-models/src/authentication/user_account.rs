use modql::field::Fields;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::types::UserSessionState;

#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct UserAccount {
	pub id: i64,
	pub user_id: String,
	pub username: String,
	pub status: UserSessionState,
}

#[derive(Deserialize)]
pub struct UserAccountForCreate {
	pub user_id: String,
	pub username: String,
	pub password_hash: String,
}

#[derive(Fields)]
pub struct UserAccountForInsert {
	pub user_id: String,
	pub username: String,
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct UserAccountForLogin {
	pub id: i64,
	pub user_id: String,
	pub username: String,

	pub password_hash: Option<String>,
	pub pass_salt: Uuid,
	pub token_salt: Uuid,
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct UserAccountForAuth {
	pub id: i64,
	pub user_id: String,
	pub username: String,

	pub token_salt: Uuid,
}
