use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(From, Debug)]
pub enum Error {
	#[from]
	DieselError(diesel::result::Error),
}
