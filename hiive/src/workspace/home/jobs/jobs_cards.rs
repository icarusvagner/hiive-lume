use gpui::*;
use gpui_component::{ActiveTheme, h_flex, label::Label, v_flex};

use crate::{data::models::jobs_model::JobsModel, workspace::components::card::custom_card};

pub struct JobCardsView {
    data: Vec<JobsModel>,
}

impl JobCardsView {
    pub fn new(data: Vec<JobsModel>, _window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self { data }
    }
    pub fn view(data: Vec<JobsModel>, window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(data, window, cx))
    }

    fn render_cards(&self, window: &mut Window, cx: &mut Context<Self>) -> Vec<Div> {
        let mut cards = Vec::new();

        for (index, job) in self.data.clone().iter().enumerate() {
            let content = v_flex()
                .gap_3()
                .child(
                    div().relative().h_16().w_16().child(
                        img(job.image())
                            .h_full()
                            .w_full()
                            .rounded_full()
                            .object_fit(ObjectFit::Cover),
                    ),
                )
                .child(
                    v_flex()
                        .child(
                            Label::new(job.title())
                                .text_lg()
                                .font_weight(FontWeight::BOLD),
                        )
                        .child(
                            Label::new(job.description())
                                .text_sm()
                                .font_weight(FontWeight::THIN)
                                .text_color(cx.theme().foreground.opacity(0.60)),
                        ),
                )
                .child(
                    h_flex()
                        .gap_5()
                        .child(
                            div()
                                .bg(cx.theme().green.opacity(0.30))
                                .px_3()
                                .py_1()
                                .rounded_full()
                                .text_center()
                                .font_weight(FontWeight::SEMIBOLD)
                                .child(format!(
                                    "{} {}",
                                    job.available_position(),
                                    if job.available_position() > 1 {
                                        "Positions"
                                    } else {
                                        "Position"
                                    }
                                )),
                        )
                        .child(
                            div()
                                .bg(cx.theme().magenta.opacity(0.30))
                                .px_3()
                                .py_1()
                                .rounded_full()
                                .text_center()
                                .font_weight(FontWeight::SEMIBOLD)
                                .child(job.job_type().as_str()),
                        )
                        .child(
                            div()
                                .bg(cx.theme().yellow.opacity(0.30))
                                .px_3()
                                .py_1()
                                .rounded_full()
                                .text_center()
                                .font_weight(FontWeight::SEMIBOLD)
                                .child(job.work_type().as_str()),
                        ),
                )
                .cursor_pointer();

            let card = div().child(custom_card(content, window, cx));

            cards.push(card);
        }

        cards
    }
}

impl Render for JobCardsView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .grid()
            .grid_cols(4)
            .gap_8()
            .children(self.render_cards(window, cx))
    }
}
