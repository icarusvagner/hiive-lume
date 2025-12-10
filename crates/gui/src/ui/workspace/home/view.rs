use gpui::{prelude::FluentBuilder, *};
use gpui_component::{
	ActiveTheme, Icon, IconName, Side, Sizable, button::{Button, ButtonVariants}, h_flex, label::Label, sidebar::{Sidebar, SidebarGroup, SidebarMenu, SidebarMenuItem}, v_flex
};

use crate::{
	states::{
		main_layout::{ActiveView, ViewState}, view_layout::HomeActiveView
	}, ui::workspace::home::{dashboard::DashboardView, employee::EmployeeView}
};

pub struct Homeview {
	view: HomeActiveView,
	collapse_menu: bool,

	// Views
	dashboard: Entity<DashboardView>,
	employee: Entity<EmployeeView>,
}

impl Homeview {
	pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
		let dashboard = DashboardView::view(window, cx);
		let employee = EmployeeView::view(window, cx);

		Self {
			view: HomeActiveView::Dashboard,
			dashboard,
			employee,
			collapse_menu: false,
		}
	}

	pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
		cx.new(|cx| Self::new(window, cx))
	}

	fn render_employee(&self, _cx: &mut Context<Self>) -> Stateful<Div> {
		v_flex().size_full().id("employee-view").child(self.employee.clone())
	}

	fn render_dashboard(&self, _cx: &mut Context<Self>) -> Stateful<Div> {
		v_flex().size_full().id("dashboard-view").child(self.dashboard.clone())
	}

	fn handle_logout(&self, _window: &mut Window, cx: &mut Context<Self>) {
		let _ = cx.update_global::<ViewState, _>(|state, _| {
			state.view = ActiveView::Loading;
		});

		cx.spawn(async move |_, cx| {
			tokio::time::sleep(std::time::Duration::from_millis(1_200)).await;
			let _ = cx.update_global::<ViewState, _>(|state, _| {
				state.view = ActiveView::Login;
			});
		})
		.detach();
	}

	fn render_content(
		&self,
		_window: &mut Window,
		cx: &mut Context<Self>,
	) -> Stateful<Div> {
		match self.view {
			HomeActiveView::Dashboard => self.render_dashboard(cx),
			HomeActiveView::Employees => self.render_employee(cx),
			_ => unreachable!(),
		}
	}

	fn render_sidebar_menu(
		&self,
		_window: &mut Window,
		cx: &mut Context<Self>,
	) -> impl IntoElement {
		let _size = if self.collapse_menu {
			px(24.)
		} else {
			px(28.)
		};

		Sidebar::new(Side::Left)
			.max_w(px(360.))
			.min_w(px(50.))
			.bg(cx.theme().background.opacity(0.50))
			.collapsed(self.collapse_menu)
			.collapsible(true)
			.header(
				Button::new("sidebar-toggler")
					.ghost()
					.large()
					.w_full()
					.child(
						div()
							.flex()
							.w_full()
							.items_center()
							.gap_2()
							.when_else(
								self.collapse_menu,
								|this| this.justify_start(),
								|this| this.justify_center(),
							)
							.when_else(
								!self.collapse_menu,
								|this| {
									this.child(
										img("images/hiive-logo.png").size_8(),
									)
								},
								|this| {
									this.child(
										Icon::new(Icon::empty().path(
											"icons/custom/text-align-justify.\
											 svg",
										))
										.size_5(),
									)
								},
							)
							.when(!self.collapse_menu, |this| {
								this.child("Admin Panel")
							}),
					)
					.on_click(cx.listener(|this, _, _, _| {
						this.collapse_menu = !this.collapse_menu
					}))
					.when_else(
						!self.collapse_menu,
						|this| this.tooltip("Collapse menu"),
						|this| this.tooltip("Expand menu"),
					),
			)
			.child(
				SidebarGroup::new("Main Menu").child(
					SidebarMenu::new()
						.child(
							SidebarMenuItem::new("Dashboard")
								.icon(IconName::LayoutDashboard)
								.on_click(|_, _, _| {
									println!("Dashboard clicked")
								})
								.when(
									self.view.eq(&HomeActiveView::Dashboard),
									|this| this.active(true),
								),
						)
						.child(
							SidebarMenuItem::new("People")
								.icon(Icon::empty().path(
									"icons/custom/users-round-outline.svg",
								))
								.children([
									SidebarMenuItem::new("Employees").when(
										self.view
											.eq(&HomeActiveView::Employees),
										|this| this.active(true),
									),
									SidebarMenuItem::new("Applicants").when(
										self.view
											.eq(&HomeActiveView::Applicants),
										|this| this.active(true),
									),
									SidebarMenuItem::new("Contracts").when(self.view.eq(&HomeActiveView::Contracts), |this| this.active(true)),
									SidebarMenuItem::new("Employment Status").when(self.view.eq(&HomeActiveView::EmploymentStatus), |this| this.active(true)),
									SidebarMenuItem::new("Departments").when(self.view.eq(&HomeActiveView::Departments), |this| this.active(true)),
									SidebarMenuItem::new("Roles & Permissions").when(self.view.eq(&HomeActiveView::RolesPermissions), |this| this.active(true)),
								]),
						)
						.child(
							SidebarMenuItem::new("Attendance")
								.icon(Icon::new(IconName::Calendar))
								.children([
									SidebarMenuItem::new("Daily Time Records").when(self.view.eq(&HomeActiveView::DailyTimeRecords), |this| this.active(true)),
									SidebarMenuItem::new("Shift Schedules").when(self.view.eq(&HomeActiveView::ShiftSchedules), |this| this.active(true)),
									SidebarMenuItem::new(
										"Overtime & Undertime",
									).when(self.view.eq(&HomeActiveView::OvertimeUnderTime), |this| this.active(true)),
									SidebarMenuItem::new("Leaves Managements").when(self.view.eq(&HomeActiveView::LeavesManagement), |this| this.active(true)),
									SidebarMenuItem::new("Holidays").when(self.view.eq(&HomeActiveView::Holidays), |this| this.active(true)),
								]),
						)
						.child(
							SidebarMenuItem::new("Payroll")
								.icon(Icon::empty().path(
									"icons/custom/\
									 money-cash-currency-finance-bank-coin-business.\
									 svg",
								))
								.children([
									SidebarMenuItem::new("Payroll Runs").when(self.view.eq(&HomeActiveView::PayrollRuns), |this| this.active(true)),
									SidebarMenuItem::new("Rates & Deductions").when(self.view.eq(&HomeActiveView::RatesDeductions), |this| this.active(true)),
									SidebarMenuItem::new("Allowances").when(self.view.eq(&HomeActiveView::Allowances), |this| this.active(true)),
									SidebarMenuItem::new("Payslip History").when(self.view.eq(&HomeActiveView::PayslipHistory), |this| this.active(true)),
								]),
						)
						.child(
							SidebarMenuItem::new("Performance")
								.icon(
									Icon::empty().path(
										"icons/custom/chart-column-grow.svg",
									),
								)
								.children([
									SidebarMenuItem::new("Evaluations").when(self.view.eq(&HomeActiveView::Evaluations), |this| this.active(true)),
									SidebarMenuItem::new("KPIs").when(self.view.eq(&HomeActiveView::KPIs), |this| this.active(true)),
									SidebarMenuItem::new("Goal Tracking").when(self.view.eq(&HomeActiveView::GoalTracking), |this| this.active(true)),
									SidebarMenuItem::new("Feedback Sessions").when(self.view.eq(&HomeActiveView::FeedbackSessions), |this| this.active(true)),
								]),
						)
						.child(
							SidebarMenuItem::new("Documents")
								.icon(Icon::empty().path(
									"icons/custom/multiple-documents-files.svg",
								))
								.children([
									SidebarMenuItem::new("Employee Files").when(self.view.eq(&HomeActiveView::EmployeeFiles), |this| this.active(true)),
									SidebarMenuItem::new("Templates").when(self.view.eq(&HomeActiveView::Templates), |this| this.active(true)),
									SidebarMenuItem::new("Certifates & Forms").when(self.view.eq(&HomeActiveView::CertificatesForms), |this| this.active(true)),
									SidebarMenuItem::new("HR Policies").when(self.view.eq(&HomeActiveView::HRPolicies), |this| this.active(true)),
								]),
						)
						.child(
							SidebarMenuItem::new("Operations")
								.icon(
									Icon::empty()
										.path("icons/custom/tools.svg"),
								)
								.children([
									SidebarMenuItem::new("Requests & Tickets").when(self.view.eq(&HomeActiveView::RequestsTickets), |this| this.active(true)),
									SidebarMenuItem::new("Asset Management").when(self.view.eq(&HomeActiveView::AssetManagement), |this| this.active(true)),
									SidebarMenuItem::new("Inventory Tracking").when(self.view.eq(&HomeActiveView::InventoryTracking), |this| this.active(true)),
									SidebarMenuItem::new("Audit Logs").when(self.view.eq(&HomeActiveView::AuditLogs), |this| this.active(true)),
								]),
						)
						.child(
							SidebarMenuItem::new("Online Portal")
								.icon(
									Icon::empty()
										.path("icons/custom/id-card.svg"),
								)
								.children([
									SidebarMenuItem::new(
										"Employee Self-Service",
									).when(self.view.eq(&HomeActiveView::EmployeeSelfService), |this| this.active(true)),
									SidebarMenuItem::new("Recruitment Portal").when(self.view.eq(&HomeActiveView::RecruitmentPortal), |this| this.active(true)),
									SidebarMenuItem::new("Announcements").when(self.view.eq(&HomeActiveView::Announcements), |this| this.active(true)),
								]),
						),
				),
			)
			.child(
				SidebarGroup::new("Settings").child(
					SidebarMenu::new()
						.child(SidebarMenuItem::new("User Accounts").icon(
							Icon::empty().path("icons/custom/users-more.svg"),
						).when(self.view.eq(&HomeActiveView::UserAccounts), |this| this.active(true)))
						.child(SidebarMenuItem::new("Integrations").icon(
							Icon::empty().path("icons/custom/data-mapping.svg"),
						).when(self.view.eq(&HomeActiveView::Integrations), |this| this.active(true)))
						.child(SidebarMenuItem::new("Backups & Sync").icon(
							Icon::empty().path("icons/custom/cloud-up.svg"),
						).when(self.view.eq(&HomeActiveView::BackupsSync), |this| this.active(true))),
				),
			)
			.footer(
				Button::new("footer-sidebar-btn")
					.w_full()
					.child(
						h_flex()
							.items_center()
							.justify_center()
							.gap_1()
							.child(
								Icon::empty()
									.path("icons/custom/log-out.svg")
									.size_4(),
							)
							.when(!self.collapse_menu, |this| {
								this.child(Label::new("Log Out").text_lg())
							}),
					)
					.large()
					.primary()
					.on_click(cx.listener(|this, _, window, cx| {
						this.handle_logout(window, cx)
					})),
			)
	}
}

impl Render for Homeview {
	fn render(
		&mut self,
		window: &mut Window,
		cx: &mut Context<Self>,
	) -> impl IntoElement {
		let content = h_flex()
			.h_full()
			.w_full()
			.child(self.render_sidebar_menu(window, cx))
			.child(self.render_content(window, cx));

		v_flex().h_full().w_full().child(content)
	}
}
