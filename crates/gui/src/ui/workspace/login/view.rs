use gpui::*;
use gpui_component::{
    Disableable, Icon, IconName, Sizable, StyledExt, WindowExt,
    button::{Button, ButtonVariants},
    form::{field, v_form},
    input::{Input, InputState},
    label::Label,
    notification::NotificationType,
    v_flex,
};

use crate::states::main_layout::{ActiveView, ViewState};

pub struct LoginView {
    username: Entity<InputState>,
    password: Entity<InputState>,
    is_loading: bool,
}

impl LoginView {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let username = cx.new(|cx| InputState::new(window, cx).placeholder("admin1234"));
            let password = cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("********")
                    .masked(true)
            });

            Self {
                username,
                password,
                is_loading: false,
            }
        })
    }

    fn _clear_inputs(&self, window: &mut Window, cx: &mut Context<Self>) {
        let _ = self
            .username
            .update(cx, |this, cx| this.set_value("", window, cx));
        let _ = self
            .password
            .update(cx, |this, cx| this.set_value("", window, cx));
        cx.notify();
    }

    fn _validate_empty_input(&self, _window: &mut Window, cx: &mut Context<Self>) -> bool {
        !self.username.read(cx).value().is_empty() && !self.password.read(cx).value().is_empty()
    }

    fn change_view(&self, cx: &mut Context<Self>) {
        let _ = cx.update_global::<ViewState, _>(|state, _| {
            state.view = ActiveView::Loading;
        });
        cx.spawn(async move |_, cx| {
            tokio::time::sleep(std::time::Duration::from_millis(1_200)).await;
            let _ = cx.update_global::<ViewState, _>(|state, _| state.view = ActiveView::Home);
        })
        .detach();
    }

    fn _auth_login(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.is_loading = true;

        if self._validate_empty_input(window, cx) {
            cx.spawn(async move |view, cx| {
                tokio::time::sleep(std::time::Duration::from_millis(1_200)).await;
                let _ = view.update(cx, |this, cx| {
                    this.is_loading = false;
                    this.change_view(cx);
                });
            })
            .detach();
            cx.notify();
        } else {
            self.is_loading = false;
            window.push_notification((NotificationType::Warning, "Input fields are required"), cx);
        }
    }

    fn _form_button(&self, _window: &mut Window, cx: &mut Context<Self>) -> Button {
        let mut form_btn = Button::new("login-submit-btn")
            .mt_4()
            .primary()
            .w_full()
            .large()
            .py_5()
            .loading(self.is_loading);

        if self.is_loading {
            form_btn = form_btn
                .disabled(self.is_loading)
                .cursor_not_allowed()
                .label("Loading...");
        } else {
            form_btn = form_btn.cursor_pointer().label("Sign-In");
        }

        form_btn.on_click(cx.listener(|this, _, window, cx| this._auth_login(window, cx)))
    }
}

impl Render for LoginView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .h_full()
            .w_full()
            .items_center()
            .justify_center()
            .p_5()
            .child(
                div().w(px(420.)).child(
                    v_form()
                        .large()
                        .gap_2()
                        .child(
                            field()
                                .col_span(2)
                                .label_indent(false)
                                .child(Label::new("Admin Login").text_3xl().font_semibold()),
                        )
                        .child(
                            field().col_span(2).label("Username").required(true).child(
                                Input::new(&self.username)
                                    .prefix(Icon::new(IconName::User))
                                    .py_5(),
                            ),
                        )
                        .child(
                            field().col_span(2).label("Password").required(true).child(
                                Input::new(&self.password)
                                    .prefix(Icon::new(
                                        Icon::empty().path("icons/custom/lock-outline.svg"),
                                    ))
                                    .mask_toggle()
                                    .py_5(),
                            ),
                        )
                        .child(
                            field()
                                .col_span(2)
                                .label_indent(false)
                                .child(self._form_button(window, cx)),
                        ),
                ),
            )
    }
}
