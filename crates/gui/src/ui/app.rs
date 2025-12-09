use gpui::{AppContext, Application};
use gpui_component::Root;

use crate::{
    assets,
    ui::{window::get_window_options, workspace::HelloWorld},
};

pub async fn run_app() -> anyhow::Result<()> {
    let app = Application::new().with_assets(assets::Assets);

    app.run(move |cx| {
        gpui_component::init(cx);

        let window_options = get_window_options(cx);

        cx.spawn(async move |cx| {
            cx.open_window(window_options, |window, cx| {
                let view = cx.new(|_| HelloWorld);

                cx.new(|cx| Root::new(view, window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
    Ok(())
}
