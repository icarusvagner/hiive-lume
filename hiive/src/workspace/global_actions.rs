use gpui::*;

use crate::states::{home_layout::HomeLayout, show_layout::LayoutState};

actions!(headermenu, [ProfileAction, SettingsAction, LogoutAction]);
actions!(candidatemenu, [ViewCandidate, RemoveCandidte]);
actions!(window, [QuitApp]);
actions!(employee, [UpdateEmployee, ShowEmployee]);

pub fn register_actions(cx: &mut App) {
    cx.on_action(profile_action);
    cx.on_action(settings_action);
    cx.on_action(logout_action);
    cx.on_action(view_candidate_action);
    cx.on_action(remove_candidate_action);
    cx.on_action(quit_app_action);
    cx.on_action(update_employee_action);
    cx.on_action(show_employee_action);

    register_key_bindings(cx);
}

fn update_employee_action(_: &UpdateEmployee, _cx: &mut App) {}
fn show_employee_action(_: &ShowEmployee, _cx: &mut App) {}

fn register_key_bindings(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("ctrl-q", QuitApp, None),
        KeyBinding::new("cmd-q", QuitApp, None),
    ]);
}

fn quit_app_action(_: &QuitApp, cx: &mut App) {
    cx.quit();
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

fn view_candidate_action(_: &ViewCandidate, _cx: &mut App) {}

fn remove_candidate_action(_: &RemoveCandidte, _cx: &mut App) {}
