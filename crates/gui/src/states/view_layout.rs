use gpui::{App, Global};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum HomeActiveView {
	Dashboard,

	// People
	Employees,
	Applicants,
	Contracts,
	EmploymentStatus,
	Departments,
	RolesPermissions,

	// Attendance
	DailyTimeRecords, // DTR
	ShiftSchedules,
	OvertimeUnderTime,
	LeavesManagement,
	Holidays,

	// Payroll
	PayrollRuns,
	RatesDeductions,
	Allowances,
	PayslipHistory,

	// Performance
	Evaluations,
	KPIs,
	GoalTracking,
	FeedbackSessions,

	// Documents,
	EmployeeFiles,
	Templates,
	CertificatesForms,
	HRPolicies,

	// Operations,
	RequestsTickets,
	AssetManagement,
	InventoryTracking,
	AuditLogs,

	// Online Portal
	EmployeeSelfService, // ESS
	RecruitmentPortal,
	Announcements,

	// Settings,
	UserAccounts,
	Integrations,
	BackupsSync,
}

#[derive(Debug, PartialEq)]
pub struct HomeView {
	pub home: HomeActiveView,
}

impl Global for HomeView {}

impl HomeView {
	pub fn init(cx: &mut App) {
		let this = HomeView { home: HomeActiveView::Dashboard };

		cx.set_global(this);
	}
}
