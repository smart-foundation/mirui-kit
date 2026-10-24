use super::collection::{ItemId, Selection, SelectionMode, SortDirection, SortOrder};
use alloc::{collections::BTreeSet, string::String, vec::Vec};
use core::cmp::Ordering;
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Cell {
    Text(String),
    Integer(i64),
}
impl Cell {
    pub fn compare(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Integer(a), Self::Integer(b)) => a.cmp(b),
            (Self::Text(a), Self::Text(b)) => a.cmp(b),
            (Self::Integer(_), _) => Ordering::Less,
            (_, Self::Integer(_)) => Ordering::Greater,
        }
    }
}
#[derive(Clone, Debug)]
pub struct Column {
    pub id: ItemId,
    pub label: String,
    pub width: u32,
    pub minimum: u32,
    pub maximum: u32,
    pub sortable: bool,
}
#[derive(Clone, Debug)]
pub struct TableRow {
    pub id: ItemId,
    pub cells: Vec<Cell>,
    pub disabled: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TableError {
    DuplicateColumn(ItemId),
    DuplicateRow(ItemId),
    InvalidWidth(ItemId),
    CellCount(ItemId),
}
#[derive(Clone, Debug)]
pub struct TableState {
    columns: Vec<Column>,
    rows: Vec<TableRow>,
    pub sort: SortOrder,
    pub selection: Selection,
}
impl TableState {
    pub fn new(
        columns: Vec<Column>,
        rows: Vec<TableRow>,
        mode: SelectionMode,
    ) -> Result<Self, TableError> {
        let mut ids = BTreeSet::new();
        for c in &columns {
            if !ids.insert(c.id) {
                return Err(TableError::DuplicateColumn(c.id));
            }
            if c.minimum > c.maximum || c.width < c.minimum || c.width > c.maximum {
                return Err(TableError::InvalidWidth(c.id));
            }
        }
        Self::validate_rows(&rows, columns.len())?;
        let mut state = Self {
            columns,
            rows,
            sort: SortOrder::default(),
            selection: Selection::new(mode),
        };
        state.selection.reconcile(&state.available());
        Ok(state)
    }
    fn validate_rows(rows: &[TableRow], columns: usize) -> Result<(), TableError> {
        let mut ids = BTreeSet::new();
        for row in rows {
            if !ids.insert(row.id) {
                return Err(TableError::DuplicateRow(row.id));
            }
            if row.cells.len() != columns {
                return Err(TableError::CellCount(row.id));
            }
        }
        Ok(())
    }
    pub fn columns(&self) -> &[Column] {
        &self.columns
    }
    /// Stable-ID lookup without sorting or cloning the collection.
    pub fn row(&self, id: ItemId) -> Option<&TableRow> {
        self.rows.iter().find(|row| row.id == id)
    }
    pub fn ordered_rows(&self) -> Vec<TableRow> {
        let mut rows = self.rows.clone();
        rows.sort_by(|a, b| {
            for (id, direction) in &self.sort.0 {
                if let Some(i) = self.columns.iter().position(|c| c.id == *id) {
                    let order = a.cells[i].compare(&b.cells[i]);
                    let order = if *direction == SortDirection::Descending {
                        order.reverse()
                    } else {
                        order
                    };
                    if order != Ordering::Equal {
                        return order;
                    }
                }
            }
            Ordering::Equal
        });
        rows
    }
    pub fn available(&self) -> Vec<ItemId> {
        self.ordered_rows()
            .iter()
            .filter(|r| !r.disabled)
            .map(|r| r.id)
            .collect()
    }
    pub fn toggle_sort(&mut self, id: ItemId, additive: bool) {
        if self.columns.iter().any(|c| c.id == id && c.sortable) {
            self.sort.toggle(id, additive);
        }
    }
    pub fn resize(&mut self, id: ItemId, width: u32) {
        if let Some(c) = self.columns.iter_mut().find(|c| c.id == id) {
            c.width = width.clamp(c.minimum, c.maximum);
        }
    }
    pub fn replace_rows(&mut self, rows: Vec<TableRow>) -> Result<(), TableError> {
        Self::validate_rows(&rows, self.columns.len())?;
        self.rows = rows;
        self.selection.reconcile(&self.available());
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::super::collection::Modifiers;
    use super::*;
    use alloc::vec;
    fn table() -> TableState {
        TableState::new(
            vec![Column {
                id: 1,
                label: "Size".into(),
                width: 100,
                minimum: 40,
                maximum: 300,
                sortable: true,
            }],
            vec![
                TableRow {
                    id: 1,
                    cells: vec![Cell::Integer(100)],
                    disabled: false,
                },
                TableRow {
                    id: 2,
                    cells: vec![Cell::Integer(9)],
                    disabled: false,
                },
            ],
            SelectionMode::Multiple,
        )
        .unwrap()
    }
    #[test]
    fn numeric_sort_not_lexical() {
        let mut t = table();
        t.toggle_sort(1, false);
        assert_eq!(t.ordered_rows()[0].id, 2);
        t.toggle_sort(1, false);
        assert_eq!(t.ordered_rows()[0].id, 1);
        t.toggle_sort(1, false);
        assert_eq!(t.ordered_rows()[0].id, 1);
    }
    #[test]
    fn sorting_preserves_stable_selection() {
        let mut t = table();
        t.selection.choose(1, &[1, 2], Modifiers::default());
        t.toggle_sort(1, false);
        assert!(t.selection.contains(1));
    }
    #[test]
    fn widths_are_bounded() {
        let mut t = table();
        t.resize(1, u32::MAX);
        assert_eq!(t.columns()[0].width, 300);
        t.resize(1, 0);
        assert_eq!(t.columns()[0].width, 40);
    }
    #[test]
    fn bad_replacement_is_atomic() {
        let mut t = table();
        assert_eq!(
            t.replace_rows(vec![TableRow {
                id: 7,
                cells: vec![],
                disabled: false
            }]),
            Err(TableError::CellCount(7))
        );
        assert_eq!(t.ordered_rows().len(), 2);
    }
    #[test]
    fn removing_selection_recovers_focus() {
        let mut t = table();
        t.selection.choose(1, &[1, 2], Modifiers::default());
        let row = t.ordered_rows().pop().unwrap();
        t.replace_rows(vec![row]).unwrap();
        assert_eq!(t.selection.focus(), Some(2));
        assert!(t.selection.selected().is_empty());
    }
}
