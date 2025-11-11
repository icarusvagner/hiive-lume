use gpui::*;
use gpui_component::{ActiveTheme, Icon, IconName};

pub struct FooterBar {}

const VERSION: &str = env!("CARGO_PKG_VERSION");

impl FooterBar {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Render for FooterBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let curated_by = div()
            .flex()
            .flex_row()
            .items_center()
            .justify_center()
            .pr_2()
            .gap_1()
            .text_xs()
            .opacity(0.6)
            .child("Devixion")
            .child(Icon::new(IconName::Frame));

        let version = div()
            .flex()
            .flex_row()
            .items_center()
            .justify_center()
            .pr_2()
            .gap_1()
            .text_xs()
            .opacity(0.6)
            .child(format!("v{}", VERSION))
            .child(Icon::new(IconName::Heart));

        div()
            .border_t_1()
            .text_xs()
            .bg(cx.theme().title_bar)
            .border_color(cx.theme().border)
            .flex()
            .flex_row()
            .justify_between()
            .items_center()
            .py_1()
            .px_2()
            .child(curated_by)
            .child(version)
    }
}
