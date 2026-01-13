use gpui::*;
use gpui_component::{
	ActiveTheme, Icon, IconName, Sizable, WindowExt, button::{Button, ButtonCustomVariant, ButtonVariants}, form::{field, v_form}, h_flex, input::{Input, InputState}, label::Label, notification::NotificationType, v_flex
};

use crate::{
	core::handlers::handlers_department::{
		DepartmentAddPayload, handlers_add_department
	}, states::db_state::ConnectionState
};

pub struct Departments {
	department_name: Entity<InputState>,
	department_address: Entity<InputState>,
	department_description: Entity<InputState>,
	loading: bool,
}

impl Departments {
	pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
		let department_name = cx.new(|cx| {
			InputState::new(window, cx).placeholder("Enter department name")
		});
		let department_address = cx.new(|cx| {
			InputState::new(window, cx).placeholder("Where it is located")
		});
		let department_description = cx.new(|cx| {
			InputState::new(window, cx)
				.placeholder("Short description")
				.auto_grow(10, 30)
		});

		Self {
			department_name,
			department_address,
			department_description,
			loading: false,
		}
	}

	pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
		cx.new(|cx| Self::new(window, cx))
	}

	fn validate_inputs(&self, window: &mut Window, cx: &mut App) -> bool {
		let d_name = self.department_name.read(cx).value();
		let d_addr = self.department_address.read(cx).value();
		let d_desc = self.department_description.read(cx).value();

		match (d_name.is_empty(), d_addr.is_empty(), d_desc.is_empty()) {
			(false, true, true) => {
				window.push_notification(
					(NotificationType::Warning, "Department name is required!"),
					cx,
				);
				false
			}
			(true, false, true) => {
				window.push_notification(
					(NotificationType::Warning, "Address is required!"),
					cx,
				);
				false
			}
			(true, true, false) => {
				window.push_notification(
					(NotificationType::Warning, "Description is required!"),
					cx,
				);
				false
			}
			(true, true, true) => true,
			(false, false, false) => {
				window.push_notification(
					(NotificationType::Error, "Fill in the required fields"),
					cx,
				);
				false
			}
			(_, _, _) => false,
		}
	}

	fn save_department(&mut self, window: &mut Window, cx: &mut Context<Self>) {
		if self.validate_inputs(window, cx) {
			return;
		}

		if let Some(mm_state) = cx.global::<ConnectionState>().mm.clone() {
			self.loading = true;
			cx.notify();

			#[rustfmt::skip]
			let payload = DepartmentAddPayload {
				name: self.department_name.read(cx).value().to_string(),
				full_address: self.department_address.read(cx).value().to_string(),
				description: self.department_description.read(cx).value().to_string(),
			};

			cx.spawn_in(window, async move |_this, cx| {
				let result =
					handlers_add_department(&mm_state, 0, payload).await;

				let _ = cx.update(|window, cx| match result {
					Ok(res) => {}
					Err(err) => {}
				});
			})
			.detach();
		}
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
							let department_name = this.department_name.clone();
							let department_address =
								this.department_address.clone();
							let department_desc =
								this.department_description.clone();
							let entity = cx.entity();

							window.open_dialog(cx, move |dialog, _, _| {
								let entity = entity.clone();

								dialog
									.title("Add Department")
									.child(
										v_form()
											.gap_6()
											.child(
												field()
													.label("Department Name")
													.child(Input::new(
														&department_name,
													)),
											)
											.child(
												field()
													.label("Full Address")
													.child(
														Input::new(
															&department_address,
														)
														.large(),
													),
											)
											.child(
												field()
													.label("Description")
													.child(
														Input::new(
															&department_desc,
														)
														.large(),
													),
											),
									)
									.confirm()
									.on_ok(move |_, window, cx| {
										cx.update_entity(
											&entity.clone(),
											|entity, cx| {
												entity.save_department(
													window, cx,
												);
												cx.notify();
											},
										);

										true
									})
							});
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
