pub type Result<T> = core::result::Result<T, AuthError>;

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

	#[error("hmac fail new from slice")]
	HmacFailNewFromSlice,

	#[error("invalid format")]
	InvalidFormat,
	#[error("cannot decode identity")]
	CannotDecodeIdent,
	#[error("cannot decode expiration")]
	CannotDecodeExp,
	#[error("signature not matching")]
	SignatureNotMatching,
	#[error("exp not iso")]
	ExpNotIso,
	#[error("token expired")]
	Expired,
}
