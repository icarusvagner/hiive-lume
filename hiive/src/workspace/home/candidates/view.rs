use gpui::*;
use gpui_component::{
	ActiveTheme, Icon, IconName, IndexPath, Selectable, Sizable, StyledExt, WindowExt, button::{Button, ButtonCustomVariant, ButtonGroup, ButtonVariants}, scroll::ScrollableElement, select::{Select, SelectState}, v_flex, white
};

use crate::{
	core::types::candidates_status::CandidateStatusType, data::home::interviews_data::InterviewsData, states::candidate_switch_state::{ActiveSwitchModeState, SwitchMode}, workspace::home::candidates::{
		card_mode::CandidateCardModeView, table_mode::TableModeView
	}
};

pub struct Candidates {
	timeframe_state: Entity<SelectState<Vec<String>>>,
	position_state: Entity<SelectState<Vec<String>>>,
	status_state: Entity<SelectState<Vec<&'static str>>>,
	card_mode: Entity<CandidateCardModeView>,
	table_mode: Entity<TableModeView>,
	switch_mode: SwitchMode,
}

impl Candidates {
	pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
		let card_mode = CandidateCardModeView::view(window, cx);
		let table_mode = TableModeView::view(window, cx);

		let data = InterviewsData::data();
		let mut positions =
			data.iter().map(|c| c.position()).collect::<Vec<String>>();
		positions.dedup();
		let timeframe_state = cx.new(|cx| {
			SelectState::new(
				vec!["New".to_string(), "Old".to_string()],
				Some(IndexPath::default()),
				window,
				cx,
			)
		});
		let position_state = cx.new(|cx| {
			SelectState::new(positions, Some(IndexPath::default()), window, cx)
		});
		let status_state = cx.new(|cx| {
			SelectState::new(
				CandidateStatusType::ALL_STR.to_vec(),
				Some(IndexPath::default()),
				window,
				cx,
			)
		});

