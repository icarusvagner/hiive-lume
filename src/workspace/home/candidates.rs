use gpui::*;
use gpui_component::{
    ActiveTheme, Icon, IconName, IndexPath, Selectable, Sizable, StyledExt, WindowExt,
    button::{Button, ButtonCustomVariant, ButtonGroup, ButtonVariants},
    h_flex,
    select::{Select, SelectState},
    white,
};

use crate::data::home::interviews_data::InterviewsData;

pub struct Candidates {
    timeframe_state: Entity<SelectState<Vec<String>>>,
    position_state: Entity<SelectState<Vec<String>>>,
    active_button: usize,
}

impl Candidates {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        let data = InterviewsData::data();
        let mut positions = data.iter().map(|c| c.position()).collect::<Vec<String>>();
        positions.dedup();

        cx.new(|cx| {
            let timeframe_state = cx.new(|cx| {
                SelectState::new(
                    vec!["New".to_string(), "Old".to_string()],
                    Some(IndexPath::default()),
                    window,
                    cx,
                )
            });

            let position_state =
                cx.new(|cx| SelectState::new(positions, Some(IndexPath::default()), window, cx));

            Candidates {
                timeframe_state,
                position_state,
                active_button: 0,
            }
        })
    }

    fn render_top_content(&mut self, window: &mut Window, cx: &mut Context<Self>) -> Div {
        let data = InterviewsData::data();
        let total_candidates = data.iter().count();

        div()
            .flex()
            .px_10()
            .pt_6()
            .pb_32()
            .bg(cx.theme().accent)
            .justify_between()
            .items_center()
            .child(
                div()
                    .flex()
                    .w_full()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .child("Candidates")
                            .text_size(AbsoluteLength::Pixels(px(55.0)))
                            .font_bold(),
                    )
                    .child(
                        div()
                            .child(format!("{} Total Candidates", total_candidates))
                            .text_size(AbsoluteLength::Pixels(px(22.0)))
                            .text_color(cx.theme().accent_foreground.opacity(0.70))
                            .font_thin(),
                    )
                    .child(self.render_filters(window, cx)),
            )
            .child(
                Button::new("add-newjob-btn")
                    .large()
                    .p_3()
                    .custom(
                        ButtonCustomVariant::new(cx)
                            .color(cx.theme().blue)
                            .foreground(white())
                            .border(cx.theme().blue)
                            .hover(cx.theme().blue.opacity(0.80))
                            .active(cx.theme().blue),
                    )
                    .rounded_full()
                    .icon(IconName::Plus)
                    .label("Add New Job")
                    .cursor_pointer()
                    .on_click(|_, window, cx| {
                        window.open_dialog(cx, |dialog, _, _| dialog.title("Post new Job").alert());
                    }),
            )
    }

    fn render_filters(&self, _window: &mut Window, cx: &mut Context<Self>) -> Div {
        div()
            .flex()
            .justify_between()
            .items_center()
            .child(
                h_flex()
                    .mt_5()
                    .items_start()
                    .gap_2()
                    .child(
                        Select::new(&self.timeframe_state)
                            .py_3()
                            .menu_width(px(110.0))
                            .appearance(false)
                            .rounded_full()
                            .border_1()
                            .border_color(cx.theme().foreground.opacity(0.40)),
                    )
                    .child(
                        Select::new(&self.position_state)
                            .py_3()
                            .menu_width(px(225.0))
                            .appearance(false)
                            .rounded_full()
                            .border_1()
                            .border_color(cx.theme().foreground.opacity(0.40)),
                    ),
            )
            .child(
                ButtonGroup::new("toggle-group")
                    .gap_3()
                    .flex()
                    .items_center()
                    .justify_end()
                    .child(
                        Button::new("table-type")
                            .rounded_full()
                            .cursor_pointer()
                            .large()
                            .icon(IconName::LayoutDashboard)
                            .selected(self.active_button == 0),
                    )
                    .child(
                        Button::new("card-type")
                            .rounded_full()
                            .cursor_pointer()
                            .large()
                            .icon(Icon::empty().path("icons/custom/list-line.svg"))
                            .selected(self.active_button == 1),
                    )
                    .on_click(cx.listener(|this, clicks: &Vec<usize>, _, _| {
                        this.active_button = clicks[0];
                    })),
            )
    }
}

impl Render for Candidates {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .relative()
            .child(self.render_top_content(window, cx))
    }
}
