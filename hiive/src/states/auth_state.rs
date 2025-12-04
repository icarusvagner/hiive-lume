use gpui::{App, Global};
use lib_models::authentication::user_account::UserAccount;

pub struct AuthState {
	pub authenticated: bool,
	pub user: Option<UserAccount>,
	pub count_fail: u32,
}

impl Global for AuthState {}

impl AuthState {}

impl AuthState {
	pub fn init(cx: &mut App) {
		let this =
			AuthState { authenticated: false, user: None, count_fail: 0 };

		cx.set_global(this);
	}
}
