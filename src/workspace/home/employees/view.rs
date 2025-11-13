use gpui::*;
use gpui_component::{ActiveTheme, StyledExt, v_flex};

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
        v_flex()
            .gap_2()
            .p_4()
            .children((0..50).map(|i| {
                div()
                    .w_full()
                    .h(px(80.))
                    .bg(cx.theme().accent)
                    .child(format!("Card {}", i))
            }))
            .scrollable(Axis::Vertical)
    }
}
