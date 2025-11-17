use gpui::*;
use gpui_component::{
    Icon, IconName, Sizable, WindowExt,
    button::{Button, ButtonVariants},
    form::{field, v_form},
    input::{Input, InputState},
    notification::NotificationType,
};

use crate::states::auth_state::AuthState;

pub struct LoginForm {
    username: Entity<InputState>,
    password: Entity<InputState>,
}

impl LoginForm {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let username =
                cx.new(|cx| InputState::new(window, cx).placeholder("Enter admin username"));
            let password = cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("Enter admin password")
                    .masked(true)
            });

            Self { username, password }
        })
    }

    pub fn _clear(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let _ = self
            .username
            .update(cx, |this, cx| this.set_value("", window, cx));
        let _ = self
            .password
            .update(cx, |this, cx| this.set_value("", window, cx));

        cx.notify();
    }

    fn _validate_empty_input(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> bool {
        !self.username.read(cx).value().is_empty() && !self.password.read(cx).value().is_empty()
    }

    fn auth_login(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self._validate_empty_input(window, cx) {
            AuthState::login(
                self.username.read(cx).value().to_string(),
                self.password.read(cx).value().to_string(),
                cx,
            );
            self._clear(window, cx);
            cx.notify();
        } else {
            window.push_notification((NotificationType::Error, "Input fields are required"), cx);
        }
    }
}

impl Render for LoginForm {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_form()
            .large()
            .gap(px(12.))
            .mb_4()
            // Username
            .child(
                field().col_span(2).label("Username").required(true).child(
                    Input::new(&self.username)
                        .prefix(Icon::new(IconName::User))
                        .large(),
                ),
            )
            .mb_4()
            // Password
            .child(
                field().col_span(2).label("Password").required(true).child(
                    Input::new(&self.password)
                        .prefix(Icon::new(
                            Icon::empty().path("icons/custom/lock-outline.svg"),
                        ))
                        .mask_toggle()
                        .large(),
                ),
            )
            .mb_8()
            // Submit button
            .child(
                field().col_span(2).label_indent(false).child(
                    Button::new("submit-btn")
                        .mt_4()
                        .label("Login")
                        .primary()
                        .cursor_pointer()
                        .w_full()
                        .large()
                        .on_click(cx.listener(|this, _, window, cx| this.auth_login(window, cx))),
                ),
            )
    }
}
