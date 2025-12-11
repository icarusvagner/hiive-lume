use gpui::*;
use gpui_component::{
	ActiveTheme, Icon, Sizable, StyledExt, button::{Button, ButtonVariants}, h_flex, label::Label, table::{Column, Table, TableDelegate, TableState}, v_flex
};

pub struct DashboardTable {
	table: Entity<TableState<DashboardTableDelegate>>,
}

impl DashboardTable {
	pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
		let table = cx.new(|cx| {
			TableState::new(DashboardTableDelegate::new(window, cx), window, cx)
		});

		Self { table }
	}

	pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
		cx.new(|cx| Self::new(window, cx))
	}
}

impl Render for DashboardTable {
	fn render(
		&mut self,
		_window: &mut Window,
		cx: &mut Context<Self>,
	) -> impl IntoElement {
		v_flex()
			.h_112()
			.w_full()
			.child(
				Label::new("Recent Clocked-In").text_2xl().font_bold().mb_3(),
			)
			.child(
				div()
					.bg(cx.theme().accent.opacity(0.30))
					.h_full()
					.w_full()
					.p_4()
					.rounded_lg()
					.child(
						Table::new(&self.table)
							.stripe(true)
							.bordered(true)
							.scrollbar_visible(true, true),
					),
			)
	}
}

struct DashboardTableDelegate {
	data: Vec<EmployeeAttendanceTable>,
	columns: Vec<Column>,
}

#[derive(Clone)]
struct EmployeeAttendanceTable {
	id: i64,
	fullname: String,
	position: String,
	time_in: String,
	status: String,
}

impl DashboardTableDelegate {
	pub fn new(_window: &mut Window, _cx: &mut App) -> Self {
		let columns = vec![
			Column::new("emp-id", "EMP ID").width(60.),
			Column::new("fullname", "Fullname"),
			Column::new("position", "Position"),
			Column::new("time-in", "Clocked In"),
			Column::new("status", "Status"),
			Column::new("action", "Action"),
		];
		let data = Self::all_data();

		Self { data, columns }
	}

