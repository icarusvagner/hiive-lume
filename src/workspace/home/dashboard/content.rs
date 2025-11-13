use gpui::*;
use gpui_component::{ActiveTheme, StyledExt, avatar::Avatar, chart::PieChart, h_flex, v_flex};

use crate::data::home::dashboard_card_data::{DashboardCardData, DashboardPieCard};

#[derive(Clone)]
pub struct DashboardContent {}

impl DashboardContent {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
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
                .p(px(22.0))
                .w_full()
                .child(
                    Avatar::new()
                        .p_4()
                        .placeholder(card.icon().text_color(black().opacity(0.70)))
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

        h_flex()
            .rounded_xl()
            .shadow_2xl()
            .border_1()
            .border_color(black().opacity(0.40))
            .bg(cx.theme().secondary)
            .p(px(22.0))
            .w_full()
            .justify_between()
            .child(
                PieChart::new(data.clone())
                    .value(|d| d.data as f32)
                    .outer_radius(60.)
                    .inner_radius(20.)
                    .pad_angle(25. / 100.),
            )
            .child(
                v_flex()
                    .gap_2()
                    .child(
                        v_flex()
                            .items_start()
                            .justify_center()
                            .child(div().child("23%").text_sm().font_thin())
                            .child(div().child("Remote").text_lg().font_bold()),
                    )
                    .child(
                        v_flex()
                            .items_start()
                            .justify_center()
                            .child(div().child("77%").text_sm().font_thin())
                            .child(div().child("Office").text_lg().font_bold()),
                    ),
            )
    }
}

impl Render for DashboardContent {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().px_10().py_6().child(
            h_flex()
                .items_center()
                .justify_between()
                .gap_8()
                .children(self.cards(cx))
                .child(self.pie_chart_card(cx)),
        )
    }
}
