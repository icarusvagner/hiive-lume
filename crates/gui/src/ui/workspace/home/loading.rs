use gpui::*;
use gpui_component::{ActiveTheme, Sizable, spinner::Spinner, v_flex};

pub struct LoadingView;

impl LoadingView {
	pub fn view(_window: &mut Window, cx: &mut App) -> Entity<Self> {
		cx.new(|_| Self)
	}
}

impl Render for LoadingView {
	fn render(
		&mut self,
		_window: &mut Window,
		cx: &mut Context<Self>,
	) -> impl IntoElement {
		v_flex()
			.flex_1()
			.flex_shrink_0()
			.h_full()
			.w_full()
			.items_center()
			.justify_center()
			.child(div().my_auto().child(
				Spinner::new().color(cx.theme().primary).with_size(px(100.)),
			))
	}
}
