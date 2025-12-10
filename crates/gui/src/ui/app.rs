use chrono::Utc;
use gpui::{AppContext, Application};
use gpui_component::{ActiveTheme, Root, theme};
use tools::themes::change_color_mode;

use crate::{
	assets, states, ui::{window::get_window_options, workspace::Workspace}
};

pub async fn run_app() -> anyhow::Result<()> {
	let app = Application::new().with_assets(assets::Assets);

	tracing::info!("{:<12} - {}", "APP Running", Utc::now().to_string());
	app.run(move |cx| {
		gpui_component::init(cx);

		let window_options = get_window_options(cx);

		cx.spawn(async move |cx| {
			cx.open_window(window_options, |window, cx| {
				theme::init(cx);
				states::init(cx);

				change_color_mode(cx.theme().mode, window, cx);

				let view = Workspace::view(window, cx);
				cx.new(|cx| Root::new(view, window, cx))
			})?;

			Ok::<_, anyhow::Error>(())
		})
		.detach();
	});
	Ok(())
}
