use gpui::*;
use gpui_component::input::InputState;

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
        div()
    }
}
