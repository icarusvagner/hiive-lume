use gpui::rgb;
use gpui_component::{Icon, IconName};

use crate::data::models::dashboard_card_model::{DashboardCardModel, DashboardCardPieChartModel};

pub struct DashboardCardData;

#[derive(Clone)]
pub struct DashboardPieCard;

impl DashboardCardData {
    pub fn all_data() -> Vec<DashboardCardModel> {
        vec![
            DashboardCardModel::new(
                "Total Employees".to_string(),
                "200/200".to_string(),
                Icon::new(IconName::User),
            ),
            DashboardCardModel::new(
                "On Leave".to_string(),
                "12/200".to_string(),
                Icon::new(Icon::empty().path("icons/custom/document-outline.svg")),
            ),
            DashboardCardModel::new(
                "New Joinee".to_string(),
                "15/200".to_string(),
                Icon::new(Icon::empty().path("icons/custom/account-lock-outline.svg")),
            ),
            DashboardCardModel::new(
                "Happiness Rate".to_string(),
                "80%".to_string(),
                Icon::new(Icon::empty().path("icons/custom/emoticon-happy-outline.svg")),
            ),
        ]
    }
}

impl DashboardPieCard {
    pub fn data() -> Vec<DashboardCardPieChartModel> {
        vec![
            DashboardCardPieChartModel::new("Office".to_string(), 153.0, rgb(0xFB8500).into()),
            DashboardCardPieChartModel::new("Remote".to_string(), 46.0, rgb(0xE63946).into()),
        ]
    }
}
