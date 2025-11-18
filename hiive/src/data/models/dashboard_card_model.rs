use gpui::Hsla;
use gpui_component::Icon;

#[derive(Clone)]
pub struct DashboardCardModel {
    label: String,
    content: String,
    icon: Icon,
}

impl DashboardCardModel {
    pub fn new(label: String, content: String, icon: Icon) -> Self {
        Self {
            label,
            content,
            icon,
        }
    }

    pub fn icon(&self) -> Icon {
        self.icon.clone()
    }

    pub fn content(&self) -> String {
        self.content.clone()
    }

    pub fn label(&self) -> String {
        self.label.clone()
    }
}

#[derive(Clone)]
pub struct DashboardCardPieChartModel {
    pub label: String,
    pub data: f64,
    pub color: Hsla,
}

impl DashboardCardPieChartModel {
    pub fn new(label: String, data: f64, color: Hsla) -> Self {
        Self { label, data, color }
    }
}
