use gpui::*;
use gpui_component::{ActiveTheme, Root, Sizable, spinner::Spinner, v_flex};

use crate::{
    states::show_layout::{ActiveLayout, LayoutState},
    workspace::{footer::FooterBar, header::HeaderBar, home::HomeSpace, login::LoginSpace},
};

pub struct Workspace {
    header_bar: Entity<HeaderBar>,
    layout: ActiveLayout,
    login_space: Entity<LoginSpace>,
    home_space: Entity<HomeSpace>,
    footer_bar: Entity<FooterBar>,
    _subscription: Vec<Subscription>,
}

impl Workspace {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let header_bar = HeaderBar::view(window, cx);
        let login_space = LoginSpace::view(window, cx);
        let home_space = HomeSpace::view(window, cx);
        let footer_bar = FooterBar::view(window, cx);

        let _subscription = vec![cx.observe_global::<LayoutState>(move |this, cx| {
            this.layout = cx.global::<LayoutState>().layout.clone();
            cx.notify();
        })];

        Self {
            header_bar,
            layout: ActiveLayout::Login,
            login_space,
            home_space,
            footer_bar,
            _subscription,
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
                v_flex()
                    .items_center()
                    .justify_center()
                    .gap_10()
                    .child(Spinner::new().color(cx.theme().blue).with_size(px(100.)))
                    .child("Loading"),
            );

        content
    }

    fn render_login(&mut self, cx: &mut Context<Self>) -> Stateful<Div> {
        let content = div()
            .id("login-content")
            .flex()
            .flex_grow()
            .overflow_hidden()
            .items_center()
            .justify_center()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .bg(cx.theme().background)
                    .child(self.login_space.clone()),
            );

        content
    }

    fn render_home(&mut self, cx: &mut Context<Self>) -> Stateful<Div> {
        let content = v_flex()
            .id("home-content")
            .flex_1()
            .flex_grow()
            .size_full()
            .bg(cx.theme().background.opacity(0.30))
            .child(self.home_space.clone());

        content
    }
}

impl Render for Workspace {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content = match self.layout {
            ActiveLayout::Login => self.render_login(cx),
            ActiveLayout::Home => self.render_home(cx),
            ActiveLayout::Loading => self.render_loading(cx),
        };

        v_flex()
            .overflow_hidden()
            .size_full()
            .child(self.header_bar.clone())
            .child(content)
            .child(self.footer_bar.clone())
            .children(Root::render_dialog_layer(window, cx))
            .children(Root::render_sheet_layer(window, cx))
            .children(Root::render_notification_layer(window, cx))
    }
}
