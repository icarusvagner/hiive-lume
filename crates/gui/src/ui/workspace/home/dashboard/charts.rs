use gpui::{prelude::FluentBuilder, *};
use gpui_component::{
	ActiveTheme, StyledExt, chart::{LineChart, PieChart}, v_flex
};

pub struct DashboardCharts {
	data: Vec<DataPoint>,
	emp_sales: Vec<EmployeeSalesPerformance>,
}

#[derive(Clone)]
struct DataPoint {
	month: String,
	revenue: f64,
}

#[derive(Clone)]
struct EmployeeSalesPerformance {
	name: String,
	profit: f64,
}

impl DashboardCharts {
	pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
		let data = vec![
			DataPoint { month: "Jan".to_string(), revenue: 100.0 },
			DataPoint { month: "Feb".to_string(), revenue: 150.0 },
			DataPoint { month: "Mar".to_string(), revenue: 120.0 },
			DataPoint { month: "Apr".to_string(), revenue: 220.0 },
			DataPoint { month: "May".to_string(), revenue: 192.0 },
			DataPoint { month: "Jun".to_string(), revenue: 123.0 },
			DataPoint { month: "Jul".to_string(), revenue: 323.0 },
			DataPoint { month: "Aug".to_string(), revenue: 223.0 },
			DataPoint { month: "Sep".to_string(), revenue: 111.0 },
			DataPoint { month: "Oct".to_string(), revenue: 234.0 },
			DataPoint { month: "Nov".to_string(), revenue: 600.0 },
			DataPoint { month: "Dec".to_string(), revenue: 783.0 },
		];

		let emp_sales = vec![
			EmployeeSalesPerformance {
				name: "Sam".to_string(),
				profit: 3_000.0,
			},
			EmployeeSalesPerformance {
				name: "Trish".to_string(),
				profit: 1_000.0,
			},
			EmployeeSalesPerformance {
				name: "Marj".to_string(),
				profit: 545.0,
			},
			EmployeeSalesPerformance {
				name: "Pope".to_string(),
				profit: 1_345.0,
			},
		];

		Self { data, emp_sales }
	}

	pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
		cx.new(|cx| Self::new(window, cx))
	}

	fn chart_container(
		&self,
		title: &str,
		chart: impl IntoElement,
		center: bool,
		cx: &mut Context<Self>,
	) -> impl IntoElement {
		v_flex()
			.flex_1()
			.h_full()
			.rounded_lg()
			.child(
				div()
					.when(center, |this| this.text_center())
					.font_semibold()
					.child(title.to_string()),
			)
			.child(
				div()
					.when(center, |this| this.text_center())
					.text_color(cx.theme().muted_foreground)
					.text_sm()
					.child("Data period label"),
			)
			.child(div().flex_1().py_4().child(chart))
			.child(
				div()
					.when(center, |this| this.text_center())
					.font_semibold()
					.text_sm()
					.child("Summary statistic"),
			)
			.child(
				div()
					.when(center, |this| this.text_center())
					.text_color(cx.theme().muted_foreground)
					.text_sm()
					.child("Including all sales for publishing"),
			)
	}

	fn weekly_sales_chart(
		&self,
		_window: &mut Window,
		cx: &mut Context<Self>,
	) -> Stateful<Div> {
		v_flex().id("publishing-chart").flex_1().h_full().child(
			self.chart_container(
				"Weekly Sales",
				LineChart::new(self.data.clone())
					.x(|d| d.month.clone())
					.y(|d| d.revenue)
					.stroke(cx.theme().info)
					.linear(),
				false,
				cx,
			),
		)
	}

	fn pie_chart_employee_sales(
		&self,
		_window: &mut Window,
		cx: &mut Context<Self>,
	) -> Div {
		v_flex().bg(cx.theme().accent.opacity(0.30)).rounded_lg().p_4().child(
			self.chart_container(
				"Employee Sales Performance",
				PieChart::new(self.emp_sales.clone())
					.value(|d| d.profit as f32)
					.outer_radius(112.)
					.color(|d| match d.name.as_str() {
						"Sam" => rgb(0x3772ff),
						"Trish" => rgb(0xdf2935),
						"Marj" => rgb(0xfdca40),
						"Pope" => rgb(0xff0054),
						_ => rgb(0x2563eb),
					}),
				true,
				cx,
			),
		)
	}

	fn line_chart_weekly_sale(
		&self,
		window: &mut Window,
		cx: &mut Context<Self>,
	) -> Div {
		v_flex()
			.flex_1()
			.h_112()
			.bg(cx.theme().accent.opacity(0.30))
			.rounded_lg()
			.p_4()
			.child(self.weekly_sales_chart(window, cx))
	}
}

impl Render for DashboardCharts {
	fn render(
		&mut self,
		window: &mut Window,
		cx: &mut Context<Self>,
	) -> impl IntoElement {
		div()
			.grid()
			.grid_cols(2)
			.gap_5()
			.h_full()
			.child(self.line_chart_weekly_sale(window, cx))
			.child(self.pie_chart_employee_sales(window, cx))
	}
}
