use gpui::*;

use crate::workspace::home::{content::HomeContent, header::HomeHeader};

mod content;
mod header;

pub struct HomeSpace {
    home_header: Entity<HomeHeader>,
    home_content: Entity<HomeContent>,
}

impl HomeSpace {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let home_header = HomeHeader::view(window, cx);
        let home_content = HomeContent::view(window, cx);

        Self {
            home_header,
            home_content,
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Render for HomeSpace {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .child(self.home_header.clone())
            .child(self.home_content.clone())
    }
}
