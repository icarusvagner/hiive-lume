use lib_models::authentication::user_account::UserAccount;

#[derive(Debug, Clone)]
pub struct LoginRequestResult {
	pub authenticated: bool,
	pub user: Option<UserAccount>,
}
