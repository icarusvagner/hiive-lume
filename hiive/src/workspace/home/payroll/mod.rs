use gpui::*;

pub struct Payroll;

impl Payroll {
	pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
		Self {}
	}

	pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
		cx.new(|cx| Self::new(window, cx))
	}
}

impl Render for Payroll {
	fn render(
		&mut self,
		_window: &mut Window,
		_cx: &mut Context<Self>,
	) -> impl IntoElement {
		div().size_full().child("This is the payroll")
	}
}
