use gpui::*;
use gpui_component::{
    ActiveTheme, IconName, Sizable, StyledExt,
    button::{Button, ButtonCustomVariant, ButtonVariants},
    v_flex,
};

use crate::{
    states::home_layout::{HomeActiveLayout, HomeLayout},
    workspace::home::dashboard::content::DashboardContent,
};

pub struct Dashboard {
    cards: Entity<DashboardContent>,
}

impl Dashboard {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let cards = DashboardContent::view(window, cx);

        Self { cards }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn navigate_content(&self, layout: HomeActiveLayout, cx: &mut App) {
        let _ = cx.update_global::<HomeLayout, _>(|state, _| {
            if !state.home.eq(&layout) {
                state.home = HomeActiveLayout::Loading;
            }
        });

        cx.spawn(async move |cx| {
            cx.background_executor()
                .timer(std::time::Duration::from_millis(500))
                .await;

            let _ = cx.update_global::<HomeLayout, _>(|state, _| {
                state.home = layout;
            });
        })
        .detach();
    }

    fn render_top_content(&mut self, cx: &mut Context<Self>) -> Div {
        div()
            .flex()
            .px_10()
            .py_6()
            .bg(cx.theme().accent)
            .justify_between()
            .items_center()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .child("Hello John Doe")
                            .text_size(AbsoluteLength::Pixels(px(22.0)))
                            .text_color(cx.theme().accent_foreground)
                            .font_thin(),
                    )
                    .child(
                        div()
                            .child("Good Morning.")
                            .text_size(AbsoluteLength::Pixels(px(55.0)))
                            .font_bold(),
                    ),
            )
            .child(
                Button::new("add-employee-btn")
                    .large()
                    .custom(
                        ButtonCustomVariant::new(cx)
                            .color(cx.theme().blue)
                            .foreground(cx.theme().background)
                            .border(cx.theme().blue)
                            .hover(cx.theme().blue.opacity(0.80))
                            .active(cx.theme().blue),
                    )
                    .text_color(white())
                    .rounded_full()
                    .icon(IconName::Plus)
                    .label("Add Employee")
                    .cursor_pointer()
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.navigate_content(HomeActiveLayout::CreateEmployee, cx)
                    })),
            )
    }
}

impl Render for Dashboard {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .child(self.render_top_content(cx))
            .child(self.cards.clone())
    }
}
