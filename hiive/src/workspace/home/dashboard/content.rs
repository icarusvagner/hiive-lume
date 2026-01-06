use chrono::NaiveDateTime;
use gpui::*;
use gpui_component::{
	ActiveTheme, StyledExt, avatar::Avatar, chart::{AreaChart, PieChart}, h_flex, label::Label, radio::RadioGroup, scroll::ScrollableElement, v_flex
};

use crate::{
	core::types::attendance_filter::AttendanceOverviewFilter, data::home::{
		attendance_overview_data::AttendanceOverviewData, dashboard_card_data::{DashboardCardData, DashboardPieCard}, events_news_data::EventsNewsData
	}, workspace::home::{
		components::card::custom_card, dashboard::{
			recent_jobs::RecentJobApplications, upcomings::UpcomingInterviews
		}
	}
};

#[derive(Clone)]
pub struct DashboardContent {
	filter_index: Option<usize>,
	filter_type: Option<AttendanceOverviewFilter>,
	recent_jobs: Entity<RecentJobApplications>,
	upcoming_interviews: Entity<UpcomingInterviews>,
}

impl DashboardContent {
	pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
		let recent_jobs = RecentJobApplications::view(window, cx);
		let upcoming_interviews = UpcomingInterviews::view(window, cx);

		Self {
			filter_index: None,
			filter_type: None,
			recent_jobs,
			upcoming_interviews,
		}
	}

	pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
		cx.new(|cx| Self::new(window, cx))
	}

	fn cards(&self, cx: &mut Context<Self>) -> Vec<Div> {
		let cards_data = DashboardCardData::all_data();
		let mut cards = Vec::new();

		for card in cards_data {
			let card = v_flex()
				.rounded_xl()
				.border_1()
				.border_color(black().opacity(0.20))
				.bg(cx.theme().secondary)
				.p_4()
				.items_start()
				.justify_center()
				.child(
					Avatar::new()
						.p_5()
						.placeholder(
							card.icon().text_color(cx.theme().foreground),
						)
						.bg(cx.theme().blue.opacity(0.40)),
				)
				.child(div().child(card.label()).text_sm())
				.child(div().child(card.content()).text_2xl().font_semibold());

			cards.push(card);
		}

		cards
	}

	fn pie_chart_card(&self, cx: &mut Context<Self>) -> Div {
		let data = DashboardPieCard::data();
		let total_count: f64 = data.iter().map(|x| x.data).sum();

		div()
			.rounded_xl()
			.border_1()
			.border_color(black().opacity(0.20))
			.bg(cx.theme().secondary)
			.grid()
			.grid_cols(2)
			.p_4()
			.child(
				div()
					.relative()
					.child(
						PieChart::new(data.clone())
							.value(|d| d.data as f32)
							.outer_radius(60.)
							.inner_radius(45.)
							.pad_angle(25. / 100.)
							.color(|d| d.color),
					)
					.child(
						div()
							.absolute()
							.top_8()
							.left_11()
							.items_center()
							.justify_center()
							.child(
								div()
									.child(format!("{total_count}"))
									.text_xs()
									.text_center(),
							)
							.child(
								div()
									.child("Employees")
									.text_xs()
									.font_semibold()
									.text_center(),
							),
					),
			)
			.child(
				v_flex()
					.gap_2()
					.child(
						v_flex()
							.items_start()
							.justify_center()
							.child(
								div()
									.child(format!(
										"{:.1}%",
										(data[0].data / total_count) * 100.0
									))
									.text_sm()
									.font_thin(),
							)
							.child(
								div()
									.child(data[0].label.clone())
									.text_lg()
									.font_bold(),
							),
					)
					.child(
						v_flex()
							.items_start()
							.justify_center()
							.child(
								div()
									.child(format!(
										"{:.1}%",
										(data[1].data / total_count) * 100.0
									))
									.text_sm()
									.font_thin(),
							)
							.child(
								div()
									.child(data[1].label.clone())
									.text_lg()
									.font_bold(),
							),
					),
			)
	}

	fn render_attendance_overview(
		&self,
		_window: &mut Window,
		cx: &mut Context<Self>,
	) -> impl IntoElement {
		let data = AttendanceOverviewData::all_data();

		custom_card(
            v_flex()
                .h_full()
                .flex_1()
                .gap_8()
                .child(
                    div()
                        .grid()
                        .grid_cols(2)
                        .child(Label::new("Attendance Overview").text_lg().font_bold())
                        .child(
                            h_flex().w_full().items_center().justify_end().child(
                                RadioGroup::horizontal("filter-attendance")
                                    .children(["On Time", "Late Arrival", "Absent"])
                                    .selected_index(self.filter_index)
                                    .on_click(cx.listener(|this, index, _, cx| {
                                        match index {
                                            0 => {
                                                this.filter_type =
                                                    Some(AttendanceOverviewFilter::OnTime);
                                                this.filter_index = Some(*index);
                                            }
                                            1 => {
                                                this.filter_type =
                                                    Some(AttendanceOverviewFilter::LateArrival);
                                                this.filter_index = Some(*index);
                                            }
                                            2 => {
                                                this.filter_type =
                                                    Some(AttendanceOverviewFilter::Absent);
                                                this.filter_index = Some(*index);
                                            }
                                            _ => {
                                                this.filter_type =
                                                    Some(AttendanceOverviewFilter::OnTime);
                                                this.filter_index = Some(*index);
                                            }
                                        }
                                        cx.notify();
                                    })),
                            ),
                        ),
                )
                .child(
                    h_flex()
                        .gap_2()
                        .child(
                            h_flex()
                                .gap_1()
                                .child(div().h_2().w_2().rounded_full().bg(cx.theme().green))
                                .child(div().text_xs().child("On Time")),
                        )
                        .child(
                            h_flex()
                                .gap_1()
                                .child(div().h_2().w_2().rounded_full().bg(cx.theme().yellow))
                                .child(div().text_xs().child("Late Arrival")),
                        )
                        .child(
                            h_flex()
                                .gap_1()
                                .child(div().h_2().w_2().rounded_full().bg(cx.theme().red))
                                .child(div().text_xs().child("Absent")),
                        ),
                )
                .child(
                    div().flex_1().py_4().child(
                        AreaChart::new(data)
                            .x(|d| d.month.clone())
                            .y(|d| d.on_time)
                            .stroke(cx.theme().green)
                            .fill(linear_gradient(
                                0.,
                                linear_color_stop(cx.theme().green.opacity(0.4), 1.),
                                linear_color_stop(cx.theme().background.opacity(0.3), 0.),
                            ))
                            .y(|d| d.late_arrival)
                            .stroke(cx.theme().yellow)
                            .fill(linear_gradient(
                                0.,
                                linear_color_stop(cx.theme().yellow, 1.),
                                linear_color_stop(cx.theme().background.opacity(0.3), 0.),
                            ))
                            .y(|d| d.absent)
                            .stroke(cx.theme().red)
                            .fill(linear_gradient(
                                0.,
                                linear_color_stop(cx.theme().red, 1.),
                                linear_color_stop(cx.theme().background.opacity(0.3), 0.),
                            ))
                            .linear(),
                    ),
                ),
            cx.theme().secondary,
        )
	}

	fn render_events_news(
		&self,
		_window: &mut Window,
		cx: &mut Context<Self>,
	) -> impl IntoElement {
		let data = EventsNewsData::all_data();
		let mut events = Vec::new();

		for event in data {
			let dt = NaiveDateTime::parse_from_str(
				&event.date_time,
				"%Y-%m-%d %H:%M:%S",
			)
			.expect("Cannot parse date and time str");
			let month = dt.format("%b").to_string();
			let day = dt.format("%d").to_string();

			let event_div = h_flex()
				.gap_4()
				.child(
					v_flex()
						.bg(cx.theme().accent.opacity(0.50))
						.rounded_lg()
						.p_5()
						.child(v_flex().child(day).child(month)),
				)
				.child(
					v_flex()
						.text_ellipsis()
						.flex_wrap()
						.overflow_hidden()
						.child(div().child(event.title).text_lg())
						.child(div().child(event.short_desc).text_xs()),
				);

			events.push(event_div);
		}

		custom_card(
			v_flex()
				.gap_8()
				.child(div().child("News & Events").text_xl().font_bold())
				.child(div().grid().grid_cols(2).gap_8().children(events)),
			cx.theme().secondary,
		)
	}

	fn render_project_overview(&self, cx: &mut Context<Self>) -> Div {
		v_flex()
			.gap_8()
			.child(Label::new("Projects Overview").text_lg().font_bold())
			.child(
				v_flex()
					.p_5()
					.rounded_xl()
					.bg(cx.theme().blue.opacity(0.30))
					.child(Label::new("125").text_lg().font_bold())
					.child(Label::new("Total Projects").text_xs().font_bold()),
			)
			.child(
				v_flex()
					.p_5()
					.rounded_xl()
					.bg(cx.theme().blue.opacity(0.30))
					.child(Label::new("12").text_lg().font_bold())
					.child(
						Label::new("Projects on Hold").text_xs().font_bold(),
					),
			)
			.child(
				v_flex()
					.p_5()
					.rounded_xl()
					.bg(cx.theme().blue.opacity(0.30))
					.child(Label::new("5").text_lg().font_bold())
					.child(
						Label::new("Upcoming Projects").text_xs().font_bold(),
					),
			)
	}
}

