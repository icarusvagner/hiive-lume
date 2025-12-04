use std::time::Duration;

use gpui::*;
use gpui_component::{
	ActiveTheme, Icon, IconName, Placement, StyledExt, WindowExt, avatar::Avatar, button::{Button, ButtonCustomVariant, ButtonVariants}, label::Label, menu::DropdownMenu, v_flex
};

use crate::{
	data::home::header_menu, states::home_layout::{HomeActiveLayout, HomeLayout}, workspace::global_actions::{LogoutAction, ProfileAction, SettingsAction}
};

pub struct HomeHeader {}

impl HomeHeader {
	pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
		Self {}
	}

	pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
		cx.new(|cx| Self::new(window, cx))
	}

	fn navigate_home_content(&self, layout: HomeActiveLayout, cx: &mut App) {
		let _ = cx.update_global::<HomeLayout, _>(|state, _| {
			if !state.home.eq(&layout) {
				state.home = HomeActiveLayout::Loading;
			}
		});

		cx.spawn(async move |cx| {
			cx.background_executor().timer(Duration::from_millis(500)).await;

			let _ = cx.update_global::<HomeLayout, _>(|state, _| {
				state.home = layout;
			});
		})
		.detach();
	}

	fn show_drawer(&self, window: &mut Window, cx: &mut Context<Self>) {
		window.open_sheet_at(Placement::Left, cx, |sheet, _, _| {
			sheet
				.title(
					div()
						.flex()
						.gap_3()
						.items_center()
						.child(img("images/hiive-logo.png").size(px(30.)))
						.child(Label::new("Hiive Lume").font_bold().text_lg()),
				)
				.child(v_flex().gap_2().mt_3().children(vec![
					Button::new("btn-01").label("Button 01"),
					Button::new("btn-02").label("Button 02"),
					Button::new("btn-03").label("Button 03"),
					Button::new("btn-04").label("Button 04"),
				]))
		});
	}

	fn menu_buttons(&self, cx: &mut Context<Self>) -> Vec<Button> {
		let mut buttons = Vec::new();

		for (indx, btn) in
			header_menu::HeaderMenu::all_data().iter().enumerate()
		{
			let layout = btn.goto_layout;

			let button = Button::new(indx)
				.custom(
					ButtonCustomVariant::new(cx)
						.color(cx.theme().background.opacity(0.20))
						.foreground(cx.theme().foreground)
						.hover(cx.theme().background)
						.active(cx.theme().background),
				)
				.label(btn.label.clone())
				.icon(btn.icon.clone())
				.on_click(cx.listener(move |this, _, _, cx| {
					this.navigate_home_content(layout, cx);
				}));

			buttons.push(button);
		}

		buttons
	}

	fn menu_profile(&self, cx: &mut Context<Self>) -> Div {
		div()
			.flex()
			.items_center()
			.gap_2()
			.py(px(2.5))
			.px(px(5.0))
			.rounded_full()
			.bg(cx.theme().secondary)
			.child(
				Avatar::new()
					.size(px(35.))
					.placeholder(
						Icon::new(IconName::CircleUser).text_color(white()),
					)
					.bg(cx.theme().primary),
			)
			.child(
				Button::new("header-menu-btn")
					.cursor_pointer()
					.label("John Doe")
					.dropdown_menu(|menu, _window, _cx| {
						menu.menu("Profile", Box::new(ProfileAction))
							.menu("Settings", Box::new(SettingsAction))
							.separator()
							.menu("Logout", Box::new(LogoutAction))
					}),
			)
	}
}

impl Render for HomeHeader {
	fn render(
		&mut self,
		_window: &mut Window,
		cx: &mut Context<Self>,
	) -> impl IntoElement {
		div()
			.bg(cx.theme().accent)
			.py(px(4.5))
			.px(px(5.0))
			.flex()
			.items_center()
			.justify_between()
			.child(
				div()
					.flex()
					.items_center()
					.gap_4()
					.child(
						Button::new("dashboard-btn-menu")
							.ghost()
							.cursor_pointer()
							.child(
								div()
									.flex()
									.gap_3()
									.items_center()
									.child(
										img("images/hiive-logo.png")
											.size(px(30.)),
									)
									.child(
										Label::new("Hiive Lume")
											.font_bold()
											.text_lg(),
									),
							)
							.on_click(cx.listener(
								move |this, _, window, cx| {
									this.show_drawer(window, cx)
								},
							)),
					)
					.child(div().w(px(20.)))
					.child(
						div().flex().gap_2().children(self.menu_buttons(cx)),
					),
			)
			.child(
				div()
					.flex()
					.items_center()
					.gap_2()
					.child(
						Button::new("btn-search")
							.icon(IconName::Search)
							.rounded_full()
							.ghost(),
					)
					.child(
						Button::new("btn-notif")
							.icon(IconName::Bell)
							.rounded_full()
							.ghost(),
					)
					.child(self.menu_profile(cx)),
			)
	}
}
