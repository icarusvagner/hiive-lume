use gpui::*;
use gpui_component::{
	Icon, IconName, Sizable, StyledExt, button::{Button, ButtonVariants}, h_flex, label::Label, v_flex
};

mod table;

pub struct EmployeeView;

impl EmployeeView {
	pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
		Self {}
	}

	pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
		cx.new(|cx| Self::new(window, cx))
	}

	fn render_header(
		&self,
		_window: &mut Window,
		_cx: &mut Context<Self>,
	) -> Stateful<Div> {
		h_flex()
			.id("employees-header-menu")
			.items_center()
			.px_5()
			.py_2()
			.child(Label::new("Employees").text_2xl().font_bold())
			.child(div().mx_auto())
			.child(
				h_flex()
					.items_center()
					.gap_3()
					.child(
						Button::new("btn-export-emp")
							.small()
							.cursor_pointer()
							.py_3()
							.label("Export")
							.icon(
								Icon::empty()
									.path("icons/custom/hard-drive-upload.svg"),
							),
					)
					.child(
						Button::new("btn-import-emp")
							.cursor_pointer()
							.small()
							.py_3()
							.label("Import")
							.icon(
								Icon::empty().path(
									"icons/custom/hard-drive-download.svg",
								),
							),
					)
					.child(
						Button::new("bulk-action")
							.cursor_pointer()
							.small()
							.py_3()
							.icon(
								Icon::empty()
									.path("icons/custom/square-pen.svg"),
							)
							.label("Bulk Action")
							.warning(),
					)
					.child(
						Button::new("delete-action")
							.small()
							.cursor_pointer()
							.py_3()
							.icon(IconName::Delete)
							.label("Delete")
							.danger(),
					)
					.child(
						Button::new("add-employee-action")
							.small()
							.cursor_pointer()
							.py_3()
							.icon(IconName::Plus)
							.label("Add Employee")
							.success(),
					),
			)
	}
}

impl Render for EmployeeView {
	fn render(
		&mut self,
		window: &mut Window,
		cx: &mut Context<Self>,
	) -> impl IntoElement {
		v_flex()
			.flex_1()
			.flex_shrink_0()
			.child(self.render_header(window, cx))
			.child(Label::new("Home Dashboard"))
	}
}
