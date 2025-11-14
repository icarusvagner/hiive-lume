use gpui::*;
use gpui_component::v_flex;

pub fn custom_card(child: impl IntoElement, bg: Hsla) -> Div {
    v_flex()
        .rounded_xl()
        .shadow_2xl()
        .border_1()
        .border_color(black().opacity(0.20))
        .bg(bg)
        .p_12()
        .w_full()
        .h_full()
        .child(child)
}
