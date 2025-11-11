use gpui::{App, Global};

#[derive(Clone, PartialEq)]
pub enum ActiveLayout {
    Login,
    Home,
    Loading,
}

#[derive(Clone, PartialEq)]
pub struct LayoutState {
    pub layout: ActiveLayout,
}

impl Global for LayoutState {}

impl LayoutState {
    pub fn init(cx: &mut App) {
        let this = LayoutState {
            layout: ActiveLayout::Login,
        };

        cx.set_global(this);
    }
}
