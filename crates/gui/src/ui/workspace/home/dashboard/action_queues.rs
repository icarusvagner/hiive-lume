use gpui::*;
use gpui_component::{ActiveTheme, Icon, h_flex};

pub struct DashboardActionQueues;

impl DashboardActionQueues {
	pub fn view(_window: &mut Window, cx: &mut App) -> Entity<Self> {
		cx.new(|_| Self)
	}

	fn approve_leave_requests(
		&self,
		_window: &mut Window,
		cx: &mut Context<Self>,
	) -> Div {
		h_flex()
			.bg(cx.theme().accent.opacity(0.30))
			.p_5()
			.rounded_xl()
			.gap_5()
			.child(
				div()
					.flex()
					.items_center()
					.justify_center()
					.p_4()
					.rounded_full()
					.bg(cx.theme().yellow_light.opacity(0.10))
					.child(
						Icon::empty()
							.size_4()
							.path("icons/custom/calendar-plus.svg")
							.text_color(cx.theme().yellow_light),
					),
			)
	}

	fn approve_overtime_request(
		&self,
		_window: &mut Window,
		cx: &mut Context<Self>,
	) -> Div {
		h_flex()
			.bg(cx.theme().accent.opacity(0.30))
			.p_5()
			.rounded_xl()
			.gap_5()
			.child(
				div()
					.flex()
					.items_center()
					.justify_center()
					.p_4()
					.rounded_full()
					.bg(cx.theme().yellow_light.opacity(0.10))
					.child(
						Icon::empty()
							.size_4()
							.path("icons/custom/clock-fading.svg")
							.text_color(cx.theme().yellow_light),
					),
			)
	}

	fn pending_contract_renewals(
		&self,
		_window: &mut Window,
		cx: &mut Context<Self>,
	) -> Div {
		h_flex()
			.bg(cx.theme().accent.opacity(0.30))
			.p_5()
			.rounded_xl()
			.gap_5()
			.child(
				div()
					.flex()
					.items_center()
					.justify_center()
					.p_4()
					.rounded_full()
					.bg(cx.theme().yellow_light.opacity(0.10))
					.child(
						Icon::empty()
							.size_4()
							.path("icons/custom/paper-recycle.svg")
							.text_color(cx.theme().yellow_light),
					),
			)
	}

	fn probationary_evaluation_due(
		&self,
		_window: &mut Window,
		cx: &mut Context<Self>,
	) -> Div {
		h_flex()
			.bg(cx.theme().accent.opacity(0.30))
			.p_5()
			.rounded_xl()
			.gap_5()
			.child(
				div()
					.flex()
					.items_center()
					.justify_center()
					.p_4()
					.rounded_full()
					.bg(cx.theme().yellow_light.opacity(0.10))
					.child(
						Icon::empty()
							.size_4()
							.path("icons/custom/time-progress.svg")
							.text_color(cx.theme().yellow_light),
					),
			)
	}

	fn visitor_onsite_pass_approvals(
		&self,
		_window: &mut Window,
		cx: &mut Context<Self>,
	) -> Div {
		h_flex()
			.bg(cx.theme().accent.opacity(0.30))
			.p_5()
			.rounded_xl()
			.gap_5()
			.child(
				div()
					.flex()
					.items_center()
					.justify_center()
					.p_4()
					.rounded_full()
					.bg(cx.theme().yellow_light.opacity(0.10))
					.child(
						Icon::empty()
							.size_4()
							.path("icons/custom/badge-id.svg")
							.text_color(cx.theme().yellow_light),
					),
			)
	}

	fn employee_info_update_progress(
		&self,
		_window: &mut Window,
		cx: &mut Context<Self>,
	) -> Div {
		h_flex()
			.bg(cx.theme().accent.opacity(0.30))
			.p_5()
			.rounded_xl()
			.gap_5()
			.child(
				div()
					.flex()
					.items_center()
					.justify_center()
					.p_4()
					.rounded_full()
					.bg(cx.theme().yellow_light.opacity(0.10))
					.child(
						Icon::empty()
							.size_4()
							.path("icons/custom/user-edit.svg")
							.text_color(cx.theme().yellow_light),
					),
			)
	}
}

impl Render for DashboardActionQueues {
	fn render(
		&mut self,
		window: &mut Window,
		cx: &mut Context<Self>,
	) -> impl IntoElement {
		div()
			.grid()
			.grid_cols(6)
			.gap_5()
			.items_center()
			.child(self.approve_leave_requests(window, cx))
			.child(self.approve_overtime_request(window, cx))
			.child(self.pending_contract_renewals(window, cx))
			.child(self.probationary_evaluation_due(window, cx))
			.child(self.visitor_onsite_pass_approvals(window, cx))
			.child(self.employee_info_update_progress(window, cx))
	}
}
