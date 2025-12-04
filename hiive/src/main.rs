mod assets;
mod core;
mod data;
mod states;
mod themes;
mod window;
mod workspace;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tracing_subscriber::fmt()
		.with_target(false)
		.without_time()
		.with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
		.init();

	self::workspace::app::run()
}
