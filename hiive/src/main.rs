mod assets;
mod core;
mod data;
mod error;
mod states;
mod themes;
mod window;
mod workspace;

pub use self::error::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {
	tracing_subscriber::fmt()
		.with_target(false)
		.without_time()
		.with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
		.init();

	self::workspace::app::run().await
}
