use std::io::Write;

use diesel::{
	deserialize::FromSql, expression::AsExpression, pg, serialize::{self, ToSql}
};
use lib_schema::schema::sql_types::AttendanceStatus;

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
#[diesel(sql_type = attendance_status)]
pub enum AttendanceStatusType {
	Present,
	Late,
	Absent,
	OnLeave,
	HalfDay,
	Overtime,
	RestDay,
	Holiday,
	WorkFromHome,
	FieldWork,
}

impl ToString for AttendanceStatusType {
	fn to_string(&self) -> String {
		match self {
			Self::Present => "present",
			Self::Late => "late",
			Self::Absent => "absent",
			Self::OnLeave => "on_leave",
			Self::HalfDay => "half_day",
			Self::Overtime => "overtime",
			Self::RestDay => "reset_day",
			Self::Holiday => "holiday",
			Self::WorkFromHome => "work_from_home",
			Self::FieldWork => "field_work",
		}
		.to_string()
	}
}

#[derive(Debug, AsExpression, PartialEq, Eq, Clone)]
#[diesel(sql_type = AttendanceStatus)]
pub struct AttendanceStatusMapping;

impl ToSql<AttendanceStatusMapping, pg::Pg> for AttendanceStatusType {
	fn to_sql<'b>(
		&'b self,
		out: &mut serialize::Output<'b, '_, pg::Pg>,
	) -> serialize::Result {
		out.write_all(self.to_string().as_bytes())?;

		Ok(serialize::IsNull::No)
	}
}

impl FromSql<AttendanceStatusMapping, pg::Pg> for AttendanceStatusType {
	fn from_sql(
		bytes: <pg::Pg as diesel::backend::Backend>::RawValue<'_>,
	) -> diesel::deserialize::Result<Self> {
		match bytes.as_bytes() {
			b"present" => Ok(Self::Present),
			b"late" => Ok(Self::Late),
			b"absent" => Ok(Self::Absent),
			b"on_leave" => Ok(Self::OnLeave),
			b"half_day" => Ok(Self::HalfDay),
			b"overtime" => Ok(Self::Overtime),
			b"rest_day" => Ok(Self::RestDay),
			b"holiday" => Ok(Self::Holiday),
			b"work_from_home" => Ok(Self::WorkFromHome),
			b"field_work" => Ok(Self::FieldWork),
			_ => Err("Unknown attendance status".into()),
		}
	}
}
