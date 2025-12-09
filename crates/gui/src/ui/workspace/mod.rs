use gpui::*;
use gpui_component::{Root, StyledExt, WindowExt, button::Button, label::Label};

pub struct HelloWorld;

impl Render for HelloWorld {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_3()
            .child(Label::new("Hello world").text_xl().font_medium())
            .child(
                Button::new("text-btn")
                    .label("Click me")
                    .on_click(cx.listener(move |_, _, window, cx| {
                        window.open_dialog(cx, |dialog, _, _| {
                            dialog.title("You have clicked me!").confirm()
                        })
                    })),
            )
            .children(Root::render_dialog_layer(window, cx))
    }
}
