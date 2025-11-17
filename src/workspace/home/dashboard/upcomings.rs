use gpui::*;
use gpui_component::{ActiveTheme, Icon, IconName, StyledExt, h_flex, label::Label, v_flex};

use crate::data::home::interviews_data::InterviewsData;

pub struct UpcomingInterviews;

impl UpcomingInterviews {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn render_filter_button(&self, _window: &mut Window, cx: &mut Context<Self>) -> Div {
        h_flex()
            .gap_5()
            .child(
                div()
                    .px_3()
                    .py_1()
                    .rounded_full()
                    .border_1()
                    .border_color(cx.theme().secondary)
                    .child("Today")
                    .hover(|this| {
                        this.bg(cx.theme().primary.opacity(0.40))
                            .text_color(white())
                    })
                    .cursor_pointer(),
            )
            .child(
                div()
                    .px_3()
                    .py_1()
                    .rounded_full()
                    .border_1()
                    .border_color(cx.theme().secondary)
                    .child("Tomorrow")
                    .hover(|this| {
                        this.bg(cx.theme().primary.opacity(0.40))
                            .text_color(white())
                    })
                    .cursor_pointer(),
            )
    }

    fn render_interviews(&self, _window: &mut Window, cx: &mut Context<Self>) -> Vec<Div> {
        let data = InterviewsData::data();
        let mut cards = Vec::new();

        for item in data {
            let card = div()
                .flex()
                .gap_2()
                .items_center()
                .justify_between()
                .child(
                    h_flex()
                        .gap_1()
                        .child(
                            div()
                                .bg(cx.theme().primary.opacity(0.20))
                                .rounded_full()
                                .h_10()
                                .w_10()
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(
                                    img(item.src())
                                        .h_full()
                                        .w_full()
                                        .object_fit(ObjectFit::Cover)
                                        .with_fallback(|| {
                                            div()
                                                .flex()
                                                .h_full()
                                                .w_full()
                                                .items_center()
                                                .justify_center()
                                                .child(Icon::new(IconName::User))
                                                .into_any()
                                        }),
                                ),
                        )
                        .child(
                            v_flex()
                                .child(div().child(item.fullname()).text_sm().font_bold())
                                .child(div().child(item.position()).text_xs()),
                        ),
                )
                .child(
                    h_flex()
                        .px_3()
                        .py(px(2.5))
                        .rounded_full()
                        .bg(cx.theme().primary.opacity(0.20))
                        .gap_1()
                        .child(div().child(item.time_start()).text_sm())
                        .child(div().child(item.end_time()).text_sm()),
                );

            cards.push(card);
        }

        cards
    }

    fn render_interviews_card(&self, window: &mut Window, cx: &mut Context<Self>) -> Div {
        div()
            .grid()
            .grid_cols(2)
            .gap_5()
            .children(self.render_interviews(window, cx))
    }
}

impl Render for UpcomingInterviews {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .rounded_xl()
            .border_1()
            .border_color(black().opacity(0.20))
            .bg(cx.theme().secondary)
            .p_4()
            .w_full()
            .h_full()
            .gap_5()
            .child(Label::new("Upcoming Interviews").text_lg().font_bold())
            .child(self.render_filter_button(window, cx))
            .child(self.render_interviews_card(window, cx))
    }
}
