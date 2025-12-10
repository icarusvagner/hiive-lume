use gpui::{App, Global};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum HomeActiveView {
    Dashboard,
    Employees,
    Jobs,
    Candidates,
    Leaves,
    Settings,
    CreateEmployee,
    // Loading state
    Loading,
}

#[derive(Debug, PartialEq)]
pub struct HomeView {
    pub home: HomeActiveView,
}

impl Global for HomeView {}

impl HomeView {
    pub fn init(cx: &mut App) {
        let this = HomeView {
            home: HomeActiveView::Dashboard,
        };

        cx.set_global(this);
    }
}
