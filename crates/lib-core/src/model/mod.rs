use crate::{
	error::{Error, Result}, store::{dbx::Dbx, new_db_pool}
};

mod base;

pub use base::*;

pub mod modql_utils;
pub mod user;
pub mod role;

#[derive(Clone)]
pub struct ModelManager {
	dbx: Dbx,
}

impl ModelManager {
	pub async fn new() -> Result<Self> {
		let db_pool = new_db_pool().await.map_err(|ex| {
			Error::CantCreateModelManagerProvider(ex.to_string())
		})?;
		let dbx = Dbx::new(db_pool, false)?;

		Ok(Self { dbx })
	}

	pub fn new_with_txn(&self) -> Result<ModelManager> {
		let dbx = Dbx::new(self.dbx.db().clone(), true)?;

		Ok(Self { dbx })
	}

	pub fn dbx(&self) -> &Dbx {
		&self.dbx
	}
}
