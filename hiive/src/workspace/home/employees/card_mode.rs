use gpui::*;
use gpui_component::{
	ActiveTheme, Icon, IconName, button::{Button, ButtonVariants}, h_flex, label::Label, menu::DropdownMenu, v_flex
};

use crate::{
	data::employees::employees_data::EmployeesData, workspace::{
		components::card::custom_card, global_actions::{ShowEmployee, UpdateEmployee}
	}
};

pub struct EmployeeCardMode;

impl EmployeeCardMode {
	pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
		Self {}
	}

	pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
		cx.new(|cx| Self::new(window, cx))
	}

	fn render_cards(
		&self,
		window: &mut Window,
		cx: &mut Context<Self>,
	) -> Vec<Div> {
		let data = EmployeesData::data()[0..10].to_vec();
		let mut cards = Vec::new();

		for (index, item) in data.iter().enumerate() {
			let content = v_flex().gap_1().child(
				div()
					.relative()
					.child(
						div().absolute().right_0().top_0().child(
							Button::new(SharedString::new(format!(
								"btn-card-candidate-{index}"
							)))
							.compact()
							.ghost()
							.rounded_full()
							.icon(
								Icon::new(IconName::EllipsisVertical)
									.text_color(cx.theme().foreground),
							)
							.dropdown_menu(|menu, _, _| {
								menu.menu("Update", Box::new(UpdateEmployee))
									.menu("View", Box::new(ShowEmployee))
							}),
						),
					)
					.child(
						v_flex()
							.child(
								div()
									.flex()
									.items_center()
									.justify_center()
									.child(
										div().h_16().w_16().child(
											div()
												.overflow_hidden()
												.rounded_full()
												.relative()
												.child(
													img(item.src())
														.h_full()
														.w_full()
														.rounded_full()
														.object_fit(
															ObjectFit::Cover,
														),
												),
										),
									),
							)
							.child(
								Label::new(item.fullname())
									.text_lg()
									.font_weight(FontWeight::BOLD)
									.text_center(),
							)
							.child(
								Label::new(item.position())
									.text_sm()
									.font_weight(FontWeight::NORMAL)
									.text_center()
									.text_color(
										cx.theme().foreground.opacity(0.70),
									),
							)
							.child(
								v_flex()
									.mt_1()
									.justify_center()
									.items_center()
									.px_2()
									.py_1()
									.rounded_full()
									.bg(item.status().color().opacity(0.20))
									.child(
										Label::new(
											item.status()
												.as_str()
												.to_uppercase(),
										)
										.font_weight(FontWeight::BOLD)
										.text_center(),
									),
							)
							.child(
								v_flex()
									.mt_4()
									.bg(white().opacity(0.30))
									.rounded_lg()
									.p_3()
									.child(
										h_flex()
											.gap_2()
											.child(Icon::new(
												Icon::empty().path(
													"icons/custom/\
													 envelope-outline.svg",
												),
											))
											.child(
												Label::new(item.email())
													.text_sm(),
											),
									)
									.child(
										h_flex()
											.gap_2()
											.child(Icon::new(
												Icon::empty().path(
													"icons/custom/\
													 smartphone-outline.svg",
												),
											))
											.child(
												Label::new(item.number())
													.text_sm(),
											),
									),
							)
							.child(
								v_flex()
									.mt_2()
									.bg(cx.theme().primary.opacity(0.10))
									.rounded_lg()
									.p_3()
									.child(
										h_flex()
											.gap_2()
											.justify_between()
											.child(
												Label::new("Department")
													.text_sm(),
											)
											.child(
												Label::new(
													item.department().as_str(),
												)
												.text_sm()
												.font_weight(FontWeight::BOLD),
											),
									)
									.child(
										h_flex()
											.gap_2()
											.justify_between()
											.child(
												Label::new("Date of Joining")
													.text_sm(),
											)
											.child(
												Label::new(item.date_joined())
													.text_sm()
													.font_weight(
														FontWeight::BOLD,
													),
											),
									),
							),
					),
			);

			let card = div().child(custom_card(content, window, cx));

			cards.push(card);
		}

		cards
	}
}

impl Render for EmployeeCardMode {
	fn render(
		&mut self,
		window: &mut Window,
		cx: &mut Context<Self>,
	) -> impl IntoElement {
		div()
			.grid()
			.grid_cols(5)
			.gap_5()
			.h_full()
			.children(self.render_cards(window, cx))
	}
}
