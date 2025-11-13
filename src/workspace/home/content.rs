use gpui::*;
use gpui_component::{ActiveTheme, Sizable, spinner::Spinner, v_flex};

use crate::{
    states::home_layout::{HomeActiveLayout, HomeLayout},
    workspace::home::{
        candidates::Candidates,
        dashboard::Dashboard,
        employees::{create::CreateEmployee, view::Employees},
        jobs::Jobs,
        leaves::Leaves,
        settings::Settings,
    },
};

pub struct HomeContent {
    active: HomeActiveLayout,
    dashboard: Entity<Dashboard>,
    employees: Entity<Employees>,
    jobs: Entity<Jobs>,
    candidates: Entity<Candidates>,
    leaves: Entity<Leaves>,
    settings: Entity<Settings>,
    create_employee: Entity<CreateEmployee>,
    _subscription: Vec<Subscription>,
}

impl HomeContent {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let dashboard = Dashboard::view(window, cx);
        let employees = Employees::view(window, cx);
        let jobs = Jobs::view(window, cx);
        let candidates = Candidates::view(window, cx);
        let leaves = Leaves::view(window, cx);
        let settings = Settings::view(window, cx);
        let create_employee = CreateEmployee::view(window, cx);

        let _subscription = vec![cx.observe_global::<HomeLayout>(move |this, cx| {
            this.active = cx.global::<HomeLayout>().home.clone();
            cx.notify();
        })];

        Self {
            active: HomeActiveLayout::Dashboard,
            dashboard,
            employees,
            jobs,
            candidates,
            leaves,
            settings,
            create_employee,
            _subscription,
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn render_dashboard(&mut self, cx: &mut Context<Self>) -> Stateful<Div> {
        let content = div()
            .id("dashboard")
            .flex()
            .flex_col()
            .content_center()
            .flex_grow()
            .h_full()
            .w_full()
            .bg(cx.theme().background)
            .child(self.dashboard.clone());

        content
    }

    fn render_employees(&mut self, cx: &mut Context<Self>) -> Stateful<Div> {
        let content = div()
            .id("employees")
            .flex()
            .flex_col()
            .content_center()
            .flex_grow()
            .h_full()
            .bg(cx.theme().background)
            .child(self.employees.clone());

        content
    }

    fn render_jobs(&mut self, cx: &mut Context<Self>) -> Stateful<Div> {
        let content = div()
            .id("jobs")
            .flex()
            .flex_col()
            .content_center()
            .flex_grow()
            .h_full()
            .bg(cx.theme().background)
            .child(self.jobs.clone());

        content
    }

    fn render_candidates(&mut self, cx: &mut Context<Self>) -> Stateful<Div> {
        let content = div()
            .id("candidates")
            .flex()
            .flex_col()
            .content_center()
            .flex_grow()
            .h_full()
            .bg(cx.theme().background)
            .child(self.candidates.clone());

        content
    }

    fn render_leaves(&mut self, cx: &mut Context<Self>) -> Stateful<Div> {
        let content = div()
            .id("leaves")
            .flex()
            .flex_col()
            .content_center()
            .flex_grow()
            .h_full()
            .bg(cx.theme().background)
            .child(self.leaves.clone());

        content
    }

    fn render_settings(&mut self, cx: &mut Context<Self>) -> Stateful<Div> {
        let content = div()
            .id("settings")
            .flex()
            .flex_col()
            .content_center()
            .flex_grow()
            .h_full()
            .bg(cx.theme().background)
            .child(self.settings.clone());

        content
    }

    fn render_create_employee(&mut self, cx: &mut Context<Self>) -> Stateful<Div> {
        let content = div()
            .id("create-employee")
            .flex()
            .flex_col()
            .content_center()
            .flex_grow()
            .h_full()
            .bg(cx.theme().background)
            .child(self.create_employee.clone());

        content
    }

    fn render_loading_home(&mut self, cx: &mut Context<Self>) -> Stateful<Div> {
        let content = div()
            .id("loading-content")
            .flex()
            .flex_grow()
            .bg(cx.theme().background)
            .justify_center()
            .items_center()
            .child(
                v_flex()
                    .items_center()
                    .justify_center()
                    .gap_10()
                    .child(Spinner::new().color(cx.theme().blue).with_size(px(160.))),
            );

        content
    }
}

impl Render for HomeContent {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content = match self.active {
            HomeActiveLayout::Dashboard => self.render_dashboard(cx),
            HomeActiveLayout::Employees => self.render_employees(cx),
            HomeActiveLayout::Jobs => self.render_jobs(cx),
            HomeActiveLayout::Candidates => self.render_candidates(cx),
            HomeActiveLayout::Leaves => self.render_leaves(cx),
            HomeActiveLayout::Settings => self.render_settings(cx),
            HomeActiveLayout::CreateEmployee => self.render_create_employee(cx),
            HomeActiveLayout::Loading => self.render_loading_home(cx),
        };

        div().flex().h_full().flex_grow().child(content)
    }
}
