use serde::{Deserialize, Serialize};

pub mod filters;
pub mod identities;

#[derive(
	Clone, Debug, sqlx::Type, derive_more::Display, Deserialize, Serialize,
)]
#[sqlx(type_name = "applicant_status")]
pub enum ApplicantStatus {
	Applied,
	InitialReview,
	ShortListed,
	PhoneInterview,
	ExamAssessment,
	TechnicalInterview,
	PanelInterview,
	HrInterview,
	ReferenceCheck,
	BackgroundCheck,
	SalaryDiscussion,
	PendingDecision,
	ForJobOffer,
	OfferAccepted,
	PreEmployment,
	ReadyForOnboarding,
	Hired,
	Rejected,
	Withdrawn,
	OfferDeclined,
	FailedRequirements,
	OnHold,
	Blacklisted,
}

impl From<ApplicantStatus> for sea_query::Value {
	fn from(value: ApplicantStatus) -> Self {
		value.to_string().into()
	}
}

#[derive(
	Clone, Debug, sqlx::Type, derive_more::Display, Deserialize, Serialize,
)]
#[sqlx(type_name = "employment_status")]
pub enum EmployementStatus {
	FullTime,
	PartTime,
	Contract,
	Temporary,
	Seasonal,
	Intern,
	Apprentice,
	Probationary,
	Casual,
	ProjectBased,
}

impl From<EmployementStatus> for sea_query::Value {
	fn from(value: EmployementStatus) -> Self {
		value.to_string().into()
	}
}

#[derive(
	Clone, Debug, sqlx::Type, derive_more::Display, Deserialize, Serialize,
)]
#[sqlx(type_name = "employee_lifecycle_status")]
pub enum EmployeeLifecycleStatus {
	Active,
	Probationary,
	OnLeave,
	Suspended,
	Terminated,
	Resigned,
	Retired,
	EndOfContract,
	AbsentWithoutLeave,
	Inactive,
}

impl From<EmployeeLifecycleStatus> for sea_query::Value {
	fn from(value: EmployeeLifecycleStatus) -> Self {
		value.to_string().into()
	}
}

#[derive(
	Clone, Debug, sqlx::Type, derive_more::Display, Deserialize, Serialize,
)]
#[sqlx(type_name = "attendance_status")]
pub enum AttendanceStatus {
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

impl From<AttendanceStatus> for sea_query::Value {
	fn from(value: AttendanceStatus) -> Self {
		value.to_string().into()
	}
}

#[derive(
	Clone, Debug, sqlx::Type, derive_more::Display, Deserialize, Serialize,
)]
#[sqlx(type_name = "leave_request_status")]
pub enum LeaveRequestStatus {
	Pending,
	Approved,
	Denied,
	Cancelled,
	ForRevision,
	Forwared,
	Escalated,
}

impl From<LeaveRequestStatus> for sea_query::Value {
	fn from(value: LeaveRequestStatus) -> Self {
		value.to_string().into()
	}
}

#[derive(
	Clone, Debug, sqlx::Type, derive_more::Display, Deserialize, Serialize,
)]
#[sqlx(type_name = "session_type")]
pub enum SessionType {
	Active,
	Inactive,
}

impl From<SessionType> for sea_query::Value {
	fn from(value: SessionType) -> Self {
		value.to_string().into()
	}
}

#[derive(
	Clone, Debug, sqlx::Type, derive_more::Display, Deserialize, Serialize,
)]
#[sqlx(type_name = "user_session_state")]
pub enum UserSessionState {
	Active,
	Idle,
	Away,
	Offline,
	Busy,
	Locked,
}

impl From<UserSessionState> for sea_query::Value {
	fn from(value: UserSessionState) -> Self {
		value.to_string().into()
	}
}

#[derive(
	Clone, Debug, sqlx::Type, derive_more::Display, Deserialize, Serialize,
)]
#[sqlx(type_name = "performance_review_status")]
pub enum PerformanceReviewStatus {
	Scheduled,
	InProgress,
	Submitted,
	UnderReview,
	Completed,
	RequiresRevision,
	Rejected,
}

impl From<PerformanceReviewStatus> for sea_query::Value {
	fn from(value: PerformanceReviewStatus) -> Self {
		value.to_string().into()
	}
}

#[derive(
	Clone, Debug, sqlx::Type, derive_more::Display, Deserialize, Serialize,
)]
#[sqlx(type_name = "payroll_status")]
pub enum PayrollStatus {
	Draft,
	Processing,
	Processed,
	Released,
	Paid,
	Cancelled,
	Reconciled,
}

impl From<PayrollStatus> for sea_query::Value {
	fn from(value: PayrollStatus) -> Self {
		value.to_string().into()
	}
}

#[derive(
	Clone, Debug, sqlx::Type, derive_more::Display, Deserialize, Serialize,
)]
#[sqlx(type_name = "disciplinary_action_status")]
pub enum DisciplinaryActionStatus {
	Filed,
	UnderInvestigation,
	HearingScheduled,
	DecisionPending,
	Resolved,
	Dismissed,
}

impl From<DisciplinaryActionStatus> for sea_query::Value {
	fn from(value: DisciplinaryActionStatus) -> Self {
		value.to_string().into()
	}
}

#[derive(
	Clone, Debug, sqlx::Type, derive_more::Display, Deserialize, Serialize,
)]
#[sqlx(type_name = "contract_status")]
pub enum ContractStatus {
	Active,
	ExpiringSoon,
	ForRenewal,
	Renewed,
	NonRenewal,
	Expired,
}

impl From<ContractStatus> for sea_query::Value {
	fn from(value: ContractStatus) -> Self {
		value.to_string().into()
	}
}
