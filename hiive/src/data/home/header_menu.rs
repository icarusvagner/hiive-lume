use gpui_component::{Icon, IconName};

use crate::{data::models::home_menu::HomeHeaderMenuModel, states::home_layout::HomeActiveLayout};

pub struct HeaderMenu;

impl HeaderMenu {
    pub fn all_data() -> Vec<HomeHeaderMenuModel> {
        vec![
            HomeHeaderMenuModel::new(
                "Dashboard".to_string(),
                Icon::new(IconName::LayoutDashboard),
                HomeActiveLayout::Dashboard,
            ),
            HomeHeaderMenuModel::new(
                "Employees".to_string(),
                Icon::new(IconName::User),
                HomeActiveLayout::Employees,
            ),
            HomeHeaderMenuModel::new(
                "Jobs".to_string(),
                Icon::new(Icon::empty().path("icons/custom/list-box-check-outline.svg")),
                HomeActiveLayout::Jobs,
            ),
            HomeHeaderMenuModel::new(
                "Candidates".to_string(),
                Icon::new(Icon::empty().path("icons/custom/users-round-outline.svg")),
                HomeActiveLayout::Candidates,
            ),
            HomeHeaderMenuModel::new(
                "Leaves".to_string(),
                Icon::new(Icon::empty().path("icons/custom/text-box-outline.svg")),
                HomeActiveLayout::Leaves,
            ),
        ]
    }
}
