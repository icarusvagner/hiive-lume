use gpui::*;
use gpui_component::{
	ActiveTheme, Icon, IconName, WindowExt, button::{Button, ButtonCustomVariant, ButtonVariants}, h_flex, label::Label, v_flex
};

use crate::{
	core::types::gen_status::GeneralStatus, data::{jobs::JobsData, models::jobs_model::JobsModel}, workspace::home::jobs::jobs_cards::JobCardsView
};

pub struct Jobs {
	jobs_card: Entity<JobCardsView>,
	data: Vec<JobsModel>,
}

impl Jobs {
	pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
		let data = JobsData::data();
		let jobs_card = JobCardsView::view(data.clone(), window, cx);

		Self { jobs_card, data }
	}

	pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
		cx.new(|cx| Self::new(window, cx))
	}

	fn render_top_content(
		&self,
		_window: &mut Window,
		cx: &mut Context<Self>,
	) -> Div {
		let total_jobs = self
			.data
			.iter()
			.filter(|x| x.job_status().eq(&GeneralStatus::Active))
			.count();

		v_flex().px_10().py_6().bg(cx.theme().accent).child(
			h_flex()
				.justify_between()
				.child(
					v_flex()
						.gap_1()
						.child(
							Label::new("Jobs")
								.text_size(AbsoluteLength::Pixels(px(55.0)))
								.font_weight(FontWeight::BOLD),
						)
						.child(
							Label::new(format!(
								"{total_jobs} Total active jobs"
							))
							.text_lg()
							.font_weight(FontWeight::THIN),
						),
				)
				.child(
					Button::new("goto-open-add-job-modal")
						.custom(
							ButtonCustomVariant::new(cx)
								.color(cx.theme().blue)
								.foreground(white())
								.border(cx.theme().blue)
								.hover(cx.theme().blue.opacity(0.80))
								.active(cx.theme().blue),
						)
						.rounded_full()
						.icon(Icon::new(IconName::Plus))
						.label("Add New Job")
						.cursor_pointer()
						.on_click(cx.listener(|this, _, window, cx| {
							this.render_add_new_job(window, cx)
						})),
				),
		)
	}

	fn render_add_new_job(&self, window: &mut Window, cx: &mut Context<Self>) {
		window.open_dialog(cx, |dialog, _, _| {
			dialog.title("Add new Job").footer(|_, _, _, _| {
				vec![
					Button::new("job-confirm-btn")
						.primary()
						.py_3()
						.label("Add new Job")
						.cursor_pointer()
						.on_click(|_, window, cx| {
							window.close_dialog(cx);
						}),
					Button::new("job-cancel-btn")
						.py_3()
						.label("Cancel")
						.cursor_pointer()
						.on_click(|_, window, cx| {
							window.close_dialog(cx);
						}),
				]
			})
		});
	}

	fn render_cards_job(&self, _cx: &mut Context<Self>) -> Stateful<Div> {
		let content = div()
			.id("jobs_cards_render")
			.px_10()
			.py_6()
			.child(self.jobs_card.clone());

		content
	}
}

impl Render for Jobs {
	fn render(
		&mut self,
		window: &mut Window,
		cx: &mut Context<Self>,
	) -> impl IntoElement {
		v_flex()
			.size_full()
			.child(self.render_top_content(window, cx))
			.child(self.render_cards_job(cx))
	}
}
