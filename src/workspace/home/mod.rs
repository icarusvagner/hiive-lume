use gpui::*;
use gpui_component::button::Button;

pub struct HomeSpace {}

impl HomeSpace {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Render for HomeSpace {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_2()
            .child(
                div()
                    .items_center()
                    .justify_center()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child("Home space")
                    .child(
                        Button::new("btn-01")
                            .label("Click me")
                            .on_click(|_, _, _| println!("Home Button clicked")),
                    ),
            )
    }
}
