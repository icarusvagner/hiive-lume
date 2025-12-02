use lib_models::authentication::{role::Role, role_permission::Permission};
use modql::{
	field::HasSeaFields, filter::{FilterNodes, OpValsInt64, OpValsString, OpValsValue}
};
use sea_query::Iden;
use serde::Deserialize;
use sqlx::{FromRow, postgres::PgRow};

use crate::model::{DbBmc, modql_utils::time_to_sea_value};

#[derive(Iden)]
pub enum RoleIden {
	Id,
	Name,
	Description,
}

#[derive(Iden)]
pub enum PermissionIden {
	Id,
	Module,
	Action,
	Level,
}

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

pub trait RoleBy:
	HasSeaFields + for<'r> FromRow<'r, PgRow> + Unpin + Send
{
}

impl RoleBy for Role {}
impl RoleBy for Permission {}

pub struct RoleBmc;
pub struct PermissionBmc;

impl DbBmc for RoleBmc {
	const TABLE: &'static str = "tbl_role";
}

impl DbBmc for PermissionBmc {
	const TABLE: &'static str = "tbl_permission";
}
