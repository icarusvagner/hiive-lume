
use diesel::{
	PgConnection, r2d2::{ConnectionManager, Pool}
};

use crate::{
	core_config, error::{Error, Result}
};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn init_connection() -> Result<DbPool> {
	let manager = ConnectionManager::<PgConnection>::new(&core_config().DB_URL);

	Pool::builder()
		.max_size(if cfg!(test) {
			1
		} else {
			15
		})
		.build(manager)
		.map_err(|_| {
			Error::DieselError(diesel::result::Error::BrokenTransactionManager)
		})
}
