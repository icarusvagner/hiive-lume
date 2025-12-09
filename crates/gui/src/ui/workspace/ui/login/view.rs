use gpui::*;
use gpui_component::{
    Icon, IconName, Sizable,
    button::{Button, ButtonVariants},
    form::{field, v_form},
    input::{Input, InputState},
    v_flex,
};

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
}

impl Render for LoginView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .items_center()
            .justify_center()
            .p_5()
            .child(
                div().w_72().child(
                    v_form()
                        .large()
                        .gap_2()
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
                                    .py_5(),
                            ),
                        )
                        .child(
                            field().col_span(2).label_indent(false).child(
                                Button::new("login-submit-btn")
                                    .label("Sign-In")
                                    .mt_4()
                                    .primary()
                                    .cursor_pointer()
                                    .w_full()
                                    .large()
                                    .py_5()
                                    .on_click(|_, _, _| println!("Login button clicked")),
                            ),
                        ),
                ),
            )
    }
}
