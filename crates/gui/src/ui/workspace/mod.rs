mod ui;

use gpui::*;
use gpui_component::{
    Root, StyledExt, WindowExt,
    button::{Button, ButtonVariants},
    label::Label,
    v_flex,
};
use hiive_ui_components::HeaderComponent;

pub struct HiiveLume {
    header: Entity<HeaderComponent>,
}

impl HiiveLume {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let header = HeaderComponent::view(window, cx);

        Self { header }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Render for HiiveLume {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div()
            .bg(rgb(0xE0E0E0))
            .size_full()
            .child(self.header.clone())
            .child(
                v_flex()
                    .size_full()
                    .items_center()
                    .justify_center()
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
                    ),
            )
            .children(Root::render_dialog_layer(window, cx))
    }
}
