mod ui;

use gpui::*;
use gpui_component::{
    Root, StyledExt, WindowExt,
    button::{Button, ButtonVariants},
    label::Label,
};

pub struct HiiveLume;

impl Render for HiiveLume {
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
                    .primary()
                    .on_click(cx.listener(move |_, _, window, cx| {
                        window.open_dialog(cx, |dialog, _, _| {
                            dialog.title("You have clicked me!").confirm()
                        })
                    })),
            )
            .children(Root::render_dialog_layer(window, cx))
    }
}
