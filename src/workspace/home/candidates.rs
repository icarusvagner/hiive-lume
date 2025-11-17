use gpui::*;
use gpui_component::{
    ActiveTheme, IconName, IndexPath, Sizable, StyledExt, WindowExt,
    button::{Button, ButtonCustomVariant, ButtonVariants},
    h_flex,
    select::{Select, SelectState},
    white,
};

use crate::data::home::interviews_data::InterviewsData;

pub struct Candidates {
    timeframe_idx: usize,
    position_idx: usize,
}

impl Candidates {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {
            timeframe_idx: 0,
            position_idx: 0,
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn render_top_content(&mut self, window: &mut Window, cx: &mut Context<Self>) -> Div {
        let data = InterviewsData::data();
        let total_candidates = data.iter().count();
        let positions = data.iter().map(|c| c.position()).collect::<Vec<String>>();

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
                    .child(self.render_filters(positions, window, cx)),
            )
            .child(
                Button::new("add-newjob-btn")
                    .large()
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
                        window
                            .open_dialog(cx, |dialog, _, _| dialog.title("Post new Jobe").alert());
                    }),
            )
    }

    fn timeframe_select(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<SelectState<Vec<&'static str>>> {
        let timeframe_select_state = cx
            .new(|cx| SelectState::new(vec!["New", "Old"], Some(IndexPath::default()), window, cx));

        timeframe_select_state.update(cx, |state, cx| {
            state.set_selected_index(
                Some(IndexPath::default().row(self.timeframe_idx)),
                window,
                cx,
            );
        });

        timeframe_select_state
    }

    fn position_state(
        &self,
        mut positions: Vec<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<SelectState<Vec<String>>> {
        positions.dedup();
        let position_state =
            cx.new(|cx| SelectState::new(positions, Some(IndexPath::default()), window, cx));

        position_state.update(cx, |state, cx| {
            state.set_selected_index(
                Some(IndexPath::default().row(self.position_idx)),
                window,
                cx,
            );
        });

        position_state
    }

    fn render_filters(
        &self,
        positions: Vec<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Div {
        h_flex().justify_between().items_center().child(
            h_flex()
                .items_start()
                .gap_3()
                .child(
                    Select::new(&self.timeframe_select(window, cx))
                        .appearance(false)
                        .rounded_full()
                        .border_1()
                        .border_color(cx.theme().foreground.opacity(0.40)),
                )
                .child(
                    Select::new(&self.position_state(positions, window, cx))
                        .appearance(false)
                        .rounded_full()
                        .border_1()
                        .border_color(cx.theme().foreground.opacity(0.40)),
                ),
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
