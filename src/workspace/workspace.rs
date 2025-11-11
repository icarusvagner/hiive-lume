use gpui::*;
use gpui_component::{ActiveTheme, indicator::Indicator};

use crate::{
    states::show_layout::ActiveLayout,
    workspace::{footer::FooterBar, header::HeaderBar, home::HomeSpace, login::LoginSpace},
};

pub struct Workspace {
    pub header_bar: Entity<HeaderBar>,
    pub active: ActiveLayout,
    pub login_space: Entity<LoginSpace>,
    pub home_space: Entity<HomeSpace>,
    pub footer_bar: Entity<FooterBar>,
}

impl Workspace {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let header_bar = HeaderBar::view(window, cx);
        let login_space = LoginSpace::view(window, cx);
        let home_space = HomeSpace::view(window, cx);
        let footer_bar = FooterBar::view(window, cx);

        Self {
            header_bar,
            active: ActiveLayout::Login,
            login_space,
            home_space,
            footer_bar,
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn render_loading(&mut self, cx: &mut Context<Self>) -> Stateful<Div> {
        let content = div()
            .id("loading-content")
            .flex()
            .flex_grow()
            .bg(cx.theme().background)
            .justify_center()
            .items_center()
            .child(
                div()
                    .flex()
                    .items_center()
                    .child(Indicator::new())
                    .child("Loading"),
            );

        content
    }

    fn render_login(&mut self, cx: &mut Context<Self>) -> Stateful<Div> {
        let content = div()
            .id("login-content")
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .bg(cx.theme().background)
            .child(self.login_space.clone());

        content
    }

    fn render_home(&mut self, cx: &mut Context<Self>) -> Stateful<Div> {
        let content = div()
            .id("home-content")
            .flex()
            .bg(cx.theme().background)
            .child(self.home_space.clone());

        content
    }
}

impl Render for Workspace {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content = match self.active {
            ActiveLayout::Login => self.render_login(cx),
            ActiveLayout::Home => self.render_home(cx),
            ActiveLayout::Loading => self.render_loading(cx),
        };

        div()
            .flex()
            .flex_col()
            .size_full()
            .child(self.header_bar.clone())
            .child(
                div()
                    .flex()
                    .flex_grow()
                    .items_center()
                    .justify_center()
                    .child(content),
            )
            .child(self.footer_bar.clone())
    }
}
