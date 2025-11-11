use gpui::*;
use gpui_component::{
    Icon, IconName, Sizable, StyledExt,
    button::{Button, ButtonVariants},
    label::Label,
};

pub struct HomeHeader {}

impl HomeHeader {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Render for HomeHeader {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .py(px(5.))
            .px(px(2.5))
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .flex()
                    .gap_3()
                    .items_center()
                    .child(img("images/hiive-logo.png").size(px(30.)))
                    .child(Label::new("Hiive Lume").font_bold().text_lg()),
            )
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(
                        Button::new("dashboard-btn")
                            .ghost()
                            .large()
                            .icon(IconName::LayoutDashboard)
                            .label("Dashboard"),
                    )
                    .child(
                        Button::new("employees-btn")
                            .ghost()
                            .large()
                            .icon(IconName::User)
                            .label("Employees"),
                    )
                    .child(
                        Button::new("jobs-btn")
                            .ghost()
                            .large()
                            .icon(Icon::empty().path("icons/custom/users-round-outline.svg"))
                            .label("Jobs"),
                    ),
            )
    }
}
