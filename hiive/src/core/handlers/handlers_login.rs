use lib_auth::pwd::{self, ContentToHash, SchemeStatus};
use lib_core::{
	ctx::Ctx, model::{ModelManager, user::UserAccountBmc}
};
use lib_models::authentication::user_account::UserAccountForLogin;

use crate::{
	Error, Result, core::types::request_res::{RequestError, RequestResult}
};

pub async fn handlers_login(
	mm: &ModelManager,
	payload: LoginPayload,
) -> Result<RequestResult> {
	let LoginPayload { username, password: pwd_clear } = payload;

	let root_ctx = Ctx::root_ctx();

	let user: UserAccountForLogin =
		UserAccountBmc::first_by_username(&root_ctx, &mm, &username)
			.await?
			.ok_or(Error::LoginFailUsernameNotFound(username.to_string()))?;
	let user_id = user.id;
	let Some(pwd) = user.password_hash else {
		return Err(Error::LoginFailUserNoPassword { user_id });
	};

	let scheme_status = pwd::validate_pwd(
		ContentToHash { salt: user.pass_salt, content: pwd_clear.clone() },
		pwd,
	)
	.await
	.map_err(|_| Error::LoginFailPasswordNotMatching { user_id })?;

	if let SchemeStatus::Outdated = scheme_status {
		tracing::debug!("password scheme outdated, upgrading.");
		UserAccountBmc::update_password(&root_ctx, &mm, user.id, &pwd_clear)
			.await?;
	}

	let result_req = RequestResult {
		message: "Login Successfully".to_string(),
		status: RequestError::Success,
	};

	Ok(result_req)
}

#[derive(Debug)]
pub struct LoginPayload {
	pub username: String,
	pub password: String,
}
