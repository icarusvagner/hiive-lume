use gpui::*;
use gpui_component::{
	ActiveTheme, Icon, IconName, WindowExt, button::{Button, ButtonCustomVariant, ButtonVariants}, h_flex, label::Label, v_flex
};

pub struct Departments;

impl Departments {
	pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
		Self
	}

	pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
		cx.new(|cx| Self::new(window, cx))
	}

	fn render_new_department(
		&self,
		window: &mut Window,
		cx: &mut Context<Self>,
	) {
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

	fn render_top_content(
		&self,
		_window: &mut Window,
		cx: &mut Context<Self>,
	) -> Div {
		let total_department = 5;

		v_flex().px_10().py_6().bg(cx.theme().accent).child(
			h_flex()
				.justify_between()
				.child(
					v_flex()
						.gap_1()
						.child(
							Label::new("Departments")
								.text_size(AbsoluteLength::Pixels(px(55.0)))
								.font_weight(FontWeight::BOLD),
						)
						.child(
							Label::new(format!(
								"{total_department} Total Departments"
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
						.label("New Department")
						.cursor_pointer()
						.on_click(cx.listener(|this, _, window, cx| {
							this.render_new_department(window, cx)
						})),
				),
		)
	}

	fn render_departments(&self, _cx: &mut Context<Self>) -> Div {
		div().child("this is the departments")
	}
}

impl Render for Departments {
	fn render(
		&mut self,
		window: &mut Window,
		cx: &mut Context<Self>,
	) -> impl IntoElement {
		v_flex()
			.size_full()
			.child(self.render_top_content(window, cx))
			.child(self.render_departments(cx))
	}
}
