use std::sync::Arc;

use super::workspace::Workspace;
use crate::states::{auth_state, candidate_switch_state, home_layout, show_layout};
use crate::window::get_window_options;
use crate::workspace::global_actions::register_actions;
use crate::{assets, themes::change_color_mode};

use gpui::*;
use gpui_component::{ActiveTheme as _, Root, init, theme};
use reqwest_client::ReqwestClient;

pub fn run() -> anyhow::Result<()> {
    let app = Application::new().with_assets(assets::Assets);

    app.run(move |cx| {
        let http_client = ReqwestClient::user_agent("hiive-lume agent").unwrap();
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
                show_layout::LayoutState::init(cx);
                home_layout::HomeLayout::init(cx);
                auth_state::AuthState::init(cx);
                candidate_switch_state::ActiveSwitchModeState::init(cx);

                change_color_mode(cx.theme().mode, window, cx);

                let workspace_view = Workspace::view(window, cx);
                cx.new(|cx| Root::new(workspace_view, window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });

    Ok(())
}
