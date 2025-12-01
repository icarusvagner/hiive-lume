use std::io::Write;

use diesel::{
	IntoSql, backend, deserialize::{self, FromSql}, expression::AsExpression, pg, serialize::{self, ToSql}
};
use lib_schema::schema::sql_types::UserSessionState;

#[derive(
	Debug,
	Clone,
	Copy,
	PartialEq,
	Eq,
	diesel::sql_types::SqlType,
	diesel::deserialize::FromSqlRow,
	serde::Serialize,
	serde::Deserialize,
)]
#[diesel(sql_type = user_session_state)]
pub enum UserSessionStateType {
	Active,
	Idle,
	Away,
	Offline,
	Busy,
	Locked,
}

impl ToString for UserSessionStateType {
	fn to_string(&self) -> String {
		match self {
			Self::Active => "active",
			Self::Idle => "idle",
			Self::Away => "away",
			Self::Offline => "offline",
			Self::Busy => "busy",
			Self::Locked => "locked",
		}
		.to_string()
	}
}

#[derive(Debug, AsExpression, PartialEq, Eq, Clone)]
#[diesel(sql_type = UserSessionState)]
pub struct UserSessionStateTypeMapping;

impl ToSql<UserSessionStateTypeMapping, pg::Pg> for UserSessionStateType {
	fn to_sql<'b>(
		&'b self,
		out: &mut serialize::Output<'b, '_, pg::Pg>,
	) -> serialize::Result {
		out.write_all(self.to_string().as_bytes())?;

		Ok(serialize::IsNull::No)
	}
}

impl FromSql<UserSessionStateTypeMapping, pg::Pg> for UserSessionStateType {
	fn from_sql(
		bytes: <pg::Pg as backend::Backend>::RawValue<'_>,
	) -> deserialize::Result<Self> {
		match bytes.as_bytes() {
			b"active" => Ok(Self::Active),
			b"idle" => Ok(Self::Idle),
			b"away" => Ok(Self::Away),
			b"offline" => Ok(Self::Offline),
			b"busy" => Ok(Self::Busy),
			b"locked" => Ok(Self::Locked),
			_ => Err("Unknown user session state".into()),
		}
	}
}
