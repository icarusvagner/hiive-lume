use chrono::NaiveDateTime;
use gpui::*;
use gpui_component::{
    ActiveTheme, StyledExt,
    avatar::Avatar,
    chart::{AreaChart, PieChart},
    h_flex,
    label::Label,
    radio::RadioGroup,
    v_flex,
};

use crate::{
    core::types::attendance_filter::AttendanceOverviewFilter,
    data::home::{
        attendance_overview_data::AttendanceOverviewData,
        dashboard_card_data::{DashboardCardData, DashboardPieCard},
        events_news_data::EventsNewsData,
    },
};

#[derive(Clone)]
pub struct DashboardContent {
    filter_index: Option<usize>,
    filter_type: Option<AttendanceOverviewFilter>,
}

impl DashboardContent {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {
            filter_index: None,
            filter_type: None,
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
                .shadow_2xl()
                .border_1()
                .border_color(black().opacity(0.40))
                .bg(cx.theme().secondary)
                .p_12()
                .w_full()
                .child(
                    Avatar::new()
                        .p_4()
                        .placeholder(card.icon().text_color(cx.theme().foreground))
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

        h_flex()
            .rounded_xl()
            .shadow_2xl()
            .border_1()
            .border_color(black().opacity(0.40))
            .bg(cx.theme().secondary)
            .p_12()
            .w_full()
            .justify_between()
            .items_center()
            .child(
                div()
                    .relative()
                    .child(
                        div().absolute().top_1().left_3().w_56().child(
                            PieChart::new(data.clone())
                                .value(|d| d.data as f32)
                                .outer_radius(60.)
                                .inner_radius(60.)
                                .pad_angle(25. / 100.)
                                .color(|d| d.color),
                        ),
                    )
                    .child(
                        div().absolute().w_32().top_3().left_5().child(
                            v_flex()
                                .items_center()
                                .justify_center()
                                .child(div().child(format!("{total_count}")).text_xs())
                                .child(div().child("Employees").text_sm().font_semibold()),
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
                                    .child(format!("{:.1}%", (data[0].data / total_count) * 100.0))
                                    .text_sm()
                                    .font_thin(),
                            )
                            .child(div().child(data[0].label.clone()).text_lg().font_bold()),
                    )
                    .child(
                        v_flex()
                            .items_start()
                            .justify_center()
                            .child(
                                div()
                                    .child(format!("{:.1}%", (data[1].data / total_count) * 100.0))
                                    .text_sm()
                                    .font_thin(),
                            )
                            .child(div().child(data[1].label.clone()).text_lg().font_bold()),
                    ),
            )
    }

    fn render_attendance_overview(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let data = AttendanceOverviewData::all_data();

        v_flex()
            .bg(cx.theme().secondary)
            .p_12()
            .gap_3()
            .border_1()
            .border_color(black().opacity(0.40))
            .rounded_xl()
            .shadow_2xl()
            .h_full()
            .max_w(px(900.0))
            .flex_1()
            .child(
                h_flex()
                    .justify_between()
                    .child(Label::new("Attendance Overview").text_lg().font_bold())
                    .child(
                        h_flex().child(
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
                div().flex_1().py_4().child(
                    AreaChart::new(data)
                        .x(|d| d.month.clone())
                        .y(|d| d.on_time)
                        .stroke(cx.theme().chart_5)
                        .fill(linear_gradient(
                            0.,
                            linear_color_stop(cx.theme().chart_5.opacity(0.4), 1.),
                            linear_color_stop(cx.theme().background.opacity(0.3), 0.),
                        ))
                        .y(|d| d.late_arrival)
                        .stroke(cx.theme().chart_2)
                        .fill(linear_gradient(
                            0.,
                            linear_color_stop(cx.theme().chart_2, 1.),
                            linear_color_stop(cx.theme().background.opacity(0.3), 0.),
                        ))
                        .y(|d| d.absent)
                        .stroke(cx.theme().chart_3)
                        .fill(linear_gradient(
                            0.,
                            linear_color_stop(cx.theme().chart_3, 1.),
                            linear_color_stop(cx.theme().background.opacity(0.3), 0.),
                        ))
                        .linear(),
                ),
            )
    }

    fn render_events_news(&self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let data = EventsNewsData::all_data();
        let mut events = Vec::new();

        for event in data {
            let dt = NaiveDateTime::parse_from_str(&event.date_time, "%Y-%m-%d %H:%M:%S")
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

        v_flex()
            .w(px(900.0))
            .bg(cx.theme().secondary)
            .p_12()
            .gap_3()
            .border_1()
            .border_color(black().opacity(0.40))
            .rounded_xl()
            .shadow_2xl()
            .child(div().child("News & Events").text_xl().font_bold())
            .child(div().grid().grid_cols(2).gap_8().children(events))
    }
}

impl Render for DashboardContent {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .px_10()
            .py_6()
            .gap_5()
            .h_full()
            .flex_1()
            .child(
                h_flex()
                    .items_center()
                    .justify_between()
                    .gap_8()
                    .children(self.cards(cx))
                    .child(self.pie_chart_card(cx)),
            )
            .child(
                h_flex()
                    .gap_8()
                    .child(self.render_attendance_overview(window, cx))
                    .child(self.render_events_news(window, cx)),
            )
            .scrollable(Axis::Vertical)
    }
}
