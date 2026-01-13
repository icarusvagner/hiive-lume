use std::sync::Arc;

use gpui::*;
use gpui_component::{ActiveTheme as _, Root, init, theme};
use reqwest_client::ReqwestClient;

use super::workspace::Workspace;
use crate::{
	Result, assets, states, themes::change_color_mode, window::get_window_options, workspace::global_actions::register_actions
};

pub async fn run() -> Result<()> {
	let app = Application::new().with_assets(assets::Assets);

	app.run(move |cx| {
		let http_client =
			ReqwestClient::user_agent("hiive-lume agent").unwrap();
		cx.set_http_client(Arc::new(http_client));

		cx.activate(true);
		cx.on_window_closed(|cx| {
			if cx.windows().len() == 0 {
				cx.quit();
			}
		})
		.detach();
		// This must be called before using any GPUI Component features.
		gpui_component::init(cx);
		let window_options = get_window_options(cx);

		register_actions(cx);

		cx.spawn(async move |cx| {
			cx.open_window(window_options, |window, cx| {
				init(cx);
				theme::init(cx);
				states::init(cx);

				change_color_mode(cx.theme().mode, window, cx);

				let workspace_view = Workspace::view(window, cx);
				cx.new(|cx| Root::new(workspace_view, window, cx))
			})?;

			tracing::debug!(
				"{:<12} - {}",
				"APP RUNNING",
				"no errors and successfully running"
			);

			Ok::<_, anyhow::Error>(())
		})
		.detach();
	});

	Ok(())
}
