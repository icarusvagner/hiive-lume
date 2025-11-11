use gpui::{prelude::FluentBuilder, *};
use gpui_component::{
    ActiveTheme, IconName, Sizable, StyledExt, ThemeMode, TitleBar,
    button::{Button, ButtonVariants},
    h_flex,
    label::Label,
};

use crate::themes::change_color_mode;

pub struct HeaderBar {}

impl HeaderBar {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    pub fn change_mode(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        let new_mode = if cx.theme().mode.is_dark() {
            ThemeMode::Light
        } else {
            ThemeMode::Dark
        };

        change_color_mode(new_mode, window, cx);
    }
}

impl Render for HeaderBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme_toggle = Button::new("theme-toggler-btn")
            .map(|this| {
                if cx.theme().mode.is_dark() {
                    this.icon(IconName::Sun)
                } else {
                    this.icon(IconName::Moon)
                }
            })
            .small()
            .ghost()
            .on_click(cx.listener(Self::change_mode));

        let github_button = Button::new("github-btn")
            .icon(IconName::GitHub)
            .small()
            .ghost()
            .on_click(|_, _, cx| cx.open_url("https://github.com/icarusvagner/hiive-lume.git"));

        TitleBar::new().child(
            h_flex()
                .w_full()
                .pr_2()
                .justify_between()
                .child(Label::new("Hiive").text_xs().font_medium())
                .child(
                    div()
                        .pr(px(5.))
                        .flex()
                        .items_center()
                        .child(theme_toggle)
                        .child(github_button),
                ),
        )
    }
}
