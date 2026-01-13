use lib_core::{ctx::Ctx, model::ModelManager};

use crate::error::{CtxError, CtxResult, CtxW};

pub async fn ctx_resolve(user_id: i64) -> CtxResult {
	Ctx::new(user_id)
		.map(CtxW)
		.map_err(|err| CtxError::CtxCreateFail(err.to_string()))
}
