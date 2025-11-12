use gpui::*;
use gpui_component::ActiveTheme;

use crate::workspace::login::{left_pane::LeftPane, right_pane::RightPane};

mod left_pane;
mod login_form;
mod right_pane;

pub struct LoginSpace {
    left_pane: Entity<LeftPane>,
    right_pane: Entity<RightPane>,
}

impl LoginSpace {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let left_pane = LeftPane::view("images/login-office.jpeg".into(), window, cx);
        let right_pane = RightPane::view(window, cx);

        Self {
            left_pane,
            right_pane,
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Render for LoginSpace {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_center()
            .size_full()
            .min_h_full()
            .bg(cx.theme().background)
            .child(
                div()
                    .flex()
                    .rounded_xl()
                    .bg(cx.theme().secondary)
                    .shadow_lg()
                    .border_1()
                    .border_color(cx.theme().accent)
                    .overflow_hidden()
                    .w(px(1020.))
                    .h(px(720.))
                    .child(self.left_pane.clone())
                    .child(self.right_pane.clone()),
            )
    }
}
