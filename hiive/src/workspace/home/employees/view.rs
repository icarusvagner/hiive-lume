use gpui::*;
use gpui_component::{
    ActiveTheme, Icon, IconName, IndexPath, Selectable, Sizable, StyledExt,
    button::{Button, ButtonCustomVariant, ButtonGroup, ButtonVariants},
    h_flex,
    label::Label,
    select::{Select, SelectState},
    v_flex,
};

use crate::{
    states::{
        candidate_switch_state::{ActiveSwitchModeState, SwitchMode},
        home_layout::HomeLayout,
    },
    workspace::home::employees::{card_mode::EmployeeCardMode, table_mode::EmployeeTableMode},
};

pub struct Employees {
    timeframe_state: Entity<SelectState<Vec<&'static str>>>,
    card_mode: Entity<EmployeeCardMode>,
    table_mode: Entity<EmployeeTableMode>,
    switch_mode: SwitchMode,
}

impl Employees {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let card_mode = EmployeeCardMode::view(window, cx);
        let table_mode = EmployeeTableMode::view(window, cx);

        let timeframe_state = cx
            .new(|cx| SelectState::new(vec!["New", "Old"], Some(IndexPath::default()), window, cx));

        Self {
            timeframe_state,
            card_mode,
            table_mode,
            switch_mode: SwitchMode::CardMode,
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn toggle_switch_mode(&mut self, index: usize, _window: &mut Window, cx: &mut Context<Self>) {
        let switch_state = cx.global_mut::<ActiveSwitchModeState>();
        switch_state.mode = self.switch_mode.clone();
        self.switch_mode = self.switch_mode.to_mode(index);
        cx.notify();
    }

    fn render_top_content(&self, window: &mut Window, cx: &mut Context<Self>) -> Div {
        v_flex()
            .px_10()
            .py_6()
            .bg(cx.theme().accent)
            .child(
                h_flex()
                    .justify_between()
                    .child(
                        Label::new("Employees")
                            .text_size(AbsoluteLength::Pixels(px(55.0)))
                            .font_weight(FontWeight::BOLD),
                    )
                    .child(
                        Button::new("goto-add-employee")
                            .custom(
                                ButtonCustomVariant::new(cx)
                                    .color(cx.theme().blue)
                                    .foreground(white())
                                    .border(cx.theme().blue)
                                    .hover(cx.theme().blue.opacity(0.80))
                                    .active(cx.theme().blue),
                            )
                            .rounded_full()
                            .icon(Icon::new(IconName::Plus))
                            .label("Add Employee")
                            .on_click(cx.listener(move |_, _, _, cx| {
                                let state = cx.global_mut::<HomeLayout>();
                                state.home =
                                    crate::states::home_layout::HomeActiveLayout::CreateEmployee;
                            })),
                    ),
            )
            .child(self.render_filters(window, cx))
    }

    fn render_filters(&self, _window: &mut Window, cx: &mut Context<Self>) -> Div {
        div()
            .mt_5()
            .flex()
            .items_center()
            .justify_between()
            .child(
                div().flex().items_start().justify_start().gap_2().child(
                    Select::new(&self.timeframe_state)
                        .py_3()
                        .menu_width(px(110.0))
                        .w_24()
                        .appearance(false)
                        .rounded_full()
                        .border_1()
                        .border_color(cx.theme().foreground.opacity(0.40)),
                ),
            )
            .child(
                ButtonGroup::new("employee-toggle-group")
                    .border_1()
                    .rounded_full()
                    .border_color(cx.theme().foreground)
                    .gap_3()
                    .flex()
                    .items_center()
                    .justify_end()
                    .child(
                        Button::new("employee-card-type")
                            .custom(
                                ButtonCustomVariant::new(cx)
                                    .color(cx.theme().secondary)
                                    .foreground(cx.theme().secondary_foreground)
                                    .hover(cx.theme().secondary.opacity(0.30))
                                    .active(cx.theme().primary),
                            )
                            .p_5()
                            .rounded_full()
                            .cursor_pointer()
                            .large()
                            .icon(Icon::new(IconName::LayoutDashboard).text_color(
                                if self.switch_mode.eq(&SwitchMode::CardMode) {
                                    white()
                                } else {
                                    cx.theme().foreground
                                },
                            ))
                            .selected(self.switch_mode.eq(&SwitchMode::CardMode)),
                    )
                    .child(
                        Button::new("employee-table-type")
                            .custom(
                                ButtonCustomVariant::new(cx)
                                    .color(cx.theme().secondary)
                                    .foreground(cx.theme().secondary_foreground)
                                    .hover(cx.theme().secondary.opacity(0.30))
                                    .active(cx.theme().primary),
                            )
                            .p_5()
                            .rounded_full()
                            .cursor_pointer()
                            .large()
                            .icon(Icon::empty().path("icons/custom/list-line.svg").text_color(
                                if self.switch_mode.eq(&SwitchMode::TableMode) {
                                    white()
                                } else {
                                    cx.theme().foreground
                                },
                            ))
                            .selected(self.switch_mode.eq(&SwitchMode::TableMode)),
                    )
                    .on_click(cx.listener(|this, clicks: &Vec<usize>, window, cx| {
                        this.toggle_switch_mode(clicks[0], window, cx)
                    })),
            )
    }

    fn render_card_mode(&self, _cx: &mut Context<Self>) -> Stateful<Div> {
        let content = v_flex()
            .id("render_employee_card_mode")
            .px_10()
            .py_6()
            .size_full()
            .child(self.card_mode.clone());

        content
    }

    fn render_table_mode(&self, _cx: &mut Context<Self>) -> Stateful<Div> {
        let content = v_flex()
            .id("render_employee_table_mode")
            .px_10()
            .py_6()
            .size_full()
            .child(self.table_mode.clone());

        content
    }
}

impl Render for Employees {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mode_content = match self.switch_mode {
            SwitchMode::CardMode => self.render_card_mode(cx),
            SwitchMode::TableMode => self.render_table_mode(cx),
        };

        div()
            .size_full()
            .flex()
            .flex_col()
            .flex_1()
            .flex_grow()
            .child(self.render_top_content(window, cx))
            .child(mode_content)
            .scrollable(Axis::Vertical)
    }
}
