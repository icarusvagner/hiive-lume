use gpui::{App, Global};

#[derive(Clone, PartialEq)]
pub enum ActiveView {
    Login,
    Home,
    Loading,
}

#[derive(Clone, PartialEq)]
pub struct ViewState {
    pub view: ActiveView,
}

impl Global for ViewState {}

impl ViewState {
    pub fn init(cx: &mut App) {
        let this = ViewState {
            view: ActiveView::Login,
        };

        cx.set_global(this);
    }
}
