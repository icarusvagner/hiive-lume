use lib_auth::{pwd, token};
use lib_core::error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error, strum::AsRefStr)]
pub enum Error {
	#[error("{0} username not found")]
	LoginFailUsernameNotFound(String),
	#[error("{user_id:?} has no password")]
	LoginFailUserNoPassword { user_id: i64 },
	#[error("{user_id:?} username has no password")]
	LoginFailPasswordNotMatching { user_id: i64 },
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
}
