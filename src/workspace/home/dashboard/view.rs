use gpui::*;
use gpui_component::{
    ActiveTheme, IconName, Sizable, StyledExt,
    button::{Button, ButtonCustomVariant, ButtonVariants},
};

use crate::{
    states::home_layout::HomeLayout, workspace::home::dashboard::content::DashboardContent,
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
                            .foreground(cx.theme().foreground)
                            .border(cx.theme().blue)
                            .hover(cx.theme().blue.opacity(0.80))
                            .active(cx.theme().blue),
                    )
                    .rounded_full()
                    .icon(IconName::Plus)
                    .label("Add Employee")
                    .cursor_pointer()
                    .on_click(cx.listener(|_, _, _, cx| {
                        let state = cx.global_mut::<HomeLayout>();
                        state.home = crate::states::home_layout::HomeActiveLayout::CreateEmployee;
                        cx.notify();
                    })),
            )
    }
}

impl Render for Dashboard {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .child(self.render_top_content(cx))
            .child(self.cards.clone())
            .scrollable(Axis::Vertical)
    }
}
