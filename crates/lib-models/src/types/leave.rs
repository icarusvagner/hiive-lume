use std::io::Write;

use diesel::{
	expression::AsExpression, pg, serialize::{self, ToSql}
};
use lib_schema::schema::sql_types::LeaveRequestStatus;

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
#[diesel(sql_type = leave_request_status)]
pub enum LeaveRequestStatusType {
	Pending,
	Approved,
	Denied,
	Cancelled,
	ForRevision,
	Forwarded,
	Escalated,
}

impl ToString for LeaveRequestStatusType {
	fn to_string(&self) -> String {
		match self {
			Self::Pending => "pending",
			Self::Approved => "approved",
			Self::Denied => "denied",
			Self::Cancelled => "cancelled",
			Self::ForRevision => "for_revision",
			Self::Forwarded => "forwarded",
			Self::Escalated => "escalated",
		}
		.to_string()
	}
}

#[derive(Debug, AsExpression, PartialEq, Eq, Clone)]
#[diesel(sql_type = LeaveRequestStatus)]
pub struct LeaveRequestStatusMapping;

impl ToSql<LeaveRequestStatusMapping, pg::Pg> for LeaveRequestStatusType {
	fn to_sql<'b>(
		&'b self,
		out: &mut diesel::serialize::Output<'b, '_, pg::Pg>,
	) -> diesel::serialize::Result {
		out.write_all(self.to_string().as_bytes())?;

		Ok(serialize::IsNull::No)
	}
}
