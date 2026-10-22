//! The content area: an icon grid and a virtualised list.

use mirui::prelude::*;
use mirui::ui::widgets::lazy_list::CreateCell;
use mirui::ui::widgets::{Button, LazyList, ListItem, ParagraphStyle, Text};

use crate::model::{DropTarget, FmState, GRID_COLUMNS, ViewMode};
use crate::theme;
use crate::ui::kindicon::{kind_icon, kind_icon_at};
use crate::ui::kit::{pad, pad_h, pad_v};

/// Marks a tile as a drop target, and says which file it holds.
#[derive(Clone, Copy, Debug)]
pub struct DropZone {
    pub position: u32,
    pub folder: bool,
    pub place: Option<crate::model::Place>,
}

impl mirui::ecs::Component for DropZone {}

/// The content pane.
#[compose]
pub fn area(fm: model!(FmState)) -> Entity {
    ui! {
        Column (
            grow: 1.0,
            shrink: 1.0,
            bg_color: ColorToken::Surface
        ) {
            if ${ fm.view() == ViewMode::Icons } {
                icon_grid (fm)
            } else {
                list_view (fm)
            }
        }
    }
}

/// Icon grid: fixed columns of tiles, one row per line.
#[compose]
fn icon_grid(fm: model!(FmState)) -> Entity {
    ui! {
        Scroll (
            width: Dimension::percent(100),
            grow: 1.0,
            padding: pad(16, 16, 16, 16)
        ) {
            Column (
                width: Dimension::percent(100),
                height: ${ Dimension::px(fm.grid_height()) },
                row_gap: 10
            ) {
                walk ${ { let _ = fm.revision(); fm.grid_row_indexes() } } with row {
                    grid_row (fm, row)
                }
            }
        }
    }
}

/// The column positions of the icon grid.
#[allow(dead_code)]
const COLUMN_INDEXES: [u32; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

/// One row of tiles.
#[compose]
fn grid_row(fm: model!(FmState), row: u32) -> Entity {
    ui! {
        Row (width: Dimension::percent(100), height: 98, column_gap: 6) {
            walk 0..GRID_COLUMNS with column {
                cell (fm, row * GRID_COLUMNS + column)
            }
        }
    }
}

/// One file tile.
#[compose]
fn cell(fm: model!(FmState), position: u32) -> Entity {
    let id = fm.entry_id(position);
    let folder = fm.entry_is_kind(id, 0);
    let icon_size = fm.icon_size();
    ui! {
        Button (
            width: 104,
            height: 98,
            border_radius: 8,
            padding: pad_v(0),
            normal_color: ${ if fm.drop_target() == Some(DropTarget::Tile(position)) { theme::DROP } else if fm.selected() == Some(id) { theme::SELECTION } else { theme::CLEAR } },
            pressed_color: theme::SELECTION
        ) [
            DropZone { position, folder, place: None }
        ] {
            Column (
                width: Dimension::percent(100),
                height: Dimension::percent(100),
                align: AlignItems::Center,
                justify: JustifyContent::Center,
                row_gap: 6
            ) {
                kind_icon (fm, id, icon_size as i32)
                Text (
                    text: ${ { let _ = fm.revision(); fm.entry_title(id) } },
                    font_size: 11,
                    text_color: ColorToken::OnSurface,
                    paragraph: ParagraphStyle::label()
                )
            }
        } on Tap {
            fm.select(id);
        } on Tap(2) {
            fm.select(id);
            fm.open_dialog(crate::model::DialogKind::GetInfo);
        } on LongPress {
            fm.open_menu(id, event.x.to_int(), event.y.to_int());
        } on DragStart {
            fm.start_drag(id, event.x.to_int(), event.y.to_int());
        } on DragMove {
            // A drag inside a scrolling area is claimed by the scroll, so the
            // move is what reliably starts one.
            if fm.dragging().is_none() {
                fm.start_drag(id, event.x.to_int(), event.y.to_int());
            } else {
                fm.drag_to(event.x.to_int(), event.y.to_int());
            }
        } on DragEnd {
            fm.end_drag();
        }
    }
}

/// Virtualised list: only the visible rows exist as entities.
#[compose]
fn list_view(fm: model!(FmState)) -> Entity {
    ui! {
        Column (width: Dimension::percent(100), grow: 1.0) {
            LazyList (
                width: Dimension::percent(100),
                grow: 1.0,
                items: ${ { let _ = fm.revision(); fm.list_count() } },
                item_extent: Fixed::from_int(26)
            ) on CreateCell(create_row::callback(fm))
        }
    }
}

/// Builds one recycled list row.
#[event]
fn create_row(fm: model!(FmState), event: event!(CreateCell)) -> Entity {
    let item = event.item.clone();
    ui! {
        row_cell (fm, item)
    }
}

/// A single row of the virtualised list.
#[compose]
fn row_cell(fm: model!(FmState), item: model!(ListItem)) -> Entity {
    ui! {
        Button (
            width: Dimension::percent(100),
            height: 26,
            padding: pad_v(0),
            normal_color: ${ { let _ = fm.revision(); if fm.selected() == Some(fm.entry_id(item.index())) { theme::SELECTION } else { theme::CLEAR } } },
            pressed_color: theme::SELECTION
        ) {
            Row (
                width: Dimension::percent(100),
                height: 26,
                align: AlignItems::Center,
                column_gap: 8,
                padding: pad_h(12)
            ) {
                kind_icon_at (fm, item, 14)
                Text (
                    text: ${ { let _ = fm.revision(); fm.entry_label(item.index()) } },
                    font_size: 12,
                    text_color: ColorToken::OnSurface,
                    grow: 1.0
                )
                Text (
                    text: ${ { let _ = fm.revision(); fm.entry_kind_at(item.index()) } },
                    font_size: 11,
                    text_color: ColorToken::OnSurfaceVariant,
                    width: 78
                )
                Text (
                    text: ${ { let _ = fm.revision(); fm.entry_size_at(item.index()) } },
                    font_size: 11,
                    text_color: ColorToken::OnSurfaceVariant,
                    width: 72
                )
                Text (
                    text: ${ { let _ = fm.revision(); fm.entry_date_at(item.index()) } },
                    font_size: 11,
                    text_color: ColorToken::OnSurfaceVariant,
                    width: 80
                )
            }
        } on Tap {
            fm.select(fm.entry_id(item.index()));
        }
    }
}
