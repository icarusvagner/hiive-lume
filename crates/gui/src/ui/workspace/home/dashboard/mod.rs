mod action_queues;
mod cards;
mod charts;

use gpui::*;
use gpui_component::{StyledExt, label::Label, v_flex};

use crate::ui::workspace::home::dashboard::{
	action_queues::DashboardActionQueues, cards::DashboardCards
};

pub struct DashboardView {
	cards: Entity<DashboardCards>,
	action_queues: Entity<DashboardActionQueues>,
}

impl DashboardView {
	pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
		let cards = DashboardCards::view(window, cx);
		let action_queues = DashboardActionQueues::view(window, cx);

		Self { cards, action_queues }
	}

	pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
		cx.new(|cx| Self::new(window, cx))
	}

	fn render_cards(&self, _cx: &mut Context<Self>) -> Stateful<Div> {
		div().id("dashboard-cards").child(self.cards.clone())
	}

	fn render_action_queues(&self, _cx: &mut Context<Self>) -> Stateful<Div> {
		div()
			.id("dashboard-action-queues")
			.mt_4()
			.child(self.action_queues.clone())
	}

	fn render_content(&self, cx: &mut Context<Self>) -> Stateful<Div> {
		v_flex()
			.id("dashboard-contents")
			.flex_1()
			.flex_shrink_0()
			.relative()
			.overflow_y_scroll()
			.child(
				Label::new("Dashboard Overview").text_2xl().font_black().mb_5(),
			)
			.child(self.render_cards(cx))
			.child(self.render_action_queues(cx))
	}
}

impl Render for DashboardView {
	fn render(
		&mut self,
		_window: &mut Window,
		cx: &mut Context<Self>,
	) -> impl IntoElement {
		v_flex()
			.flex_1()
			.flex_shrink_0()
			.px_12()
			.py_4()
			.child(self.render_content(cx))
	}
}
