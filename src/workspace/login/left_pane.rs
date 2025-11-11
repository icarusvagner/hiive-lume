use gpui::*;

pub struct LeftPane {
    pub src: SharedString,
}

impl LeftPane {
    pub fn new(src: String, _window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self { src: src.into() }
    }

    pub fn view(src: String, window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(src, window, cx))
    }
}

impl Render for LeftPane {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(530.))
            .h_full()
            .overflow_hidden()
            .child(img(self.src.clone()).w_full().object_fit(ObjectFit::Cover))
    }
}
