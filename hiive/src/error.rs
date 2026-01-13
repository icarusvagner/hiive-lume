use lib_auth::{pwd, token};
use lib_core::{ctx::Ctx, error};

pub type Result<T> = core::result::Result<T, Error>;
pub type CtxResult = core::result::Result<CtxW, CtxError>;

#[derive(Debug, thiserror::Error, strum::AsRefStr)]
pub enum Error {
	#[error("{0} username not found")]
	LoginFailUsernameNotFound(String),
	#[error("{0} has no password")]
	LoginFailUserNoPassword(String),
	#[error("{0} username has no password")]
	LoginFailPasswordNotMatching(String),
	#[error("Data store disconnected")]
	Disconnect(#[from] std::io::Error),
	#[error("Internal error something went wrong")]
	InternalError,
	#[error("{0}")]
	AnyhowError(#[from] anyhow::Error),
	#[error("{0}")]
	ModelError(#[from] error::Error),
	#[error("{0}")]
	PwdError(#[from] pwd::Error),
	#[error("{0}")]
	TokenError(#[from] token::Error),
	#[error("{0}")]
	ServiceError(String),
	#[error("{0}")]
	CtxError(#[from] self::CtxError),
}

#[derive(Debug, Clone)]
pub struct CtxW(pub Ctx);

#[derive(Clone, Debug, thiserror::Error, strum::AsRefStr)]
pub enum CtxError {
	#[error("Context user id is not in the request")]
	CtxNotInRequest,
	#[error("{0}")]
	CtxCreateFail(String),
}
