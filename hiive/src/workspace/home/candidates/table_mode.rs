use std::ops::Range;

use gpui::*;
use gpui_component::{
    ActiveTheme, IndexPath, Sizable, WindowExt,
    checkbox::Checkbox,
    h_flex,
    label::Label,
    select::{Select, SelectItem, SelectState},
    table::{Column, ColumnSort, Table, TableDelegate, TableEvent, TableState},
    v_flex,
};

use crate::data::{
    home::candidates_data::Candidatesdata, models::candidates_model::card_mode::CandidatesCardModel,
};

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

pub struct TableModeView {
    table_state: Entity<TableState<TableModeViewDelegate>>,
    rows_per_page_state: Entity<SelectState<Vec<RowsPerPage>>>,
}

struct TableModeViewDelegate {
    rows: Vec<CandidatesCardModel>,
    columns: Vec<Column>,
    loading: bool,
    visible_rows: Range<usize>,
    select_all: bool,
}

impl TableModeViewDelegate {
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            columns: Vec::new(),
            loading: false,
            visible_rows: Range::default(),
            select_all: false,
        }
    }

    pub fn update(&mut self, data: Vec<CandidatesCardModel>) {
        let columns = vec![
            Column::new("checkbox", "").width(40.0).movable(false),
            Column::new("candidate-name", "Candidate Name")
                .width(220.0)
                .resizable(true)
                .movable(false)
                .sortable(),
            Column::new("designation", "Designation")
                .width(325.0)
                .resizable(true)
                .movable(false)
                .sortable(),
            Column::new("candidate-status", "Status")
                .width(190.0)
                .resizable(true)
                .movable(false)
                .sortable(),
            Column::new("candidate-email", "Email")
                .width(280.0)
                .resizable(true)
                .movable(false)
                .sortable(),
            Column::new("candidate-number", "Mobile Number")
                .width(220.0)
                .resizable(true)
                .movable(false)
                .sortable(),
            Column::new("applied-on", "Applied On")
                .width(160.0)
                .resizable(true)
                .movable(false)
                .sortable(),
        ];

        self.rows = data;
        self.columns = columns;
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

        match col.key.as_ref() {
            "checkbox" => {
                div().child(Checkbox::new("select_all_checkbox").checked(self.select_all))
            }
            _ => div().child(col.name.to_string()),
        }
    }

    fn render_tr(&self, row_ix: usize, _window: &mut Window, _cx: &mut App) -> Stateful<Div> {
        h_flex().id(row_ix).h_16().py_2()
    }

    fn render_td(
        &self,
        row_ix: usize,
        col_ix: usize,
        _window: &mut Window,
        _cx: &mut App,
    ) -> impl IntoElement {
        let candidates = &self.rows[row_ix];
        let column = &self.columns[col_ix];

        match column.key.as_str() {
            "checkbox" => div().child(Checkbox::new("row_checkbox").checked(false)),
            "candidate-name" => h_flex()
                .font_weight(FontWeight::BOLD)
                .gap_1()
                .child(
                    img(candidates.src())
                        .h_8()
                        .w_8()
                        .rounded_full()
                        .overflow_hidden()
                        .object_fit(ObjectFit::Cover),
                )
                .child(candidates.fullname()),
            "designation" => div()
                .font_weight(FontWeight::BOLD)
                .min_w_40()
                .child(candidates.position()),
            "candidate-status" => div()
                .font_weight(FontWeight::BOLD)
                .rounded_full()
                .px_3()
                .py_1()
                .text_center()
                .bg(candidates.status().color().opacity(0.20))
                .child(candidates.status().to_string().to_uppercase()),
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
            _ => div(),
        }
    }

    fn render_empty(&self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex().justify_center().h_full().w_full().p_3().child(
            div()
                .bg(cx.theme().accent)
                .rounded_xl()
                .shadow_lg()
                .child("No Data available"),
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

    // fn load_more(&mut self, window: &mut Window, cx: &mut Context<gpui_component::table::TableState<Self>>) {
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
    //     }).detach();
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

        cx.subscribe_in(&table_state, window, |_view, _table, event, window, cx| {
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
        })
        .detach();

        Self {
            table_state,
            rows_per_page_state,
        }
    }
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn render_table_mode(&self, window: &mut Window, cx: &mut Context<Self>) -> Div {
        let data = Candidatesdata::data();

        self.table_state.update(cx, |table, cx| {
            table.delegate_mut().update(data.clone());
            table.loop_selection = true;
            table.col_resizable = true;
            table.sortable = true;
            table.col_movable = false;
            table.refresh(cx);

            cx.notify();
        });

        v_flex()
            .gap_5()
            .child(
                v_flex().bg(cx.theme().accent).h(px(620.)).child(
                    Table::new(&self.table_state.clone())
                        .with_size(px(50.))
                        .stripe(true)
                        .bordered(true)
                        .scrollbar_visible(true, true),
                ),
            )
            .child(self.render_table_status(window, cx))
    }

    fn render_table_status(&self, _window: &mut Window, cx: &mut Context<Self>) -> Div {
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
}

impl Render for TableModeView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .rounded_xl()
            .child(self.render_table_mode(window, cx))
    }
}
