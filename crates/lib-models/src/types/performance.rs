use std::io::Write;

use diesel::{
	backend, deserialize::{self, FromSql}, expression::AsExpression, pg, serialize::{self, ToSql}
};
use lib_schema::schema::sql_types::PerformanceReviewStatus;

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
#[diesel(sql_type = performance_review_status)]
pub enum PerformanceReviewStatusType {
	Scheduled,
	InProgress,
	Submitted,
	UnderReview,
	Completed,
	RequiresRevision,
	Rejected,
}

impl ToString for PerformanceReviewStatusType {
	fn to_string(&self) -> String {
		match self {
			Self::Scheduled => "scheduled",
			Self::InProgress => "in_progress",
			Self::Submitted => "submitted",
			Self::UnderReview => "under_review",
			Self::Completed => "completed",
			Self::RequiresRevision => "requires_revision",
			Self::Rejected => "rejected",
		}
		.to_string()
	}
}

#[derive(Debug, AsExpression, PartialEq, Eq, Clone)]
#[diesel(sql_type = PerformanceReviewStatus)]
pub struct PerformanceReviewStatusMapping;

impl ToSql<PerformanceReviewStatusMapping, pg::Pg>
	for PerformanceReviewStatusType
{
	fn to_sql<'b>(
		&'b self,
		out: &mut serialize::Output<'b, '_, pg::Pg>,
	) -> serialize::Result {
		out.write_all(self.to_string().as_bytes())?;

		Ok(serialize::IsNull::No)
	}
}

impl FromSql<PerformanceReviewStatusMapping, pg::Pg>
	for PerformanceReviewStatusType
{
	fn from_sql(
		bytes: <pg::Pg as backend::Backend>::RawValue<'_>,
	) -> deserialize::Result<Self> {
		match bytes.as_bytes() {
			b"scheduled" => Ok(Self::Scheduled),
			b"in_progress" => Ok(Self::InProgress),
			b"submitted" => Ok(Self::Submitted),
			b"under_review" => Ok(Self::UnderReview),
			b"completed" => Ok(Self::Completed),
			b"requires_revision" => Ok(Self::RequiresRevision),
			b"rejected" => Ok(Self::Rejected),
			_ => Err("Unknown performance review status".into()),
		}
	}
}
