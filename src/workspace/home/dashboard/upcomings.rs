use gpui::*;
use gpui_component::{ActiveTheme, StyledExt, h_flex, label::Label, v_flex};

pub struct UpcomingInterviews;

impl UpcomingInterviews {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn render_filter_button(&self, _window: &mut Window, cx: &mut Context<Self>) -> Div {
        h_flex()
            .gap_5()
            .child(
                div()
                    .px_3()
                    .py_1()
                    .rounded_full()
                    .child("Today")
                    .hover(|this| {
                        this.bg(cx.theme().primary.opacity(0.40))
                            .text_color(white())
                    })
                    .cursor_pointer(),
            )
            .child(
                div()
                    .px_3()
                    .py_1()
                    .rounded_full()
                    .child("Tomorrow")
                    .hover(|this| {
                        this.bg(cx.theme().primary.opacity(0.40))
                            .text_color(white())
                    })
                    .cursor_pointer(),
            )
    }
}

impl Render for UpcomingInterviews {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .rounded_xl()
            .border_1()
            .border_color(black().opacity(0.20))
            .bg(cx.theme().secondary)
            .p_4()
            .w_full()
            .h_full()
            .gap_8()
            .child(Label::new("Upcoming Interviews").text_lg().font_bold())
            .child(self.render_filter_button(window, cx))
    }
}
