use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
	CtxCannotNewRootCtx,
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(
			fmt,
			"{:?}",
			match self {
				Self::CtxCannotNewRootCtx =>
					"context cannot create new root context",
			}
		)
	}
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
