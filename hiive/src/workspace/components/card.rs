use gpui::*;
use gpui_component::ActiveTheme;

pub fn custom_card(
    child: impl IntoElement,
    _window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_1()
        .p_4()
        .rounded_xl()
        .shadow_lg()
        .bg(cx.theme().accent)
        .child(child)
}
