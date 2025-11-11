use gpui::*;
use gpui_component::{ActiveTheme as _, Root, init, theme};

use crate::{
    themes::change_color_mode, window::get_window_options, workspace::workspace::Workspace,
};

mod assets;
mod data;
mod states;
mod themes;
mod window;
mod workspace;

fn main() {
    let app = Application::new().with_assets(assets::Assets);

    app.run(move |cx| {
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

        cx.spawn(async move |cx| {
            cx.open_window(window_options, |window, cx| {
                init(cx);
                theme::init(cx);
                change_color_mode(cx.theme().mode, window, cx);

                let workspace_view = Workspace::view(window, cx);
                cx.new(|cx| Root::new(workspace_view.into(), window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
