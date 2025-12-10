use gpui::*;
use gpui_component::{ActiveTheme, Icon, IconName, h_flex, label::Label, red_400};

const VERSION: &str = env!("CARGO_PKG_VERSION");
pub struct FooterComponent;

impl FooterComponent {
    pub fn view(_window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|_| FooterComponent)
    }
}

impl Render for FooterComponent {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let curated_by = h_flex()
            .items_center()
            .justify_start()
            .gap_1()
            .text_xs()
            .opacity(0.6)
            .child(Label::new("Devixion Team"))
            .child(Icon::new(
                Icon::empty().path("icons/custom/code-block-tags.svg"),
            ));

        let version = h_flex()
            .items_center()
            .justify_end()
            .text_xs()
            .opacity(0.6)
            .child(format!("v{VERSION}"))
            .child(Icon::new(IconName::Heart).text_color(red_400()));

        h_flex()
            .bg(cx.theme().background.opacity(0.50))
            .border_t_1()
            .border_color(cx.theme().foreground.opacity(0.20))
            .w_full()
            .px_1()
            .items_center()
            .justify_between()
            .child(curated_by)
            .child(version)
    }
}
