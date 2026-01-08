use gpui::*;
use gpui_component::{
	ActiveTheme, Sizable, scroll::ScrollableElement, spinner::Spinner, v_flex
};

use crate::{
	states::home_layout::{HomeActiveLayout, HomeLayout}, workspace::home::{
		candidates::view::Candidates, dashboard::Dashboard, employees::{create::CreateEmployee, view::Employees}, jobs::view::Jobs, leaves::Leaves, payroll::Payroll, settings::Settings
	}
};

pub struct HomeContent {
	active: HomeActiveLayout,
	dashboard: Entity<Dashboard>,
	employees: Entity<Employees>,
	jobs: Entity<Jobs>,
	candidates: Entity<Candidates>,
	leaves: Entity<Leaves>,
	payroll: Entity<Payroll>,
	settings: Entity<Settings>,
	create_employee: Entity<CreateEmployee>,
	_subscription: Vec<Subscription>,
}

impl HomeContent {
	pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
		let dashboard = Dashboard::view(window, cx);
		let employees = Employees::view(window, cx);
		let jobs = Jobs::view(window, cx);
		let candidates = Candidates::view(window, cx);
		let leaves = Leaves::view(window, cx);
		let settings = Settings::view(window, cx);
		let create_employee = CreateEmployee::view(window, cx);
		let payroll = Payroll::view(window, cx);

		let _subscription =
			vec![cx.observe_global::<HomeLayout>(move |this, cx| {
				this.active = cx.global::<HomeLayout>().home.clone();
				cx.notify();
			})];

		Self {
			active: HomeActiveLayout::CreateEmployee,
			dashboard,
			employees,
			jobs,
			candidates,
			payroll,
			leaves,
			settings,
			create_employee,
			_subscription,
		}
	}

	pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
		cx.new(|cx| Self::new(window, cx))
	}

	fn render_dashboard(&mut self, cx: &mut Context<Self>) -> Stateful<Div> {
		let content = v_flex()
			.id("dashboard")
			.size_full()
			.bg(cx.theme().background)
			.child(self.dashboard.clone());

		content
	}

	fn render_employees(&mut self, cx: &mut Context<Self>) -> Stateful<Div> {
		let content = v_flex()
			.id("employees")
			.size_full()
			.bg(cx.theme().background)
			.child(self.employees.clone());

		content
	}

	fn render_jobs(&mut self, cx: &mut Context<Self>) -> Stateful<Div> {
		let content = v_flex()
			.id("jobs")
			.size_full()
			.bg(cx.theme().background)
			.child(self.jobs.clone());

		content
	}

	fn render_candidates(&mut self, cx: &mut Context<Self>) -> Stateful<Div> {
		let content = v_flex()
			.id("candidates")
			.size_full()
			.bg(cx.theme().background)
			.child(self.candidates.clone());

		content
	}

	fn render_leaves(&mut self, cx: &mut Context<Self>) -> Stateful<Div> {
		let content = v_flex()
			.id("leaves")
			.size_full()
			.bg(cx.theme().background)
			.child(self.leaves.clone());

		content
	}

	fn render_payroll(&mut self, cx: &mut Context<Self>) -> Stateful<Div> {
		let content = v_flex()
			.id("payroll")
			.size_full()
			.bg(cx.theme().background)
			.child(self.payroll.clone());

		content
	}

	fn render_settings(&mut self, cx: &mut Context<Self>) -> Stateful<Div> {
		let content = v_flex()
			.id("settings")
			.size_full()
			.bg(cx.theme().background)
			.child(self.settings.clone());

		content
	}

	fn render_create_employee(
		&mut self,
		cx: &mut Context<Self>,
	) -> Stateful<Div> {
		let content = v_flex()
			.id("create-employee")
			.size_full()
			.bg(cx.theme().background)
			.child(self.create_employee.clone());

		content
	}

	fn render_loading_home(&mut self, cx: &mut Context<Self>) -> Stateful<Div> {
		let content = div()
			.id("home-loading-content")
			.flex()
			.flex_grow()
			.bg(cx.theme().background)
			.justify_center()
			.items_center()
			.child(
				v_flex()
					.mt_80()
					.items_center()
					.justify_center()
					.gap_10()
					.child(
						Spinner::new()
							.color(cx.theme().blue)
							.with_size(px(100.)),
					),
			);

		content
	}
}

impl Render for HomeContent {
	fn render(
		&mut self,
		_window: &mut Window,
		cx: &mut Context<Self>,
	) -> impl IntoElement {
		let content = match self.active {
			HomeActiveLayout::Dashboard => self.render_dashboard(cx),
			HomeActiveLayout::Employees => self.render_employees(cx),
			HomeActiveLayout::Jobs => self.render_jobs(cx),
			HomeActiveLayout::Candidates => self.render_candidates(cx),
			HomeActiveLayout::Leaves => self.render_leaves(cx),
			HomeActiveLayout::Settings => self.render_settings(cx),
			HomeActiveLayout::Payroll => self.render_payroll(cx),
			HomeActiveLayout::CreateEmployee => self.render_create_employee(cx),
			HomeActiveLayout::Loading => self.render_loading_home(cx),
		};

		v_flex()
			.size_full()
			.flex_1()
			.flex_grow()
			.pb_24()
			.bg(cx.theme().background)
			.child(div().relative().child(content))
			.overflow_y_scrollbar()
	}
}
