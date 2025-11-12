use gpui::{App, Global};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum HomeActiveLayout {
    Dashboard,
    Employees,
    Jobs,
    Candidates,
    Leaves,
    Settings,
    CreateEmployee,
}

#[derive(Debug, PartialEq)]
pub struct HomeLayout {
    pub home: HomeActiveLayout,
}

impl Global for HomeLayout {}

impl HomeLayout {
    pub fn init(cx: &mut App) {
        let this = HomeLayout {
            home: HomeActiveLayout::Dashboard,
        };

        cx.set_global(this);
    }
}
