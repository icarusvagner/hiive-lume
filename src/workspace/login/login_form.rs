use gpui::*;
use gpui_component::{
    Icon, IconName,
    button::{Button, ButtonVariants},
    form::form_field,
    input::{InputState, TextInput},
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
        }
    }
}

impl Render for LoginForm {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex_col()
            .flex()
            .gap(px(12.))
            .mb_4()
            // Username
            .child(
                form_field()
                    .col_span(2)
                    .label("Username")
                    .required(true)
                    .child(TextInput::new(&self.username).prefix(Icon::new(IconName::User))),
            )
            // Password
            .child(
                form_field()
                    .col_span(2)
                    .label("Password")
                    .required(true)
                    .child(
                        TextInput::new(&self.password)
                            .prefix(Icon::new(
                                Icon::empty().path("icons/custom/lock-outline.svg"),
                            ))
                            .mask_toggle(),
                    ),
            )
            // Submit button
            .child(
                Button::new("submit-btn")
                    .label("Login")
                    .primary()
                    .cursor_pointer()
                    .on_click(cx.listener(|this, _, window, cx| this.auth_login(window, cx))),
            )
    }
}
