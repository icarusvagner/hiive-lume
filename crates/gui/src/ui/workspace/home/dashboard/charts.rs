use chrono::{Timelike, Utc};
use gpui::{prelude::FluentBuilder, *};
use gpui_component::{
	ActiveTheme, StyledExt, chart::LineChart, label::Label, tab::{Tab, TabBar}, v_flex
};

pub struct DashboardCharts {
	active_tab: usize,
	tabs: Vec<String>,
	data: Vec<DataPoint>,
}

#[derive(Clone)]
struct DataPoint {
	month: String,
	revenue: f64,
}

impl DashboardCharts {
	pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
		let tabs = vec!["Publishings".to_string(), "Animations".to_string()];

		let data = vec![
			DataPoint { month: "Jan".to_string(), revenue: 100.0 },
			DataPoint { month: "Feb".to_string(), revenue: 150.0 },
			DataPoint { month: "Mar".to_string(), revenue: 120.0 },
			DataPoint { month: "Apr".to_string(), revenue: 220.0 },
			DataPoint { month: "May".to_string(), revenue: 192.0 },
			DataPoint { month: "Jun".to_string(), revenue: 123.0 },
		];

		Self { active_tab: 0, tabs, data }
	}

	pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
		cx.new(|cx| Self::new(window, cx))
	}

	fn publishing_chart(
		&self,
		_window: &mut Window,
		cx: &mut Context<Self>,
	) -> Stateful<Div> {
		v_flex().id("publishing-chart").flex_1().h_full().child(
			div().flex_1().p_3().child(
				LineChart::new(self.data.clone())
					.x(|d| d.month.clone())
					.y(|d| d.revenue)
					.stroke(cx.theme().info)
					.tick_margin(1)
					.linear(),
			),
		)
	}

	fn animation_chart(
		&self,
		_window: &mut Window,
		cx: &mut Context<Self>,
	) -> Stateful<Div> {
		v_flex().id("animation-chart").flex_1().h_full().child(
			div().flex_1().p_3().child(
				LineChart::new(self.data.clone())
					.x(|d| d.month.clone())
					.y(|d| d.revenue * Utc::now().hour() as f64)
					.stroke(cx.theme().success)
					.linear(),
			),
		)
	}

	fn line_chart_weekly_sale(
		&self,
		window: &mut Window,
		cx: &mut Context<Self>,
	) -> Div {
		v_flex()
			.h_96()
			.flex_1()
			.bg(cx.theme().accent.opacity(0.30))
			.rounded_lg()
			.p_3()
			.child(Label::new("Weekly Sales").text_xl().font_bold().mb_2())
			.child(
				TabBar::new("default-tabs")
					.underline()
					.selected_index(self.active_tab)
					.on_click(cx.listener(|this, index, _, cx| {
						this.active_tab = *index;
						cx.notify();
					}))
					.children(
						self.tabs
							.iter()
							.map(|tab_name| Tab::new().label(tab_name.clone())),
					),
			)
			.when(self.active_tab == 0, |this| {
				this.child(self.publishing_chart(window, cx))
			})
			.when(self.active_tab == 1, |this| {
				this.child(self.animation_chart(window, cx))
			})
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
	}
}
