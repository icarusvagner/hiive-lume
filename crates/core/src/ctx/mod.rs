// region:    --- Modules

mod error;

pub use self::error::{Error, Result};

// endregion: --- Modules

#[derive(Clone, Debug)]
pub struct Ctx {
	user_id: i64,

	/// Note: For the future ACS (Access Control System)
	conv_id: Option<i64>,
}

// Constructors.
impl Ctx {
	pub fn root_ctx() -> Self {
		Ctx { user_id: 0, conv_id: None }
	}

	pub fn new(user_id: i64) -> Result<Self> {
		if user_id == 0 {
			Err(Error::CtxCannotNewRootCtx)
		} else {
			Ok(Self { user_id, conv_id: None })
		}
	}
}

// Property Accessors.
impl Ctx {
	pub fn user_id(&self) -> i64 {
		self.user_id
	}
}
