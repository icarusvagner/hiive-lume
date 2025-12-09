mod ui;

use gpui::*;
use gpui_component::{
    Root, StyledExt, WindowExt,
    button::{Button, ButtonVariants},
    label::Label,
    v_flex,
};
use hiive_ui_components::HeaderComponent;

use crate::ui::workspace::ui::login::LoginView;

pub struct HiiveLume {
    header: Entity<HeaderComponent>,
    view: Entity<LoginView>,
}

impl HiiveLume {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let header = HeaderComponent::view(window, cx);
        let view = LoginView::view(window, cx);

        Self { header, view }
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
            .child(self.view.clone())
            .children(Root::render_dialog_layer(window, cx))
    }
}
