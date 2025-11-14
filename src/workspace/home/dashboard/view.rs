use gpui::*;
use gpui_component::{
    ActiveTheme, IconName, Sizable, StyledExt, WindowExt,
    button::{Button, ButtonCustomVariant, ButtonVariants},
    v_flex,
};

use crate::workspace::home::dashboard::content::DashboardContent;

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
                            .foreground(cx.theme().background)
                            .border(cx.theme().blue)
                            .hover(cx.theme().blue.opacity(0.80))
                            .active(cx.theme().blue),
                    )
                    .rounded_full()
                    .icon(IconName::Plus)
                    .label("Add Employee")
                    .cursor_pointer()
                    .on_click(|_, window, cx| {
                        println!("Open dialog");
                        window.open_dialog(cx, |dialog, _, _| {
                            dialog
                                .p_5()
                                .rounded_lg()
                                .title("Add Employee")
                                .child("thisis a dialog")
                        });
                    }),
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
