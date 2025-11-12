use gpui::*;
use gpui_component::{ActiveTheme, StyledExt, label::Label};

use crate::workspace::login::login_form::LoginForm;

pub struct RightPane {
    login_form: Entity<LoginForm>,
}

impl RightPane {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let login_form = LoginForm::view(window, cx);

        Self { login_form }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Render for RightPane {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .justify_center()
            .px(px(48.))
            .py(px(56.))
            .gap(px(20.))
            .w(px(480.))
            .child(
                div()
                    .child(Label::new("Admin Login").text_3xl().font_bold())
                    .child(
                        Label::new("Secure access to the admin dashboard")
                            .text_sm()
                            .text_color(cx.theme().colors.secondary_foreground)
                            .mt(px(4.)),
                    ),
            )
            .child(self.login_form.clone())
    }
}
