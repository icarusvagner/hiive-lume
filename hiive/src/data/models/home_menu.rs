use gpui_component::Icon;

use crate::states::home_layout::HomeActiveLayout;

pub struct HomeHeaderMenuModel {
    pub label: String,
    pub icon: Icon,
    pub goto_layout: HomeActiveLayout,
}

impl HomeHeaderMenuModel {
    pub fn new(label: String, icon: Icon, goto_layout: HomeActiveLayout) -> Self {
        Self {
            label,
            icon,
            goto_layout,
        }
    }
}
