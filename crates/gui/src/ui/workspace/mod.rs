mod ui;

use gpui::*;
use gpui_component::{ActiveTheme, Root, Sizable, spinner::Spinner, v_flex};
use ui_components::{FooterComponent, HeaderComponent};

use crate::{
    states::main_layout::{ActiveView, ViewState},
    ui::workspace::ui::{home::Homeview, login::LoginView},
};

pub struct Workspace {
    header: Entity<HeaderComponent>,
    view: ActiveView,
    login: Entity<LoginView>,
    home: Entity<Homeview>,
    footer: Entity<FooterComponent>,
    _subscription: Vec<Subscription>,
}

impl Workspace {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let header = HeaderComponent::view(window, cx);
        let login = LoginView::view(window, cx);
        let home = Homeview::view(window, cx);
        let footer = FooterComponent::view(window, cx);

        let _subscription = vec![cx.observe_global::<ViewState>(move |this, cx| {
            this.view = cx.global::<ViewState>().view.clone();
            cx.notify();
        })];

        Self {
            header,
            view: ActiveView::Home,
            login,
            home,
            footer,
            _subscription,
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn render_login(&self, _cx: &mut Context<Self>) -> Stateful<Div> {
        v_flex()
            .id("login-view")
            .h_full()
            .w_full()
            .child(self.login.clone())
    }

    fn render_view(&self, _cx: &mut Context<Self>) -> Stateful<Div> {
        v_flex()
            .id("home-view")
            .h_full()
            .w_full()
            .child(self.home.clone())
    }

    fn render_loading(&self, cx: &mut Context<Self>) -> Stateful<Div> {
        v_flex()
            .id("loading-view")
            .items_center()
            .justify_center()
            .h_full()
            .w_full()
            .child(Spinner::new().color(cx.theme().blue).with_size(px(100.)))
    }

    fn render_content(&self, _window: &mut Window, cx: &mut Context<Self>) -> Stateful<Div> {
        match self.view {
            ActiveView::Login => self.render_login(cx),
            ActiveView::Home => self.render_view(cx),
            ActiveView::Loading => self.render_loading(cx),
        }
    }
}

impl Render for Workspace {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        v_flex()
            .bg(rgb(0xE0E0E0))
            .size_full()
            .child(self.header.clone())
            .child(self.render_content(window, cx))
            .child(self.footer.clone())
            .children(Root::render_dialog_layer(window, cx))
            .children(Root::render_notification_layer(window, cx))
            .children(Root::render_sheet_layer(window, cx))
    }
}
