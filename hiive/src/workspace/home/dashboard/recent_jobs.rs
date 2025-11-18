use gpui::*;
use gpui_component::{ActiveTheme, StyledExt, avatar::Avatar, h_flex, label::Label, v_flex};

use crate::data::home::job_applicants_data::JobApplicantData;

pub struct RecentJobApplications;

impl RecentJobApplications {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn render_applicants(&self, _window: &mut Window, cx: &mut Context<Self>) -> Div {
        let data = JobApplicantData::data()[0..5].to_vec();
        let mut applications_card = Vec::new();

        for item in data {
            let card_item = h_flex()
                .gap_3()
                .justify_between()
                .items_center()
                .child(
                    h_flex()
                        .gap_2()
                        .child(
                            Avatar::new()
                                .name(format!(
                                    "{:?}{:?}",
                                    item.fullname().chars().next(),
                                    item.fullname().chars().last()
                                ))
                                .text_color(cx.theme().foreground),
                        )
                        .child(
                            v_flex().child(Label::new(item.fullname())).child(
                                Label::new(item.location())
                                    .text_xs()
                                    .text_color(cx.theme().foreground.opacity(0.60)),
                            ),
                        ),
                )
                .child(
                    div()
                        .rounded_full()
                        .px_3()
                        .py_2()
                        .bg(cx.theme().cyan.opacity(0.20))
                        .child(Label::new(item.position())),
                );

            applications_card.push(card_item);
        }

        v_flex().gap_5().children(applications_card)
    }
}

impl Render for RecentJobApplications {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .rounded_xl()
            .border_1()
            .border_color(black().opacity(0.20))
            .bg(cx.theme().secondary)
            .p_4()
            .w_full()
            .h_full()
            .gap_8()
            .child(Label::new("Recent Job Applicants").text_lg().font_bold())
            .child(self.render_applicants(window, cx))
            .scrollable(Axis::Vertical)
    }
}