	#[rustfmt::skip]
	pub fn all_data() -> Vec<EmployeeAttendanceTable> {
		vec![
			EmployeeAttendanceTable { id: 1, fullname: "Aiden Cruz".into(), position: "Software Engineer".into(), time_in: "08:56".into(), status: "On-time".into(), },
			EmployeeAttendanceTable { id: 2, fullname: "Bella Santos".into(), position: "HR Officer".into(), time_in: "09:12".into(), status: "Late".into(), },
			EmployeeAttendanceTable { id: 3, fullname: "Caleb Dimaranan".into(), position: "UI/UX Designer".into(), time_in: "—".into(), status: "Absent".into(), },
			EmployeeAttendanceTable { id: 4, fullname: "Daphne Villareal".into(), position: "Project Manager".into(), time_in: "08:45".into(), status: "On-time".into(), },
			EmployeeAttendanceTable { id: 5, fullname: "Ezekiel Ramos".into(), position: "QA Tester".into(), time_in: "09:03".into(), status: "Late".into(), },
			EmployeeAttendanceTable { id: 6, fullname: "Fiona Martinez".into(), position: "Accountant".into(), time_in: "—".into(), status: "On Leave".into(), },
			EmployeeAttendanceTable { id: 7, fullname: "Gabriel Navarro".into(), position: "IT Support".into(), time_in: "08:41".into(), status: "On-time".into(), },
			EmployeeAttendanceTable { id: 8, fullname: "Harper Lee".into(), position: "Front Desk Associate".into(), time_in: "09:28".into(), status: "Late".into(), },
			EmployeeAttendanceTable { id: 9, fullname: "Ivan Torres".into(), position: "Security Personnel".into(), time_in: "07:01".into(), status: "On-time".into(), },
			EmployeeAttendanceTable { id: 10, fullname: "Jasmine Ocampo".into(), position: "Marketing Specialist".into(), time_in: "08:59".into(), status: "On-time".into(), },
			EmployeeAttendanceTable { id: 11, fullname: "Kyle Bautista".into(), position: "Content Writer".into(), time_in: "09:15".into(), status: "Late".into(), },
			EmployeeAttendanceTable { id: 12, fullname: "Luna Cervantes".into(), position: "HR Assistant".into(), time_in: "—".into(), status: "Absent".into(), },
			EmployeeAttendanceTable { id: 13, fullname: "Mason Aguilar".into(), position: "DevOps Engineer".into(), time_in: "08:32".into(), status: "On-time".into(), },
			EmployeeAttendanceTable { id: 14, fullname: "Nina Soriano".into(), position: "Product Owner".into(), time_in: "08:49".into(), status: "On-time".into(), },
			EmployeeAttendanceTable { id: 15, fullname: "Owen Castillo".into(), position: "Business Analyst".into(), time_in: "09:07".into(), status: "Late".into(), },
			EmployeeAttendanceTable { id: 16, fullname: "Paige Alonzo".into(), position: "Finance Clerk".into(), time_in: "—".into(), status: "On Leave".into(), },
			EmployeeAttendanceTable { id: 17, fullname: "Quinn Mendez".into(), position: "Graphic Designer".into(), time_in: "08:51".into(), status: "On-time".into(), },
			EmployeeAttendanceTable { id: 18, fullname: "Riley Serrano".into(), position: "Warehouse Staff".into(), time_in: "07:58".into(), status: "On-time".into(), },
			EmployeeAttendanceTable { id: 19, fullname: "Sophie Enriquez".into(), position: "Admin Assistant".into(), time_in: "09:34".into(), status: "Late".into(), },
			EmployeeAttendanceTable { id: 20, fullname: "Theo Manalo".into(), position: "Legal Officer".into(), time_in: "—".into(), status: "Absent".into(), },
			EmployeeAttendanceTable { id: 21, fullname: "Uma Delgado".into(), position: "Procurement Officer".into(), time_in: "08:44".into(), status: "On-time".into(), },
			EmployeeAttendanceTable { id: 22, fullname: "Victor Lam".into(), position: "IT Admin".into(), time_in: "08:36".into(), status: "On-time".into(), },
			EmployeeAttendanceTable { id: 23, fullname: "Will Torres".into(), position: "Customer Support".into(), time_in: "09:05".into(), status: "Late".into(), },
			EmployeeAttendanceTable { id: 24, fullname: "Xena Prado".into(), position: "Recruitment Specialist".into(), time_in: "—".into(), status: "On Leave".into(), },
			EmployeeAttendanceTable { id: 25, fullname: "Yuri Santos".into(), position: "Systems Analyst".into(), time_in: "08:21".into(), status: "On-time".into(), },
			EmployeeAttendanceTable { id: 26, fullname: "Zane Rubio".into(), position: "Junior Developer".into(), time_in: "09:11".into(), status: "Late".into(), },
			EmployeeAttendanceTable { id: 27, fullname: "Adam Peralta".into(), position: "Logistics".into(), time_in: "08:02".into(), status: "On-time".into(), },
			EmployeeAttendanceTable { id: 28, fullname: "Bea Cruz".into(), position: "Office Clerk".into(), time_in: "09:22".into(), status: "Late".into(), },
			EmployeeAttendanceTable { id: 29, fullname: "Cody Rivera".into(), position: "Inventory Keeper".into(), time_in: "—".into(), status: "Absent".into(), },
			EmployeeAttendanceTable { id: 30, fullname: "Dana Vicente".into(), position: "Nurse (Company Clinic)".into(), time_in: "08:18".into(), status: "On-time".into(), },
			EmployeeAttendanceTable { id: 31, fullname: "Evan Sy".into(), position: "Training Coordinator".into(), time_in: "08:53".into(), status: "On-time".into(), },
			EmployeeAttendanceTable { id: 32, fullname: "Faith Go".into(), position: "Account Manager".into(), time_in: "09:17".into(), status: "Late".into(), },
			EmployeeAttendanceTable { id: 33, fullname: "Gino Robles".into(), position: "Production Staff".into(), time_in: "07:45".into(), status: "On-time".into(), },
			EmployeeAttendanceTable { id: 34, fullname: "Hannah Mercado".into(), position: "Payroll Specialist".into(), time_in: "08:39".into(), status: "On-time".into(), },
			EmployeeAttendanceTable { id: 35, fullname: "Ian Feliciano".into(), position: "Backend Developer".into(), time_in: "09:30".into(), status: "Late".into(), },
			EmployeeAttendanceTable { id: 36, fullname: "Joy Reyes".into(), position: "Office Staff".into(), time_in: "—".into(), status: "On Leave".into(), },
			EmployeeAttendanceTable { id: 37, fullname: "Ken Garcia".into(), position: "IT Auditor".into(), time_in: "08:47".into(), status: "On-time".into(), },
			EmployeeAttendanceTable { id: 38, fullname: "Lara Estrella".into(), position: "CSR".into(), time_in: "09:25".into(), status: "Late".into(), },
			EmployeeAttendanceTable { id: 39, fullname: "Mark Uy".into(), position: "Electrician".into(), time_in: "07:33".into(), status: "On-time".into(), },
			EmployeeAttendanceTable { id: 40, fullname: "Nora David".into(), position: "Purchasing Assistant".into(), time_in: "—".into(), status: "Absent".into(), },
			EmployeeAttendanceTable { id: 41, fullname: "Omar Castillo".into(), position: "Operations Supervisor".into(), time_in: "08:25".into(), status: "On-time".into(), },
			EmployeeAttendanceTable { id: 42, fullname: "Polly Guevarra".into(), position: "Quality Analyst".into(), time_in: "09:19".into(), status: "Late".into(), },
			EmployeeAttendanceTable { id: 43, fullname: "Raffy Jimenez".into(), position: "Field Technician".into(), time_in: "07:56".into(), status: "On-time".into(), },
			EmployeeAttendanceTable { id: 44, fullname: "Sandy Quirino".into(), position: "Training Assistant".into(), time_in: "—".into(), status: "On Leave".into(), },
			EmployeeAttendanceTable { id: 45, fullname: "Troy Del Mundo".into(), position: "Security Personnel".into(), time_in: "07:10".into(), status: "On-time".into(), },
			EmployeeAttendanceTable { id: 46, fullname: "Ursula Lim".into(), position: "Project Coordinator".into(), time_in: "09:08".into(), status: "Late".into(), },
			EmployeeAttendanceTable { id: 47, fullname: "Vince Corpuz".into(), position: "Junior Accountant".into(), time_in: "08:51".into(), status: "On-time".into(), },
			EmployeeAttendanceTable { id: 48, fullname: "Wena Paras".into(), position: "Sales Assistant".into(), time_in: "09:04".into(), status: "Late".into(), },
			EmployeeAttendanceTable { id: 49, fullname: "Xavier Ong".into(), position: "Software Tester".into(), time_in: "08:28".into(), status: "On-time".into(), },
			EmployeeAttendanceTable { id: 50, fullname: "Yanna Soliman".into(), position: "Executive Assistant".into(), time_in: "—".into(), status: "Absent".into(), },
		]
	}
}

impl TableDelegate for DashboardTableDelegate {
	fn columns_count(&self, _cx: &App) -> usize {
		self.columns.len()
	}

	fn rows_count(&self, _cx: &App) -> usize {
		self.data.len()
	}

	fn column(&self, col_ix: usize, _cx: &App) -> &Column {
		&self.columns[col_ix]
	}

	fn render_td(
		&mut self,
		row_ix: usize,
		col_ix: usize,
		_window: &mut Window,
		_cx: &mut Context<gpui_component::table::TableState<Self>>,
	) -> impl IntoElement {
		let row = &self.data[row_ix];
		let col = &self.columns[col_ix];

		match col.key.as_ref() {
			"emp-id" => div().child(row.id.to_string()),
			"fullname" => div().child(row.fullname.clone()),
			"position" => div().child(row.position.clone()),
			"time-in" => div().child(row.time_in.clone()),
			"status" => div().child(row.status.clone()),
			"action" => h_flex().gap_1().child(
				Button::new("update")
					.icon(
						Icon::empty()
							.path("icons/custom/update-page.svg")
							.size_4(),
					)
					.small()
					.compact()
					.ghost(),
			),
			_ => div(),
		}
	}

	fn load_more_threshold(&self) -> usize {
		20
	}
}
