use gpui::App;

pub mod main_layout;
pub mod view_layout;

pub fn init(cx: &mut App) {
    main_layout::ViewState::init(cx);
    view_layout::HomeView::init(cx);
}