impl Render for DashboardContent {
	fn render(
		&mut self,
		window: &mut Window,
		cx: &mut Context<Self>,
	) -> impl IntoElement {
		v_flex()
			.px_10()
			.py_6()
			.gap_8()
			.h_full()
			.flex_1()
			.child(
				div()
					.grid()
					.grid_cols(5)
					.gap_8()
					.children(self.cards(cx))
					.child(self.pie_chart_card(cx)),
			)
			.child(
				div()
					.grid()
					.grid_cols(2)
					.gap_8()
					.child(self.render_attendance_overview(window, cx))
					.child(self.render_events_news(window, cx)),
			)
			.child(
				div()
					.grid()
					.grid_cols(6)
					.gap_8()
					.h_full()
					.child(div().col_span(2).child(self.recent_jobs.clone()))
					.child(
						div()
							.col_span(3)
							.bg(cx.theme().secondary)
							.rounded_xl()
							.border_1()
							.border_color(black().opacity(0.20))
							.p_4()
							.child(self.upcoming_interviews.clone()),
					)
					.child(
						div()
							.col_span(1)
							.bg(cx.theme().secondary)
							.rounded_xl()
							.border_1()
							.border_color(black().opacity(0.20))
							.p_4()
							.child(self.render_project_overview(cx)),
					),
			)
			.overflow_y_scrollbar()
	}
}
