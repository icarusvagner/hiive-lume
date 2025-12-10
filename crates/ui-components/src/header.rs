use gpui::{prelude::FluentBuilder, *};
use gpui_component::{
	ActiveTheme, IconName, Sizable, StyledExt, ThemeMode, TitleBar, button::{Button, ButtonVariants}, h_flex, label::Label
};
use tools::themes::change_color_mode;

pub struct HeaderComponent;

impl HeaderComponent {
	pub fn view(_window: &mut Window, cx: &mut App) -> Entity<Self> {
		cx.new(|_| HeaderComponent)
	}

	fn change_mode(&mut self, window: &mut Window, cx: &mut Context<Self>) {
		let new_mode = if cx.theme().mode.is_dark() {
			ThemeMode::Light
		} else {
			ThemeMode::Dark
		};

		change_color_mode(new_mode, window, cx);
	}
}

impl Render for HeaderComponent {
	fn render(
		&mut self,
		_: &mut Window,
		cx: &mut Context<Self>,
	) -> impl IntoElement {
		let theme_toggle = Button::new("theme-toggler")
			.map(|this| {
				if cx.theme().mode.is_dark() {
					this.icon(IconName::Sun)
				} else {
					this.icon(IconName::Moon)
				}
			})
			.small()
			.ghost()
			.on_click(
				cx.listener(|this, _, window, cx| this.change_mode(window, cx)),
			)
			.tooltip("Toggle theme");

		let github_btn = Button::new("github-url")
			.icon(IconName::GitHub)
			.small()
			.ghost()
			.on_click(|_, _, cx| {
				cx.open_url("https://github.com/icarusvagner/hiive-lume.git")
			})
			.tooltip("Open github repo");

		TitleBar::new().bg(cx.theme().background.opacity(0.50)).child(
			h_flex()
				.w_full()
				.pr_2()
				.justify_between()
				.child(Label::new("MORPHIQ LUME").text_xs().font_medium())
				.child(
					div()
						.pr_1()
						.flex()
						.items_center()
						.child(theme_toggle)
						.child(github_btn),
				),
		)
	}
}
