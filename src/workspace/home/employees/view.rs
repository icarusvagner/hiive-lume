use gpui::*;
use gpui_component::{ActiveTheme, v_flex};

pub struct Employees {}

impl Employees {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Render for Employees {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().size_full().children((0..50).map(|i| {
            div()
                .h(px(40.))
                .w_full()
                .bg(cx.theme().secondary)
                .child(format!("Employee Item {}", i))
        }))
    }
}
