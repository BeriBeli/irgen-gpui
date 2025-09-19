use gpui::*;
use gpui_component::table::{Column, Table, TableDelegate};

struct Row {
    id: String,
    name: String,
}

impl Row {
    fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
        }
    }
}

struct RegFieldDelegate {
    columns: Vec<Column>,
    rows: Vec<Row>,
}

impl RegFieldDelegate {
    fn new() -> Self {
        Self {
            columns: vec![Column::new("id", "ID"), Column::new("name", "Name")],
            rows: vec![Row::new("1", "2"), Row::new("3", "4")],
        }
    }
}

impl TableDelegate for RegFieldDelegate {
    fn columns_count(&self, _cx: &App) -> usize {
        self.columns.len()
    }
    fn rows_count(&self, _cx: &App) -> usize {
        self.rows.len()
    }
    fn column(&self, col_ix: usize, _cx: &App) -> &Column {
        &self.columns[col_ix]
    }
    fn render_th(
        &self,
        col_ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Table<Self>>,
    ) -> impl IntoElement {
        let col = self.columns.get(col_ix).unwrap();
        div().child(col.name.clone())
    }
    fn render_tr(
        &self,
        row_ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Table<Self>>,
    ) -> Stateful<Div> {
        div().id(row_ix)
    }

    fn render_td(
        &self,
        row_ix: usize,
        col_ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Table<Self>>,
    ) -> impl IntoElement {
        let row = self.rows.get(row_ix).unwrap();
        let col = self.columns.get(col_ix).unwrap();

        match col.key.as_ref() {
            "id" => row.id.to_string().into_any_element(),
            "name" => row.name.to_string().into_any_element(),
            _ => "--".to_string().into_any_element(),
        }
    }
}

pub struct RegField {
    table: Entity<Table<RegFieldDelegate>>,
}

impl RegField {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let table = cx.new(|cx| Table::new(RegFieldDelegate::new(), window, cx));
        Self { table }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Render for RegField {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.table.clone()
    }
}
