use gpui::*;
use gpui_component::{
    ActiveTheme, Icon, IconName, Sizable, StyledExt,
    button::{Button, ButtonVariants},
    input::{InputState, TextInput},
    label::Label,
    white,
};

use crate::states::show_layout::{ActiveLayout, LayoutState};

pub struct RightPane {
    pub username: Entity<InputState>,
    pub password: Entity<InputState>,
}

impl RightPane {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let uname_input =
            cx.new(|cx| InputState::new(window, cx).placeholder("Enter admin username"));
        let pass_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Enter admin password")
                .masked(true)
        });

        Self {
            username: uname_input,
            password: pass_input,
        }
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
            .child(
                div()
                    .flex_col()
                    .flex()
                    .gap(px(6.))
                    .mb_4()
                    .child(Label::new("Username").font_medium())
                    // Username
                    .child(
                        TextInput::new(&self.username.clone())
                            .prefix(Icon::new(IconName::User))
                            .border_1()
                            .border_color(cx.theme().colors.accent)
                            .rounded_lg()
                            .large()
                            .text_lg()
                            .text_color(rgb(0x374151)),
                    )
                    // Password
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(6.))
                            .mb_4()
                            .child(Label::new("Password").font_medium())
                            .child(
                                TextInput::new(&self.password.clone())
                                    .prefix(
                                        Icon::new(Icon::empty())
                                            .path("icons/custom/lock-outline.svg"),
                                    )
                                    .border_1()
                                    .text_lg()
                                    .border_color(cx.theme().colors.accent)
                                    .rounded_lg()
                                    .mask_toggle()
                                    .large()
                                    .text_color(rgb(0x374151)),
                            ),
                    )
                    // Submit button
                    .child(
                        Button::new("submit-btn")
                            .label("Login")
                            .primary()
                            .rounded_lg()
                            .cursor_pointer()
                            .large()
                            .text_color(white())
                            .font_medium()
                            .shadow_md()
                            .on_click(cx.listener(|_, _, _, cx| {
                                let state = cx.global_mut::<LayoutState>();
                                state.layout = ActiveLayout::Home;
                                cx.notify();
                            })),
                    ),
            )
    }
}
