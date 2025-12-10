use gpui::*;
use gpui_component::{
	ActiveTheme, Icon, StyledExt, h_flex, label::Label, tooltip::Tooltip, v_flex, white
};

pub struct DashboardCards;

impl DashboardCards {
	pub fn view(_window: &mut Window, cx: &mut App) -> Entity<Self> {
		cx.new(|_| Self)
	}
}

impl Render for DashboardCards {
	fn render(
		&mut self,
		_window: &mut Window,
		cx: &mut Context<Self>,
	) -> impl IntoElement {
		div()
			.grid()
			.grid_cols(3)
			.gap_5()
			// first card
			.child(
				v_flex()
					.p_4()
					.rounded_lg()
					.bg(cx.theme().blue_light)
					.items_center()
					.justify_center()
					.text_color(white())
					.child(
						Icon::empty()
							.path("icons/custom/users-more.svg")
							.size_12()
							.mb_2(),
					)
					.child(Label::new("People Flow").text_lg().font_medium())
					.child(
						div().child("200/200").text_2xl().font_black().mb_3(),
					)
					.child(
						h_flex()
							.w_full()
							.items_center()
							.justify_between()
							.child(
								v_flex()
									.items_start()
									.justify_center()
									.child(
										Label::new("New Applicants")
											.text_sm()
											.font_normal(),
									)
									.child(
										Label::new("15")
											.text_sm()
											.font_normal(),
									),
							)
							.child(
								v_flex()
									.items_start()
									.justify_center()
									.child(
										Label::new("New Hires")
											.text_sm()
											.font_normal(),
									)
									.child(
										Label::new("25")
											.text_sm()
											.font_normal(),
									),
							)
							.child(
								v_flex()
									.items_start()
									.justify_center()
									.child(
										Label::new("Active Employees")
											.text_sm()
											.font_normal(),
									)
									.child(
										Label::new("185/200")
											.text_sm()
											.font_normal(),
									),
							),
					),
			)
			// second card
			.child(
				v_flex()
					.p_4()
					.rounded_lg()
					.bg(cx.theme().red_light)
					.items_center()
					.justify_center()
					.text_color(white())
					.child(
						Icon::empty()
							.path("icons/custom/calendar-days.svg")
							.size_12()
							.mb_2(),
					)
					.child(
						Label::new("Attendance Pulse").text_lg().font_medium(),
					)
					.child(
						div()
							.child("195/200")
							.text_2xl()
							.font_black()
							.mb_3()
							.id("ap-id")
							.tooltip(|window, cx| {
								Tooltip::new("Total present people")
									.build(window, cx)
							}),
					)
					.child(
						h_flex()
							.w_full()
							.items_center()
							.justify_between()
							.child(
								v_flex()
									.items_start()
									.justify_center()
									.child(
										Label::new("Time-in")
											.text_sm()
											.font_normal(),
									)
									.child(
										Label::new("85%")
											.text_sm()
											.font_normal(),
									),
							)
							.child(
								v_flex()
									.items_start()
									.justify_center()
									.child(
										Label::new("OT HRs")
											.text_sm()
											.font_normal(),
									)
									.child(
										Label::new("100")
											.text_sm()
											.font_normal(),
									),
							)
							.child(
								v_flex()
									.items_start()
									.justify_center()
									.child(
										div()
											.id("lrp-id")
											.child("LRP")
											.text_sm()
											.font_normal()
											.tooltip(|window, cx| {
												Tooltip::new(
													"Leave Requests Pending",
												)
												.build(window, cx)
											}),
									)
									.child(
										Label::new("10")
											.text_sm()
											.font_normal(),
									),
							),
					),
			)
			// third card
			.child(
				v_flex()
					.p_4()
					.rounded_lg()
					.bg(cx.theme().green_light)
					.items_center()
					.justify_center()
					.text_color(white())
					.child(
						Icon::empty()
							.path("icons/custom/hand-coins.svg")
							.size_12()
							.mb_2(),
					)
					.child(
						Label::new("Payroll Snapshot").text_lg().font_medium(),
					)
					.child(
						div()
							.id("pr-ss")
							.child("189/200")
							.text_2xl()
							.font_black()
							.mb_3()
							.tooltip(|window, cx| {
								Tooltip::new("Total success payroll")
									.build(window, cx)
							}),
					)
					.child(
						h_flex()
							.w_full()
							.items_center()
							.justify_between()
							.child(
								v_flex()
									.items_start()
									.justify_center()
									.child(
										Label::new("Completed")
											.text_sm()
											.font_normal(),
									)
									.child(
										Label::new("189")
											.text_sm()
											.font_normal(),
									),
							)
							.child(
								v_flex()
									.items_start()
									.justify_center()
									.child(
										Label::new("Processing")
											.text_sm()
											.font_normal(),
									)
									.child(
										Label::new("6").text_sm().font_normal(),
									),
							)
							.child(
								v_flex()
									.items_start()
									.justify_center()
									.child(
										Label::new("Open")
											.text_sm()
											.font_normal(),
									)
									.child(
										Label::new("5").text_sm().font_normal(),
									),
							),
					),
			)
	}
}
