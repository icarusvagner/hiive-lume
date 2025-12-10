use gpui::{prelude::FluentBuilder, *};
use gpui_component::{
    ActiveTheme, Icon, IconName, Side, Sizable, h_flex,
    sidebar::{
        Sidebar, SidebarFooter, SidebarHeader, SidebarMenu, SidebarMenuItem, SidebarToggleButton,
    },
    v_flex,
};

use crate::{
    states::view_layout::HomeActiveView,
    ui::workspace::ui::home::{dashboard::DashboardView, employee::EmployeeView},
};

pub struct Homeview {
    view: HomeActiveView,
    dashboard: Entity<DashboardView>,
    employee: Entity<EmployeeView>,
    collapse_menu: bool,
}

impl Homeview {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let dashboard = DashboardView::view(window, cx);
        let employee = EmployeeView::view(window, cx);

        Self {
            view: HomeActiveView::Dashboard,
            dashboard,
            employee,
            collapse_menu: false,
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn render_employee(&self, _cx: &mut Context<Self>) -> Stateful<Div> {
        v_flex()
            .size_full()
            .id("employee-view")
            .child(self.employee.clone())
    }

    fn render_dashboard(&self, _cx: &mut Context<Self>) -> Stateful<Div> {
        v_flex()
            .size_full()
            .id("dashboard-view")
            .child(self.dashboard.clone())
    }

    fn render_content(&self, _window: &mut Window, cx: &mut Context<Self>) -> Stateful<Div> {
        match self.view {
            HomeActiveView::Dashboard => self.render_dashboard(cx),
            HomeActiveView::Employees => self.render_employee(cx),
            _ => unreachable!(),
        }
    }

    fn render_sidebar_menu(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let _size = if self.collapse_menu { px(24.) } else { px(28.) };

        Sidebar::new(Side::Left)
            .max_w(px(360.))
            .min_w(px(50.))
            .bg(cx.theme().background.opacity(0.50))
            .collapsed(self.collapse_menu)
            .collapsible(true)
            .header(
                SidebarHeader::new().child(
                    h_flex()
                        .gap_2()
                        .child(
                            Icon::new(Icon::empty().path("icons/custom/crown.svg"))
                                .text_color(cx.theme().primary)
                                .size(_size),
                        )
                        .when(!self.collapse_menu, |this| this.child("Admin Panel")),
                ),
            )
            .child(
                SidebarMenu::new().child(
                    SidebarMenuItem::new("Dashbaord")
                        .icon(Icon::new(IconName::LayoutDashboard).size(_size))
                        .active(true),
                ),
            )
            .footer(
                SidebarFooter::new()
                    .child(Icon::empty().path("icons/custom/log-out.svg").size(_size))
                    .when(!self.collapse_menu, |this| this.child("Administrator")),
            )
    }

    fn render_header(&self, _window: &mut Window, cx: &mut Context<Self>) -> Stateful<Div> {
        h_flex()
            .bg(cx.theme().background.opacity(0.50))
            .id("home-header")
            .w_full()
            .items_center()
            .px_32()
            .child(
                SidebarToggleButton::left()
                    .collapsed(self.collapse_menu)
                    .on_click(
                        cx.listener(|this, _, _, _| this.collapse_menu = !this.collapse_menu),
                    ),
            )
    }
}

impl Render for Homeview {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content = h_flex()
            .h_full()
            .w_full()
            .child(self.render_sidebar_menu(window, cx))
            .child(
                v_flex()
                    .h_full()
                    .w_full()
                    .child(self.render_header(window, cx))
                    .child(self.render_content(window, cx)),
            );

        v_flex().h_full().w_full().child(content)
    }
}
