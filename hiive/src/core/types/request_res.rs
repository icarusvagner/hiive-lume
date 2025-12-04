#[derive(Clone, Debug)]
pub struct RequestResult {
	pub message: String,
	pub status: RequestError,
}

#[derive(Clone, Debug)]
pub enum RequestError {
	Success,
	Failed,
}
