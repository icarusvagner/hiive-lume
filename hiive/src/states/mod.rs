use gpui::App;

pub mod auth_state;
pub mod candidate_switch_state;
pub mod db_state;
pub mod home_layout;
pub mod show_layout;

pub fn init(cx: &mut App) {
	show_layout::LayoutState::init(cx);
	home_layout::HomeLayout::init(cx);
	auth_state::AuthState::init(cx);
	db_state::ConnectionState::init(cx);
	candidate_switch_state::ActiveSwitchModeState::init(cx);
}
