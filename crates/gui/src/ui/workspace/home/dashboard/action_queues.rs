use gpui::*;
use gpui_component::{
	ActiveTheme, Icon, StyledExt, h_flex, label::Label, v_flex
};

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
					.gap_4()
					.rounded_full()
					.bg(cx.theme().yellow_light.opacity(0.10))
					.child(
						Icon::empty()
							.size_4()
							.path("icons/custom/calendar-plus.svg")
							.text_color(cx.theme().yellow_light),
					),
			)
			.child(
				v_flex()
					.items_start()
					.justify_center()
					.child(
						Label::new("Approved Leave Requests")
							.text_sm()
							.font_medium(),
					)
					.child(Label::new("15").text_2xl().font_bold()),
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
					.bg(cx.theme().red_light.opacity(0.10))
					.child(
						Icon::empty()
							.size_4()
							.path("icons/custom/clock-fading.svg")
							.text_color(cx.theme().red_light),
					),
			)
			.child(
				v_flex()
					.items_start()
					.justify_center()
					.child(
						Label::new("Approved OT Requests")
							.text_sm()
							.font_medium(),
					)
					.child(Label::new("150").text_2xl().font_bold()),
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
					.bg(cx.theme().blue_light.opacity(0.10))
					.child(
						Icon::empty()
							.size_4()
							.path("icons/custom/paper-recycle.svg")
							.text_color(cx.theme().blue_light),
					),
			)
			.child(
				v_flex()
					.items_start()
					.justify_center()
					.child(
						Label::new("Pending Contract Renewals")
							.text_sm()
							.font_medium(),
					)
					.child(Label::new("50").text_2xl().font_bold()),
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
					.bg(cx.theme().green_light.opacity(0.10))
					.child(
						Icon::empty()
							.size_4()
							.path("icons/custom/time-progress.svg")
							.text_color(cx.theme().green_light),
					),
			)
			.child(
				v_flex()
					.items_start()
					.justify_center()
					.child(
						Label::new("Probationary Evaluations Due")
							.text_sm()
							.font_medium(),
					)
					.child(Label::new("20").text_2xl().font_bold()),
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
			.grid_cols(4)
			.gap_5()
			.items_center()
			.child(self.approve_leave_requests(window, cx))
			.child(self.approve_overtime_request(window, cx))
			.child(self.pending_contract_renewals(window, cx))
			.child(self.probationary_evaluation_due(window, cx))
	}
}
