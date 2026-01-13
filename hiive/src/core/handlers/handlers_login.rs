use lib_auth::pwd::{self, ContentToHash, SchemeStatus};
use lib_core::{
	ctx::Ctx, model::{ModelManager, user::UserAccountBmc}
};
use lib_models::{
	authentication::user_account::{UserAccount, UserAccountForLogin}, types::UserSessionState
};

use crate::{Error, Result, core::types::request_res::LoginRequestResult};

pub async fn handlers_login(
	mm: &ModelManager,
	payload: LoginPayload,
) -> Result<LoginRequestResult> {
	let LoginPayload { username, password: pwd_clear } = payload;

	let root_ctx = Ctx::root_ctx();

	let user: UserAccountForLogin =
		UserAccountBmc::first_by_username(&root_ctx, &mm, &username)
			.await?
			.ok_or(Error::LoginFailUsernameNotFound(username.to_string()))?;
	let Some(pwd) = user.password_hash else {
		return Err(Error::LoginFailUserNoPassword(
			username.clone().to_string(),
		));
	};

	let scheme_status = pwd::validate_pwd(
		ContentToHash { salt: user.pass_salt, content: pwd_clear.clone() },
		pwd,
	)
	.await
	.map_err(|_| {
		Error::LoginFailPasswordNotMatching(username.clone().to_string())
	})?;

	if let SchemeStatus::Outdated = scheme_status {
		tracing::debug!("password scheme outdated, upgrading.");
		UserAccountBmc::update_password(&root_ctx, &mm, user.id, &pwd_clear)
			.await?;
	}

	let auth_user = UserAccount {
		id: user.id,
		user_id: user.user_id,
		username: user.username,
		status: UserSessionState::Active,
	};

	let result_req =
		LoginRequestResult { authenticated: true, user: Some(auth_user) };

	Ok(result_req)
}

#[derive(Debug)]
pub struct LoginPayload {
	pub username: String,
	pub password: String,
}
