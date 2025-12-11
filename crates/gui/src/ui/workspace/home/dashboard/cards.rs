use gpui::*;
use gpui_component::{
	ActiveTheme, Icon, StyledExt, h_flex, label::Label, v_flex, white
};

pub struct DashboardCards;

impl DashboardCards {
	pub fn view(_window: &mut Window, cx: &mut App) -> Entity<Self> {
		cx.new(|_| Self)
	}

	fn first_card(&self, _window: &mut Window, cx: &mut Context<Self>) -> Div {
		v_flex()
			.p_4()
			.rounded_lg()
			.bg(cx.theme().blue_light)
			.items_center()
			.justify_center()
			.text_color(white())
			.child(
				Icon::empty()
					.path("icons/custom/users-more.svg")
					.size_12()
					.mb_2(),
			)
			.child(div().child("200/200").text_2xl().font_black())
			.child(Label::new("Total Employee").text_sm().font_medium())
	}

	fn second_card(&self, _window: &mut Window, cx: &mut Context<Self>) -> Div {
		v_flex()
			.p_4()
			.rounded_lg()
			.bg(cx.theme().red_light)
			.items_center()
			.justify_center()
			.text_color(white())
			.child(
				Icon::empty()
					.path("icons/custom/calendar-days.svg")
					.size_12()
					.mb_2(),
			)
			.child(div().child("195/200").text_2xl().font_black())
			.child(Label::new("Active Employees").text_sm().font_medium())
	}

	fn third_card(&self, _window: &mut Window, cx: &mut Context<Self>) -> Div {
		v_flex()
			.p_4()
			.rounded_lg()
			.bg(cx.theme().green_light)
			.items_center()
			.justify_center()
			.text_color(white())
			.child(
				Icon::empty()
					.path("icons/custom/hand-coins.svg")
					.size_12()
					.mb_2(),
			)
			.child(div().child("189/200").text_2xl().font_black())
			.child(Label::new("Total Success Payroll").text_sm().font_medium())
	}

	fn fourth_card(&self, _window: &mut Window, cx: &mut Context<Self>) -> Div {
		v_flex()
			.p_4()
			.rounded_lg()
			.bg(cx.theme().info)
			.items_center()
			.justify_center()
			.text_color(white())
			.child(
				Icon::empty()
					.path("icons/custom/credit-card.svg")
					.size_12()
					.mb_2(),
			)
			.child(
				h_flex()
					.items_center()
					.gap(px(0.50))
					.child(
						Icon::empty().path("icons/custom/philippine-peso.svg"),
					)
					.child(Label::new("123,450.00").text_lg().font_black()),
			)
			.child(Label::new("Last Month").text_sm().font_medium())
	}

	fn fifth_card(&self, _window: &mut Window, cx: &mut Context<Self>) -> Div {
		v_flex()
			.p_4()
			.rounded_lg()
			.bg(cx.theme().magenta)
			.items_center()
			.justify_center()
			.text_color(white())
			.child(
				Icon::empty()
					.path("icons/custom/sales-amount.svg")
					.size_12()
					.mb_2(),
			)
			.child(
				h_flex()
					.items_center()
					.gap(px(0.50))
					.child(
						Icon::empty().path("icons/custom/philippine-peso.svg"),
					)
					.child(Label::new("12,123,450.00").text_lg().font_black()),
			)
			.child(Label::new("All-Time Sales").text_sm().font_medium())
	}
}

impl Render for DashboardCards {
	fn render(
		&mut self,
		window: &mut Window,
		cx: &mut Context<Self>,
	) -> impl IntoElement {
		div()
			.grid()
			.grid_cols(5)
			.gap_5()
			// first card
			.child(self.first_card(window, cx))
			// second card
			.child(self.second_card(window, cx))
			// third card
			.child(self.third_card(window, cx))
			// fourth card
			.child(self.fourth_card(window, cx))
			// fifth card
			.child(self.fifth_card(window, cx))
	}
}
