#[derive(Debug, thiserror::Error)]
pub enum AuthError {
	#[error("invalid credentials")]
	InvalidCredentials,

	#[error("network error: {0}")]
	Network(String),

	#[error("token error: {0}")]
	Token(String),

	#[error("unknown auth error")]
	Unknown,
}
