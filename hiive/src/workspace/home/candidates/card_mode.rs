use gpui::*;
use gpui_component::{
    ActiveTheme, Icon, IconName, Sizable, StyledExt,
    button::{Button, ButtonCustomVariant, ButtonVariants},
    h_flex,
    label::Label,
    menu::DropdownMenu,
    v_flex,
};

use crate::{
    data::home::candidates_data::Candidatesdata,
    workspace::{
        components::card::custom_card,
        global_actions::{RemoveCandidte, ViewCandidate},
    },
};

pub struct CandidateCardModeView;

impl CandidateCardModeView {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn render_cards(&self, window: &mut Window, cx: &mut Context<Self>) -> Vec<Div> {
        let data = Candidatesdata::data();
        let mut cards = Vec::new();

        for (index, item) in data.iter().enumerate() {
            let content = v_flex().gap_1().child(
                div()
                    .relative()
                    .child(
                        div().absolute().right_0().top_0().child(
                            Button::new(SharedString::new(format!("btn-card-candidate-{index}")))
                                .ghost()
                                .rounded_full()
                                .icon(
                                    Icon::new(IconName::EllipsisVertical)
                                        .text_color(cx.theme().foreground),
                                )
                                .dropdown_menu(|menu, _, _| {
                                    menu.menu("View", Box::new(ViewCandidate))
                                        .menu("Remove", Box::new(RemoveCandidte))
                                }),
                        ),
                    )
                    .child(
                        v_flex()
                            .child(
                                div().flex().items_center().justify_center().child(
                                    div().h_16().w_16().child(
                                        div().overflow_hidden().rounded_full().relative().child(
                                            img(item.src())
                                                .h_full()
                                                .w_full()
                                                .rounded_full()
                                                .object_fit(ObjectFit::Cover),
                                        ),
                                    ),
                                ),
                            )
                            .child(
                                Label::new(item.fullname())
                                    .text_lg()
                                    .font_bold()
                                    .text_center(),
                            )
                            .child(
                                Label::new(item.position())
                                    .text_sm()
                                    .font_thin()
                                    .text_center(),
                            )
                            .child(
                                v_flex()
                                    .mt_1()
                                    .justify_center()
                                    .items_center()
                                    .px_2()
                                    .py_1()
                                    .rounded_full()
                                    .bg(item.status().color().opacity(0.20))
                                    .child(
                                        Label::new(item.status().to_string().to_uppercase())
                                            .font_bold()
                                            .text_center(),
                                    ),
                            )
                            .child(
                                v_flex()
                                    .mt_4()
                                    .bg(cx.theme().background)
                                    .rounded_lg()
                                    .p_3()
                                    .child(
                                        h_flex()
                                            .gap_2()
                                            .child(Icon::new(
                                                Icon::empty()
                                                    .path("icons/custom/envelope-outline.svg"),
                                            ))
                                            .child(Label::new(item.email()).text_sm()),
                                    )
                                    .child(
                                        h_flex()
                                            .gap_2()
                                            .child(Icon::new(
                                                Icon::empty()
                                                    .path("icons/custom/smartphone-outline.svg"),
                                            ))
                                            .child(Label::new(item.number()).text_sm()),
                                    ),
                            )
                            .child(
                                v_flex()
                                    .mt_2()
                                    .bg(cx.theme().primary.opacity(0.10))
                                    .rounded_lg()
                                    .p_3()
                                    .child(
                                        h_flex()
                                            .gap_2()
                                            .justify_between()
                                            .child(Label::new("Experience").text_sm())
                                            .child(
                                                Label::new(item.experience()).text_sm().font_bold(),
                                            ),
                                    )
                                    .child(
                                        h_flex()
                                            .gap_2()
                                            .justify_between()
                                            .child(Label::new("Applied On").text_sm())
                                            .child(
                                                Label::new(item.date_applied())
                                                    .text_sm()
                                                    .font_bold(),
                                            ),
                                    ),
                            )
                            .child(
                                div()
                                    .mt_3()
                                    .grid()
                                    .grid_cols(2)
                                    .gap_3()
                                    .child(
                                        Button::new("btn-candidate-notes")
                                            .custom(
                                                ButtonCustomVariant::new(cx)
                                                    .color(cx.theme().yellow.opacity(0.60))
                                                    .foreground(cx.theme().background)
                                                    .border(cx.theme().yellow)
                                                    .hover(cx.theme().yellow)
                                                    .active(cx.theme().yellow),
                                            )
                                            .label("Notes")
                                            .text_center()
                                            .large()
                                            .p_3()
                                            .rounded_full(),
                                    )
                                    .child(
                                        Button::new("btn-candidate-view")
                                            .custom(
                                                ButtonCustomVariant::new(cx)
                                                    .color(cx.theme().primary.opacity(0.60))
                                                    .foreground(cx.theme().background)
                                                    .border(cx.theme().primary)
                                                    .hover(cx.theme().primary)
                                                    .active(cx.theme().primary),
                                            )
                                            .label("View")
                                            .text_center()
                                            .large()
                                            .p_3()
                                            .rounded_full(),
                                    ),
                            ),
                    ),
            );

            let card = div().child(custom_card(content, window, cx));

            cards.push(card);
        }

        cards
    }
}

impl Render for CandidateCardModeView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .grid()
            .grid_cols(5)
            .gap_5()
            .h_full()
            .children(self.render_cards(window, cx))
    }
}
