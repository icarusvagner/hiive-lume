use std::io::Write;

use diesel::{
	backend, deserialize::{self, FromSql}, expression::AsExpression, pg::Pg, serialize::{self, ToSql}
};
use lib_schema::schema::sql_types::ApplicantStatus;

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
#[diesel(sql_type = applicant_status)]
pub enum ApplicantStatusType {
	Applied,
	InitialReview,
	Shortlisted,
	PhoneInterview,
	ExamAssessment,
	TechnicalInterview,
	FinalInterview,
	BackgroundCheck,
	PendingDecision,
	ForJobOffer,
	OfferReleased,
	OfferAccepted,
	PreEmployment,
	ReadyForOnboarding,
	Hired,
	Rejected,
	Withdrawn,
	OfferDeclined,
}

impl ToString for ApplicantStatusType {
	fn to_string(&self) -> String {
		match self {
			ApplicantStatusType::Applied => "applied",
			ApplicantStatusType::InitialReview => "initial_review",
			ApplicantStatusType::Shortlisted => "shortlisted",
			ApplicantStatusType::PhoneInterview => "phone_interview",
			ApplicantStatusType::ExamAssessment => "exam_assessment",
			ApplicantStatusType::TechnicalInterview => "technical_interview",
			ApplicantStatusType::FinalInterview => "final_interview",
			ApplicantStatusType::BackgroundCheck => "background_check",
			ApplicantStatusType::PendingDecision => "pending_decision",
			ApplicantStatusType::ForJobOffer => "for_job_offer",
			ApplicantStatusType::OfferReleased => "offer_released",
			ApplicantStatusType::OfferAccepted => "offer_accepted",
			ApplicantStatusType::PreEmployment => "pre_employment",
			ApplicantStatusType::ReadyForOnboarding => "ready_for_onboarding",
			ApplicantStatusType::Hired => "hired",
			ApplicantStatusType::Rejected => "rejected",
			ApplicantStatusType::Withdrawn => "withdrawn",
			ApplicantStatusType::OfferDeclined => "offer_declined",
		}
		.to_string()
	}
}

#[derive(Debug, AsExpression, PartialEq, Eq, Clone)]
#[diesel(sql_type = ApplicantStatus)]
pub struct ApplicantStatusMapping;

impl ToSql<ApplicantStatusMapping, Pg> for ApplicantStatusType {
	fn to_sql<'b>(
		&'b self,
		out: &mut serialize::Output<'b, '_, diesel::pg::Pg>,
	) -> serialize::Result {
		out.write_all(self.to_string().as_bytes())?;

		Ok(serialize::IsNull::No)
	}
}

impl FromSql<ApplicantStatusMapping, Pg> for ApplicantStatusType {
	fn from_sql(
		bytes: <Pg as backend::Backend>::RawValue<'_>,
	) -> deserialize::Result<Self> {
		match bytes.as_bytes() {
			b"applied" => Ok(Self::Applied),
			b"initial_review" => Ok(Self::InitialReview),
			b"shortlisted" => Ok(Self::Shortlisted),
			b"phone_interview" => Ok(Self::PhoneInterview),
			b"exam_assessment" => Ok(Self::ExamAssessment),
			b"technical_interview" => Ok(Self::TechnicalInterview),
			b"final_interview" => Ok(Self::FinalInterview),
			b"background_check" => Ok(Self::BackgroundCheck),
			b"pending_decision" => Ok(Self::PendingDecision),
			b"for_job_offer" => Ok(Self::ForJobOffer),
			b"offer_released" => Ok(Self::OfferReleased),
			b"offer_accepted" => Ok(Self::OfferAccepted),
			b"pre_employment" => Ok(Self::PreEmployment),
			b"ready_for_onboarding" => Ok(Self::ReadyForOnboarding),
			b"hired" => Ok(Self::Hired),
			b"rejected" => Ok(Self::Rejected),
			b"withdrawn" => Ok(Self::Withdrawn),
			b"offer_declined" => Ok(Self::OfferDeclined),
			_ => Err("Unknown applicant status".into()),
		}
	}
}
