use gpui::*;
use gpui_component::v_flex;

use crate::data::models::jobs_model::JobsModel;

pub struct JobInfo {
    job: JobsModel,
}

impl JobInfo {
    pub fn view(job: JobsModel, _window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|_| Self { job })
    }
}

impl Render for JobInfo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .p_5()
            .child(self.job.title())
            .child(self.job.description())
    }
}
