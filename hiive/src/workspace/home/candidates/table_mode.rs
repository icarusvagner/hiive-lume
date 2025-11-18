use std::ops::Range;

use gpui::*;
use gpui_component::{
    ActiveTheme, Sizable,
    button::{Button, ButtonCustomVariant, ButtonVariants},
    table::{Column, ColumnSort, Table, TableDelegate, TableState},
    v_flex,
};

use crate::data::{
    home::candidates_data::Candidatesdata, models::candidates_model::card_mode::CandidatesCardModel,
};

pub struct TableModeView {
    table_state: Entity<TableState<TableModeViewDelegate>>,
}

struct TableModeViewDelegate {
    rows: Vec<CandidatesCardModel>,
    columns: Vec<Column>,
    loading: bool,
    visible_rows: Range<usize>,
}

impl TableModeViewDelegate {
    pub fn new() -> Self {
        let rows = Candidatesdata::data();

        let columns = vec![
            Column::new("candidate-name", "Candidate Name")
                .resizable(true)
                .sortable(),
            Column::new("designation", "Designation")
                .resizable(true)
                .sortable(),
            Column::new("candidate-status", "Status")
                .resizable(true)
                .sortable(),
            Column::new("candidate-email", "Email")
                .resizable(true)
                .sortable(),
            Column::new("candidate-number", "Mobile Number")
                .resizable(true)
                .sortable(),
            Column::new("applied-on", "Applied On")
                .resizable(true)
                .sortable(),
            Column::new("actions", "Action").width(150.0),
        ];

        Self {
            rows,
            columns,
            loading: false,
            visible_rows: Range::default(),
        }
    }
}

impl TableDelegate for TableModeViewDelegate {
    fn columns_count(&self, _cx: &App) -> usize {
        self.columns.len()
    }

    fn rows_count(&self, _cx: &App) -> usize {
        self.rows.len()
    }

    fn column(&self, col_ix: usize, _cx: &App) -> &Column {
        self.columns.get(col_ix).unwrap()
    }

    fn render_th(&self, col_ix: usize, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let col = self.column(col_ix, cx);

        div().child(format!("{}", col.clone().name)).w_full()
    }

    fn render_tr(&self, row_ix: usize, _window: &mut Window, _cx: &mut App) -> Stateful<Div> {
        div().id(row_ix).on_click(move |ev, _, _| {
            println!(
                "Row {} clicked\nSecondary: {}",
                row_ix,
                ev.modifiers().secondary()
            )
        })
    }

    fn render_td(
        &self,
        row_ix: usize,
        col_ix: usize,
        _window: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement {
        let candidates = &self.rows[row_ix];
        let column = &self.columns[col_ix];

        match column.key.as_str() {
            "candidate-name" => div()
                .font_weight(FontWeight::BOLD)
                .min_w_32()
                .child(candidates.fullname()),
            "designation" => div()
                .font_weight(FontWeight::BOLD)
                .min_w_40()
                .child(candidates.position()),
            "candidate-status" => div()
                .font_weight(FontWeight::BOLD)
                .min_w_20()
                .child(candidates.status().to_string()),
            "candidate-email" => div()
                .font_weight(FontWeight::BOLD)
                .min_w_40()
                .child(candidates.email()),
            "candidate-number" => div()
                .font_weight(FontWeight::BOLD)
                .min_w_32()
                .child(candidates.number()),
            "applied-on" => div()
                .font_weight(FontWeight::BOLD)
                .min_w_32()
                .child(candidates.date_applied()),
            "actions" => div()
                .grid()
                .grid_cols(2)
                .gap_2()
                .child(
                    Button::new("view-candidate-table-action")
                        .custom(
                            ButtonCustomVariant::new(cx)
                                .color(cx.theme().primary.opacity(0.60))
                                .foreground(cx.theme().background)
                                .border(cx.theme().primary)
                                .hover(cx.theme().primary)
                                .active(cx.theme().primary),
                        )
                        .label("View")
                        .text_center()
                        .small()
                        .p_3()
                        .rounded_full(),
                )
                .child(
                    Button::new("notes-candidate-table-action")
                        .custom(
                            ButtonCustomVariant::new(cx)
                                .color(cx.theme().yellow.opacity(0.60))
                                .foreground(cx.theme().background)
                                .border(cx.theme().yellow)
                                .hover(cx.theme().yellow)
                                .active(cx.theme().yellow),
                        )
                        .label("Notes")
                        .text_center()
                        .small()
                        .p_3()
                        .rounded_full(),
                ),
            _ => div(),
        }
    }

    fn render_empty(&self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex()
            .justify_center()
            .items_center()
            .h_full()
            .w_full()
            .child(
                div()
                    .p_4()
                    .bg(cx.theme().accent)
                    .rounded_lg()
                    .child("No Data Available"),
            )
    }

    fn perform_sort(
        &mut self,
        col_ix: usize,
        sort: gpui_component::table::ColumnSort,
        _window: &mut Window,
        _cx: &mut Context<gpui_component::table::TableState<Self>>,
    ) {
        match self.columns[col_ix].key.as_ref() {
            "candidate-name" => self.rows.sort_by(|a, b| match sort {
                ColumnSort::Descending => b.fullname().cmp(&a.fullname()),
                _ => a.fullname().cmp(&b.fullname()),
            }),
            "designation" => self.rows.sort_by(|a, b| match sort {
                ColumnSort::Descending => b.position().cmp(&a.position()),
                _ => a.position().cmp(&b.position()),
            }),
            "candidate-status" => self.rows.sort_by(|a, b| match sort {
                ColumnSort::Descending => b.status().to_string().cmp(&a.status().to_string()),
                _ => a.status().to_string().cmp(&b.status().to_string()),
            }),
            "candidate-email" => self.rows.sort_by(|a, b| match sort {
                ColumnSort::Descending => b.email().cmp(&a.email()),
                _ => a.email().cmp(&b.email()),
            }),
            _ => {}
        }
    }

    // fn load_more(
    //     &mut self,
    //     window: &mut Window,
    //     cx: &mut Context<gpui_component::table::TableState<Self>>,
    // ) {
    //     if self.loading {
    //         return;
    //     }

    //     self.loading = true;

    //     cx.spawn(async move |view, cx| {
    //         let new_data = fetch_more_candidate_data().await;

    //         cx.update(|cx| {
    //             view.update(cx, |view, _| {
    //                 let delegate = view.table.delegate_mut();
    //                 delegate.data.extend(new_data);
    //                 delegate.loading = false;
    //                 delegate.has_more_data = !new_data.is_empty();
    //             });
    //         })
    //     })
    //     .detach();
    // }

    fn loading(&self, _cx: &App) -> bool {
        self.loading
    }

    fn load_more_threshold(&self) -> usize {
        110
    }

    fn visible_rows_changed(
        &mut self,
        visible_range: Range<usize>,
        _window: &mut Window,
        _cx: &mut Context<gpui_component::table::TableState<Self>>,
    ) {
        self.visible_rows = visible_range;
    }
}

impl TableModeView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let delegate = TableModeViewDelegate::new();
        let table_state = cx.new(|cx| TableState::new(delegate, window, cx).sortable(true));

        Self { table_state }
    }
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Render for TableModeView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .bg(cx.theme().accent)
            .rounded_xl()
            .size_full()
            .child(Table::new(&self.table_state.clone()).stripe(true))
    }
}
