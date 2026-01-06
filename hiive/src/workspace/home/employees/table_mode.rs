use gpui::*;
use gpui_component::{
	ActiveTheme, IndexPath, Sizable, WindowExt, button::{Button, ButtonVariants}, checkbox::Checkbox, h_flex, label::Label, select::{Select, SelectItem, SelectState}, table::{Column, Table, TableDelegate, TableEvent, TableState}, v_flex
};

use crate::data::{
	employees::employees_data::EmployeesData, models::employees_model::EmployeeModel
};

pub struct EmployeeTableMode {
	table_state: Entity<TableState<EmployeeTableDelegate>>,
	rows_per_page_state: Entity<SelectState<Vec<RowsPerPage>>>,
}

struct EmployeeTableDelegate {
	data: Vec<EmployeeModel>,
	columns: Vec<Column>,
	select_all: bool,
	loading: bool,
}

#[derive(Clone, Debug)]
struct RowsPerPage {
	count: SharedString,
	name: SharedString,
}

impl SelectItem for RowsPerPage {
	type Value = SharedString;

	fn title(&self) -> SharedString {
		self.name.clone()
	}

	fn display_title(&self) -> Option<AnyElement> {
		Some(format!("{} {}", self.count, self.name).into_any_element())
	}

	fn value(&self) -> &Self::Value {
		&self.count
	}
}

impl EmployeeTableDelegate {
	pub fn new() -> Self {
		let data = EmployeesData::data();
		let columns = vec![
			Column::new("checkbox", "").width(40.0),
			Column::new("emp-name", "Employee Name").width(220.0).sortable(),
			Column::new("emp-position", "Position").width(325.0).sortable(),
			Column::new("emp-status", "Status").width(190.0).sortable(),
			Column::new("emp-email", "Email").sortable(),
			Column::new("emp-number", "Mobile Number").width(220.0),
			Column::new("emp-department", "Department").width(225.0).sortable(),
			Column::new("emp-date-join", "Date of Joining").width(160.0),
			Column::new("emp-actions", "Actions").width(200.0),
		];

		Self { data, columns, select_all: false, loading: false }
	}
}

impl TableDelegate for EmployeeTableDelegate {
	fn columns_count(&self, _cx: &App) -> usize {
		self.columns.len()
	}

	fn rows_count(&self, _cx: &App) -> usize {
		self.data.len()
	}

	fn column(&self, col_ix: usize, _cx: &App) -> &Column {
		&self.columns[col_ix]
	}

	fn render_th(
		&mut self,
		col_ix: usize,
		_window: &mut Window,
		_cx: &mut Context<TableState<Self>>,
	) -> impl IntoElement {
		let col = &self.columns[col_ix];

		match col.key.as_ref() {
			"checkbox" => div().child(
				Checkbox::new("select_all_employee").checked(self.select_all),
			),
			_ => div().child(col.name.clone()),
		}
	}

	fn render_td(
		&mut self,
		row_ix: usize,
		col_ix: usize,
		_window: &mut Window,
		_cx: &mut Context<TableState<Self>>,
	) -> impl IntoElement {
		let emp = &self.data[row_ix];
		let col = &self.columns[col_ix];

		match col.key.as_ref() {
			"checkbox" => div().child(Checkbox::new(SharedString::new(
				format!("{}-row_checked", col_ix),
			))),
			"emp-name" => h_flex()
				.gap_1()
				.child(img(emp.src()).h_8().w_8().rounded_full())
				.child(Label::new(emp.fullname())),
			"emp-position" => div().child(emp.position()),
			"emp-status" => div()
				.px_2()
				.text_center()
				.rounded_full()
				.bg(emp.status().color().opacity(0.30))
				.child(emp.status().as_str()),
			"emp-number" => div().child(Label::new(emp.number())),
			"emp-department" => {
				div().child(Label::new(emp.department().as_str()))
			}
			"emp-date-join" => div().child(Label::new(emp.date_joined())),
			"emp-actions" => h_flex().gap_2().child(
				Button::new(SharedString::new(format!(
					"emp-table-action-btn-{}",
					col_ix
				)))
				.px_3()
				.compact()
				.small()
				.label("More")
				.primary(),
			),
			_ => div(),
		}
	}

