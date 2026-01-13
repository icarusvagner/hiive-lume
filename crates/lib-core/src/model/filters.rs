use modql::filter::{FilterNodes, OpValsInt64, OpValsString, OpValsValue};
use serde::Deserialize;

use super::modql_utils::time_to_sea_value;

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct RoleFilter {
	pub id: Option<OpValsInt64>,
	pub name: Option<OpValsString>,
	pub description: Option<OpValsString>,

	pub cid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub ctime: Option<OpValsValue>,
	pub mid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub mtime: Option<OpValsValue>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct DepartmentFilter {
	pub id: Option<OpValsInt64>,
	pub name: Option<OpValsString>,
	pub full_address: Option<OpValsString>,
	pub description: Option<OpValsString>,

	pub cid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub ctime: Option<OpValsValue>,
	pub mid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub mtime: Option<OpValsValue>,
}
