use chrono::{Local, TimeZone};
use gpui::*;
use gpui_component::{
	ActiveTheme, Sizable, StyledExt, WindowExt, button::{Button, ButtonVariants}, date_picker::{DatePicker, DatePickerState}, form::{field, v_form}, h_flex, input::{Input, InputState}, label::Label, notification::NotificationType, select::{Select, SelectState}, v_flex
};

fn positions() -> Vec<String> {
	vec![
		"IT Specialist".to_string(),
		"Call Center Agent".to_string(),
		"Graphic Designer".to_string(),
	]
}

pub struct CreateEmployee {
	// General Info
	firstname: Entity<InputState>,
	middlename: Entity<InputState>,
	lastname: Entity<InputState>,
	date_of_birth: Entity<DatePickerState>,
	suffix: Entity<InputState>,
	position: Entity<SelectState<Vec<String>>>,

	// Contact Info
	email: Entity<InputState>,
	phone_number: Entity<InputState>,
	emergency_contact_fname: Entity<InputState>,
	emergency_contact_lname: Entity<InputState>,
	emergency_contact_number: Entity<InputState>,
}

impl CreateEmployee {
	pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
		cx.new(|cx| {
			let firstname = cx.new(|cx| {
				InputState::new(window, cx)
					.placeholder("Enter employee firstname")
					.clean_on_escape()
			});
			let middlename = cx.new(|cx| {
				InputState::new(window, cx)
					.placeholder("Enter employee middlename")
					.clean_on_escape()
			});
			let lastname = cx.new(|cx| {
				InputState::new(window, cx)
					.placeholder("Enter employee lastname")
					.clean_on_escape()
			});
			let date_of_birth = cx.new(|cx| {
				let mut picker = DatePickerState::new(window, cx);
				let early_2000 = Local
					.with_ymd_and_hms(2000, 1, 1, 0, 0, 0)
					.unwrap()
					.naive_local()
					.date();
				picker.set_date(early_2000, window, cx);

				picker
			});
			let suffix = cx.new(|cx| {
				InputState::new(window, cx)
					.placeholder("Enter suffix name")
					.clean_on_escape()
			});
			let position =
				cx.new(|cx| SelectState::new(positions(), None, window, cx));
			let email = cx.new(|cx| {
				InputState::new(window, cx)
					.placeholder("Enter contact email")
					.clean_on_escape()
			});
			let phone_number = cx.new(|cx| {
				InputState::new(window, cx)
					.placeholder("(+63)987 1234 567")
					.clean_on_escape()
			});
			let emergency_contact_fname = cx.new(|cx| {
				InputState::new(window, cx)
					.placeholder("Enter emergency contact firstname")
					.clean_on_escape()
			});
			let emergency_contact_lname = cx.new(|cx| {
				InputState::new(window, cx)
					.placeholder("Enter emergency contact lastname")
					.clean_on_escape()
			});
			let emergency_contact_number = cx.new(|cx| {
				InputState::new(window, cx)
					.placeholder("Enter emergency contact number")
					.clean_on_escape()
			});

			CreateEmployee {
				firstname,
				middlename,
				lastname,
				date_of_birth,
				suffix,
				position,

				email,
				phone_number,
				emergency_contact_fname,
				emergency_contact_lname,
				emergency_contact_number,
			}
		})
	}

	fn clear_input(&mut self, window: &mut Window, cx: &mut Context<Self>) {
		let early_2000 = Local
			.with_ymd_and_hms(2000, 1, 1, 0, 0, 0)
			.unwrap()
			.naive_local()
			.date();

		let _ = self
			.firstname
			.update(cx, |this, cx| this.set_value("", window, cx));
		let _ = self
			.middlename
			.update(cx, |this, cx| this.set_value("", window, cx));
		let _ =
			self.lastname.update(cx, |this, cx| this.set_value("", window, cx));
		let _ = self
			.date_of_birth
			.update(cx, |this, cx| this.set_date(early_2000, window, cx));
		let _ =
			self.suffix.update(cx, |this, cx| this.set_value("", window, cx));
		let _ =
			self.email.update(cx, |this, cx| this.set_value("", window, cx));
		let _ = self
			.phone_number
			.update(cx, |this, cx| this.set_value("", window, cx));
		let _ = self
			.emergency_contact_fname
			.update(cx, |this, cx| this.set_value("", window, cx));
		let _ = self
			.emergency_contact_lname
			.update(cx, |this, cx| this.set_value("", window, cx));
		let _ = self
			.emergency_contact_number
			.update(cx, |this, cx| this.set_value("", window, cx));

		window.push_notification(
			(NotificationType::Success, "Creat employee form cleared"),
			cx,
		);
		cx.notify();
	}

	fn render_header(&self, cx: &mut Context<Self>) -> Div {
		div()
			.flex()
			.px_10()
			.py_6()
			.bg(cx.theme().accent)
			.justify_between()
			.items_center()
			.child(
				div().flex().flex_col().gap_1().child(
					div()
						.child("Register Employee")
						.text_size(AbsoluteLength::Pixels(px(22.0)))
						.text_color(cx.theme().accent_foreground)
						.font_thin(),
				),
			)
	}

	fn render_form(&self, _window: &mut Window, cx: &mut Context<Self>) -> Div {
		div()
			.px_10()
			.py_6()
			.grid()
			.grid_cols(2)
			.gap_5()
			.child(
				v_flex()
					.gap_8()
					.p_10()
					.child(
						Label::new("General Information")
							.text_size(AbsoluteLength::Pixels(px(34.0)))
							.font_bold(),
					)
					.child(
						v_form()
							.child(
								field()
									.label("Position")
									.child(Select::new(&self.position).large()),
							)
							.child(
								field()
									.child(
										h_flex()
											.gap_8()
											.items_center()
											.w_full()
											.child(
												field()
													.label("Firstname")
													.required(true)
													.child(
														Input::new(
															&self.firstname,
														)
														.large(),
													),
											)
											.child(
												field()
													.label("Middlename")
													.required(true)
													.child(
														Input::new(
															&self.middlename,
														)
														.large(),
													),
											),
									)
									.child(
										h_flex()
											.gap_5()
											.items_center()
											.w_full()
											.child(
												field()
													.label("Lastname")
													.required(true)
													.child(
														Input::new(
															&self.lastname,
														)
														.large(),
													),
											)
											.child(
												field()
													.label("Suffix (Optional)")
													.child(
														Input::new(
															&self.suffix,
														)
														.large(),
													),
											),
									),
							)
							.child(field().label("Date of Birth").child(
								DatePicker::new(&self.date_of_birth).large(),
							)),
					),
			)
			.child(
				v_flex()
					.gap_8()
					.pt_10()
					.child(
						Label::new("Contact Details")
							.text_size(AbsoluteLength::Pixels(px(34.0)))
							.font_bold(),
					)
					.child(
						v_flex().gap_8().child(
							v_form()
								.child(
									field().child(
										h_flex()
											.gap_8()
											.items_center()
											.child(
												field()
													.label("Email Address")
													.required(true)
													.child(
														Input::new(&self.email)
															.large(),
													),
											)
											.child(
												field()
													.label("Phone Number")
													.required(true)
													.child(
														Input::new(
															&self.phone_number,
														)
														.large(),
													),
											),
									),
								)
								.child(
									field()
										.label("Emergency Contact")
										.label_indent(false)
										.child(
											v_flex()
												.gap_8()
												.items_center()
												.child(
													Input::new(
														&self
															.emergency_contact_fname,
													)
													.large(),
												)
												.child(
													Input::new(
														&self
															.emergency_contact_lname,
													)
													.large(),
												)
												.child(
													Input::new(
														&self
															.emergency_contact_number,
													)
													.large(),
												),
										),
								),
						),
					)
					.child(
						field().label_indent(false).child(
							h_flex()
								.gap_2()
								.items_center()
								.child(
									Button::new("submit-form-fields")
										.label("Create")
										.primary()
										.large()
										.cursor_pointer()
										.w(px(120.0)),
								)
								.child(
									Button::new("clear-form-fields")
										.label("Clear")
										.ghost()
										.cursor_pointer()
										.on_click(cx.listener(
											|this, _, window, cx| {
												this.clear_input(window, cx)
											},
										))
										.w(px(120.0)),
								),
						),
					),
			)
	}
}

impl Render for CreateEmployee {
	fn render(
		&mut self,
		window: &mut Window,
		cx: &mut Context<Self>,
	) -> impl IntoElement {
		div()
			.size_full()
			.gap_8()
			.child(self.render_header(cx))
			.child(self.render_form(window, cx))
	}
}