	fn render_tr(
		&mut self,
		row_ix: usize,
		_window: &mut Window,
		_cx: &mut Context<TableState<Self>>,
	) -> Stateful<Div> {
		div().flex().items_center().id(row_ix).h_12()
	}

	fn render_empty(
		&mut self,
		_window: &mut Window,
		cx: &mut Context<TableState<Self>>,
	) -> impl IntoElement {
		v_flex().justify_center().h_full().w_full().p_3().child(
			div()
				.bg(cx.theme().accent)
				.rounded_xl()
				.shadow_lg()
				.child("No Data available"),
		)
	}

	fn loading(&self, _cx: &App) -> bool {
		self.loading
	}
}

impl EmployeeTableMode {
	pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
		let delegate = EmployeeTableDelegate::new();
		let table_state = cx.new(|cx| {
			TableState::new(delegate, window, cx)
				.col_resizable(true)
				.col_movable(false)
				.sortable(true)
				.col_selectable(true)
				.row_selectable(true)
		});
		let rows_per_page_state = cx.new(|cx| {
			SelectState::new(
				vec![
					RowsPerPage {
						count: "10".into(),
						name: "Rows per page".into(),
					},
					RowsPerPage {
						count: "20".into(),
						name: "Rows per page".into(),
					},
					RowsPerPage {
						count: "30".into(),
						name: "Rows per page".into(),
					},
					RowsPerPage {
						count: "45".into(),
						name: "Rows per page".into(),
					},
					RowsPerPage {
						count: "55".into(),
						name: "Rows per page".into(),
					},
					RowsPerPage {
						count: "70".into(),
						name: "Rows per page".into(),
					},
					RowsPerPage {
						count: "85".into(),
						name: "Rows per page".into(),
					},
					RowsPerPage {
						count: "100".into(),
						name: "Rows per page".into(),
					},
				],
				Some(IndexPath::default()),
				window,
				cx,
			)
		});

		cx.subscribe_in(
			&table_state,
			window,
			|_view, _table, event, window, cx| {
				match event {
					TableEvent::DoubleClickedRow(row_ix) => {
						println!("Row {} double-clicked", row_ix);
						// Open detail view or edit mode

						window.open_dialog(cx, |dialog, _, _| {
							dialog
								.title("Show Candidates")
								.child("This is double clicked on row")
						});
					}
					_ => {}
				}
			},
		)
		.detach();

		Self { table_state, rows_per_page_state }
	}

	pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
		cx.new(|cx| Self::new(window, cx))
	}

	fn render_table_status(
		&self,
		_window: &mut Window,
		cx: &mut Context<Self>,
	) -> Div {
		div()
			.flex()
			.items_center()
			.justify_between()
			.w_full()
			.child(
				div().flex().items_start().justify_start().child(
					Select::new(&self.rows_per_page_state)
						.rounded_full()
						.bg(cx.theme().accent),
				),
			)
			.child(
				h_flex()
					.child(Label::new("1-10").font_weight(FontWeight::SEMIBOLD))
					.child(
						Label::new("/990 results")
							.text_sm()
							.font_weight(FontWeight::LIGHT),
					),
			)
	}

	fn render_table_mode(
		&self,
		window: &mut Window,
		cx: &mut Context<Self>,
	) -> Div {
		v_flex()
			.gap_5()
			.child(
				div().h(px(610.)).child(
					Table::new(&self.table_state.clone())
						.with_size(px(50.))
						.stripe(true)
						.scrollbar_visible(true, true)
						.bordered(true),
				),
			)
			.child(self.render_table_status(window, cx))
	}
}

impl Render for EmployeeTableMode {
	fn render(
		&mut self,
		window: &mut Window,
		cx: &mut Context<Self>,
	) -> impl IntoElement {
		div()
			.size_full()
			.flex()
			.flex_col()
			.flex_1()
			.flex_grow()
			.rounded_xl()
			.child(self.render_table_mode(window, cx))
	}
}