		Self {
			timeframe_state,
			position_state,
			status_state,
			card_mode,
			table_mode,
			switch_mode: SwitchMode::CardMode,
		}
	}

	pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
		cx.new(|cx| Self::new(window, cx))
	}

	fn render_top_content(
		&mut self,
		window: &mut Window,
		cx: &mut Context<Self>,
	) -> Div {
		let data = InterviewsData::data();
		let total_candidates = data.iter().count();

		v_flex()
			.px_10()
			.py_6()
			.bg(cx.theme().accent)
			.child(
				div()
					.flex()
					.justify_between()
					.items_center()
					.child(
						div()
							.flex()
							.w_full()
							.flex_col()
							.gap_1()
							.child(
								div()
									.child("Candidates")
									.text_size(AbsoluteLength::Pixels(px(55.0)))
									.font_bold(),
							)
							.child(
								div()
									.child(format!(
										"{} Total Candidates",
										total_candidates
									))
									.text_size(AbsoluteLength::Pixels(px(22.0)))
									.text_color(
										cx.theme()
											.accent_foreground
											.opacity(0.70),
									)
									.font_thin(),
							),
					)
					.child(
						Button::new("add-candidates-btn")
							.large()
							.p_3()
							.custom(
								ButtonCustomVariant::new(cx)
									.color(cx.theme().blue)
									.foreground(white())
									.border(cx.theme().blue)
									.hover(cx.theme().blue.opacity(0.80))
									.active(cx.theme().blue),
							)
							.rounded_full()
							.icon(IconName::Plus)
							.label("Add Candidate")
							.cursor_pointer()
							.on_click(|_, window, cx| {
								window.open_dialog(cx, |dialog, _, _| {
									dialog.title("Add candidate").alert()
								});
							}),
					),
			)
			.child(self.render_filters(window, cx))
	}

	fn toggle_switch_mode(
		&mut self,
		index: usize,
		_window: &mut Window,
		cx: &mut Context<Self>,
	) {
		let switch_state = cx.global_mut::<ActiveSwitchModeState>();
		switch_state.mode = self.switch_mode.clone();
		self.switch_mode = self.switch_mode.to_mode(index);
		cx.notify();
	}

	fn render_filters(
		&self,
		_window: &mut Window,
		cx: &mut Context<Self>,
	) -> Div {
		div()
			.mt_5()
			.flex()
			.items_center()
			.justify_between()
			.child(
				div()
					.flex()
					.items_start()
					.justify_start()
					.gap_2()
					.child(
						Select::new(&self.timeframe_state)
							.py_3()
							.menu_width(px(110.0))
							.w_24()
							.appearance(false)
							.rounded_full()
							.border_1()
							.border_color(cx.theme().foreground.opacity(0.40)),
					)
					.child(
						Select::new(&self.position_state)
							.py_3()
							.menu_width(px(225.0))
							.w(px(226.0))
							.appearance(false)
							.rounded_full()
							.border_1()
							.border_color(cx.theme().foreground.opacity(0.40)),
					)
					.child(
						Select::new(&self.status_state)
							.py_3()
							.menu_width(px(160.0))
							.w_40()
							.appearance(false)
							.rounded_full()
							.border_1()
							.border_color(cx.theme().foreground.opacity(0.40)),
					),
			)
			.child(
				ButtonGroup::new("candidate-toggle-group")
					.border_1()
					.rounded_full()
					.border_color(cx.theme().foreground)
					.gap_3()
					.flex()
					.items_center()
					.justify_end()
					.child(
						Button::new("card-type")
							.custom(
								ButtonCustomVariant::new(cx)
									.color(cx.theme().secondary)
									.foreground(cx.theme().secondary_foreground)
									.hover(cx.theme().secondary.opacity(0.30))
									.active(cx.theme().primary),
							)
							.p_5()
							.rounded_full()
							.cursor_pointer()
							.large()
							.icon(
								Icon::new(IconName::LayoutDashboard)
									.text_color(
										if self
											.switch_mode
											.eq(&SwitchMode::CardMode)
										{
											white()
										} else {
											cx.theme().foreground
										},
									),
							)
							.selected(
								self.switch_mode.eq(&SwitchMode::CardMode),
							),
					)
					.child(
						Button::new("table-type")
							.custom(
								ButtonCustomVariant::new(cx)
									.color(cx.theme().secondary)
									.foreground(cx.theme().secondary_foreground)
									.hover(cx.theme().secondary.opacity(0.30))
									.active(cx.theme().primary),
							)
							.p_5()
							.rounded_full()
							.cursor_pointer()
							.large()
							.icon(
								Icon::empty()
									.path("icons/custom/list-line.svg")
									.text_color(
										if self
											.switch_mode
											.eq(&SwitchMode::TableMode)
										{
											white()
										} else {
											cx.theme().foreground
										},
									),
							)
							.selected(
								self.switch_mode.eq(&SwitchMode::TableMode),
							),
					)
					.on_click(cx.listener(
						|this, clicks: &Vec<usize>, window, cx| {
							this.toggle_switch_mode(clicks[0], window, cx)
						},
					)),
			)
	}

	fn render_card_mode(&mut self, _cx: &mut Context<Self>) -> Stateful<Div> {
		let content = v_flex()
			.id("render-card-mode-candidate")
			.px_10()
			.py_6()
			.child(self.card_mode.clone());

		content
	}

	fn render_table_mode(&mut self, _cx: &mut Context<Self>) -> Stateful<Div> {
		let content = v_flex()
			.id("render-table-mode-candidate")
			.px_10()
			.py_6()
			.size_full()
			.child(self.table_mode.clone());

		content
	}
}

impl Render for Candidates {
	fn render(
		&mut self,
		window: &mut Window,
		cx: &mut Context<Self>,
	) -> impl IntoElement {
		let mode_content = match self.switch_mode {
			SwitchMode::CardMode => self.render_card_mode(cx),
			SwitchMode::TableMode => self.render_table_mode(cx),
		};

		div()
			.size_full()
			.relative()
			.child(self.render_top_content(window, cx))
			.child(mode_content)
			.overflow_y_scrollbar()
	}
}
