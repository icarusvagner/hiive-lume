use gpui::*;

use crate::states::{home_layout::HomeLayout, show_layout::LayoutState};

actions!(headermenu, [ProfileAction, SettingsAction, LogoutAction]);

pub fn register_actions(cx: &mut App) {
    cx.on_action(profile_action);
    cx.on_action(settings_action);
    cx.on_action(logout_action);
}

fn profile_action(_: &ProfileAction, cx: &mut App) {
    let state = cx.global_mut::<HomeLayout>();
    state.home = crate::states::home_layout::HomeActiveLayout::Dashboard;
}

fn settings_action(_: &SettingsAction, cx: &mut App) {
    let state = cx.global_mut::<HomeLayout>();
    state.home = crate::states::home_layout::HomeActiveLayout::Settings;
}

fn logout_action(_: &LogoutAction, cx: &mut App) {
    let state = cx.global_mut::<LayoutState>();
    state.layout = crate::states::show_layout::ActiveLayout::Login;
}
