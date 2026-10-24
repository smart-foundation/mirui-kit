//! Small/medium data table composition. Consumer supplies a horizontal scroll
//! container when columns exceed available width. Virtualized/editable grid is
//! a separate contract, not implied by this read-only cell renderer.
use crate::behavior::{
    collection::{ItemId, Modifiers, SelectionMode},
    table::{Cell, Column, TableError, TableRow, TableState},
};
use crate::{
    Actions, FocusItem, font, style, theme,
    widgets::{FocusRing, text_size},
};
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use mirui::prelude::*;
use mirui::ui::widgets::{Button, Text};
#[model]
pub struct DataTable {
    state: TableState,
    #[observe]
    pub revision: u64,
}
impl DataTable {
    pub fn new(
        columns: Vec<Column>,
        rows: Vec<TableRow>,
        mode: SelectionMode,
    ) -> Result<Self, TableError> {
        Ok(Self {
            state: TableState::new(columns, rows, mode)?,
            revision: 0,
        })
    }
}
#[model]
impl DataTable {
    pub fn columns(&self) -> Vec<Column> {
        self.state.columns().to_vec()
    }
    pub fn rows(&self) -> Vec<TableRow> {
        self.state.ordered_rows()
    }
    pub fn selected(&self, id: ItemId) -> bool {
        self.state.selection.contains(id)
    }
    pub fn focused(&self, id: ItemId) -> bool {
        self.state.selection.focus() == Some(id)
    }
    pub fn sort(&mut self, id: ItemId) {
        self.state.toggle_sort(id, false);
        self.revision = self.revision.wrapping_add(1);
    }
    pub fn choose(&mut self, id: ItemId) {
        self.state
            .selection
            .choose(id, &self.state.available(), Modifiers::default());
        self.revision = self.revision.wrapping_add(1);
    }
    pub fn move_focus(&mut self, delta: i32, modifiers: Modifiers) {
        let ids = self.state.available();
        if ids.is_empty() {
            return;
        }
        let i = self
            .state
            .selection
            .focus()
            .and_then(|id| ids.iter().position(|x| *x == id))
            .unwrap_or(0);
        let next = if delta < 0 {
            i.saturating_sub(1)
        } else {
            (i + 1).min(ids.len() - 1)
        };
        if modifiers.shift {
            self.state.selection.choose(ids[next], &ids, modifiers);
        } else {
            self.state.selection.focus_only(ids[next], &ids);
        }
        self.revision = self.revision.wrapping_add(1);
    }
    pub fn toggle_focused(&mut self) {
        let ids = self.state.available();
        if let Some(id) = self.state.selection.focus() {
            self.state.selection.choose(
                id,
                &ids,
                Modifiers {
                    ctrl: true,
                    shift: false,
                },
            );
            self.revision = self.revision.wrapping_add(1);
        }
    }
    pub fn replace_rows(&mut self, rows: Vec<TableRow>) -> Result<(), TableError> {
        self.state.replace_rows(rows)?;
        self.revision = self.revision.wrapping_add(1);
        Ok(())
    }
    pub fn resize(&mut self, id: ItemId, width: u32) {
        self.state.resize(id, width);
        self.revision = self.revision.wrapping_add(1);
    }
}
#[compose]
pub fn data_table(
    actions: model!(Actions),
    table: model!(DataTable),
    key: u16,
    overlay: bool,
) -> Entity {
    ui! {Column (row_gap:2,border_width:2,border_color:theme::CLEAR,border_radius:style::RADIUS) [Focusable,FocusItem{overlay,order:key},FocusRing{actions:actions.clone()}] {
        Row (column_gap:1) {
            walk ${ {let _=table.revision();table.columns()} } with column by column.id {
                Button (width:column.width.min(i32::MAX as u32) as i32,height:40,normal_color:ColorToken::SurfaceVariant,pressed_color:theme::PRESSED,padding:Padding::all(8)) {
                    Text (text:column.label.clone(),font:font::SEMIBOLD,font_size:${text_size(actions.text_scale(),font::BODY_SIZE)},text_color:ColorToken::OnSurface)
                } on Tap {if actions.modal()==overlay {table.sort(column.id);}}
            }
        }
        walk ${ {let _=table.revision();table.rows()} } with row by row.id {
            table_row (actions,table,row,overlay)
        }
        Text ("No rows",visible:${ {let _=table.revision();table.rows().is_empty()} },font:font::MEDIUM,font_size:font::BODY_SIZE,text_color:ColorToken::OnSurfaceVariant)
    } on Key(|event:event!(Key)| {
        if !event.pressed || actions.modal()!=overlay {return false;}
        let modifiers=Modifiers{ctrl:event.modifiers.is_some_and(|m|m.control),shift:event.modifiers.is_some_and(|m|m.shift)};
        match event.physical {
            Some(PhysicalKey::ArrowUp)=>{table.move_focus(-1,modifiers);true},
            Some(PhysicalKey::ArrowDown)=>{table.move_focus(1,modifiers);true},
            Some(PhysicalKey::Space) if event.repeat!=Some(true)=>{table.toggle_focused();true},_=>false,
        }
    }) on FocusChanged(|event:event!(FocusChanged)| {actions.focus(key,event.focused);})}
}
#[compose]
fn table_row(
    actions: model!(Actions),
    table: model!(DataTable),
    row: TableRow,
    overlay: bool,
) -> Entity {
    let columns = table.columns();
    let cells: Vec<(u32, String)> = row
        .cells
        .iter()
        .zip(columns.iter())
        .map(|(cell, column)| {
            (
                column.width,
                match cell {
                    Cell::Text(s) => s.clone(),
                    Cell::Integer(n) => n.to_string(),
                },
            )
        })
        .collect();
    ui! {Button (height:40,normal_color:${ {let _=table.revision();if table.selected(row.id) {theme::SELECTION} else {theme::CLEAR}} },pressed_color:theme::PRESSED,border_width:2,border_color:${ {let _=table.revision();if table.focused(row.id) {theme::FOCUS} else {theme::CLEAR}} },border_radius:style::RADIUS,padding:Padding::all(0)) {
        Row (column_gap:1) {
            walk cells with cell {
                View (width:cell.0.min(i32::MAX as u32) as i32,padding:Padding::all(8)) {
                    Text (text:cell.1.clone(),font:font::MEDIUM,font_size:${text_size(actions.text_scale(),font::BODY_SIZE)},text_color:if row.disabled {ColorToken::OnSurfaceVariant} else {ColorToken::OnSurface})
                }
            }
        }
    } on Tap {if actions.modal()==overlay {table.choose(row.id);}}}
}
