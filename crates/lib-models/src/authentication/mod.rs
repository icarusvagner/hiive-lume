use modql::filter::{FilterNodes, OpValsInt64, OpValsString, OpValsValue};
use sea_query::Iden;
use serde::Deserialize;

mod permission;
mod role;
mod role_permission;
mod user_account;
mod user_role;

#[derive(Iden)]
pub enum UserIden {
	Id,
	Username,
	Password,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct UserFilter {
	pub id: Option<OpValsInt64>,
	pub user_id: Option<OpValsString>,
	pub username: Option<OpValsString>,

	pub cid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub ctime: Option<OpValsValue>,
}
