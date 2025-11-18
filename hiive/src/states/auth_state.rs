use std::time::Duration;

use gpui::{App, BorrowAppContext, Global};

use crate::states::show_layout::LayoutState;

pub struct AuthState {
    pub authenticated: bool,
    pub user_id: Option<i128>,
}

impl Global for AuthState {}

impl AuthState {}

impl AuthState {
    pub fn init(cx: &mut App) {
        let this = AuthState {
            authenticated: false,
            user_id: None,
        };

        cx.set_global(this);
    }

    pub fn login(_username: String, _password: String, cx: &mut App) {
        let _ = cx.update_global::<LayoutState, _>(|state, _| {
            state.layout = super::show_layout::ActiveLayout::Loading;
        });

        cx.spawn(async move |cx| {
            cx.background_executor()
                .timer(Duration::from_millis(1200))
                .await;

            let _ = cx.update_global::<LayoutState, _>(|state, _| {
                state.layout = super::show_layout::ActiveLayout::Home;
            });
            let _ = cx.update_global::<AuthState, _>(|state, _| {
                state.authenticated = true;
                state.user_id = Some(1_001);
            });
        })
        .detach();
    }
}
