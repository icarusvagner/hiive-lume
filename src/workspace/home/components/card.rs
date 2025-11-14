use gpui::*;

pub fn custom_card(child: impl IntoElement, bg: Hsla) -> Div {
    div()
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
