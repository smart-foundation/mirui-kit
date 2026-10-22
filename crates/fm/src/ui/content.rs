//! The content area: an icon grid and a virtualised list.
//!
//! A tile is built once and kept: its icon and its name are static, so a
//! selection costs one repaint instead of a re-layout of the whole grid. The
//! grid is keyed by the entry id, so a change of the listing rebuilds exactly
//! the tiles whose file changed.

use mirui::prelude::*;
use mirui::ui::widgets::lazy_list::CreateCell;
use mirui::ui::widgets::{Icon, LazyList, ListItem, ParagraphStyle, Text};

use crate::data::Kind;
use crate::model::{DialogKind, DropTarget, FmState, GRID_COLUMNS, ViewMode};
use crate::text;
use crate::theme;
use crate::ui::kindicon::kind_icon_at;
use crate::ui::kit::{pad, pad_h, pad_v};

/// Marks a tile as a drop target, and says which file it holds.
#[derive(Clone, Copy, Debug)]
pub struct DropZone {
    pub id: u32,
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
        Column (
            width: Dimension::percent(100),
            grow: 1.0,
            padding: pad(16, 16, 16, 16)
        ) {
            Column (
                width: Dimension::percent(100),
                height: ${ Dimension::px(fm.grid_height()) },
                row_gap: 10
            ) {
                // Keyed by a checksum of the row: any change to the listing
                // rebuilds the rows whose files changed.
                walk ${ { let _ = fm.listing_revision(); fm.grid_row_indexes() } } with row by fm.grid_row_key(row) {
                    grid_row (fm, row)
                }
            }
        }
    }
}

/// The column positions of the icon grid.
const COLUMN_INDEXES: [u32; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

/// One row of tiles.
#[compose]
fn grid_row(fm: model!(FmState), row: u32) -> Entity {
    ui! {
        Row (width: Dimension::percent(100), height: 98, column_gap: 6) {
            walk COLUMN_INDEXES with column {
                cell (fm, row * GRID_COLUMNS + column)
            }
        }
    }
}

/// One file tile.
#[compose]
fn cell(fm: model!(FmState), position: u32) -> Entity {
    let id = fm.grid_entry(position);
    let kind = fm.entry_kind(id);
    ui! {
        View (
            width: 104,
            height: 98,
            border_radius: 8,
            padding: pad_v(0),
            bg_color: ${ if fm.drop_target() == Some(DropTarget::Tile(id)) { theme::DROP } else if fm.selected() == Some(id) { theme::SELECTION } else { theme::CLEAR } }
        ) [
            HitTarget
            InteractionFeedback
            DropZone { id, folder: kind == Kind::Folder, place: None }
        ] {
            Column (
                width: Dimension::percent(100),
                height: Dimension::percent(100),
                align: AlignItems::Center,
                justify: JustifyContent::Center,
                row_gap: 6
            ) {
                Icon (
                    path: kind.icon().clone(),
                    color: kind.tint(),
                    width: ${ fm.icon_size() as i32 },
                    height: ${ fm.icon_size() as i32 }
                )
                Text (
                    text: ${ { let _ = fm.names_revision(); fm.entry_title(id) } },
                    width: Dimension::percent(100),
                    font_size: text::SMALL,
                    text_color: ColorToken::OnSurface,
                    paragraph: ParagraphStyle::label()
                )

            }
        } on Tap {
            fm.select(id);
        } on Tap(2) {
            fm.select(id);
            fm.open_dialog(DialogKind::GetInfo);
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
                items: ${ { let _ = fm.listing_revision(); fm.list_count() } },
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
        View (
            width: Dimension::percent(100),
            height: 26,
            padding: pad_v(0),
            bg_color: ${ { let _ = fm.listing_revision(); if fm.selected() == Some(fm.entry_id(item.index())) { theme::SELECTION } else { theme::CLEAR } } }
        ) [
            HitTarget
            InteractionFeedback
        ] {
            Row (
                width: Dimension::percent(100),
                height: 26,
                align: AlignItems::Center,
                column_gap: 8,
                padding: pad_h(12)
            ) {
                kind_icon_at (fm, item, 14)
                Text (
                    text: ${ { let _ = fm.names_revision(); fm.entry_label(item.index()) } },
                    font_size: text::BODY,
                    text_color: ColorToken::OnSurface,
                    grow: 1.0
                )
                Text (
                    text: ${ { let _ = fm.listing_revision(); fm.entry_kind_at(item.index()) } },
                    font_size: text::SMALL,
                    text_color: ColorToken::OnSurfaceVariant,
                    width: 74
                )
                Text (
                    text: ${ { let _ = fm.listing_revision(); fm.entry_size_at(item.index()) } },
                    font_size: text::SMALL,
                    text_color: ColorToken::OnSurfaceVariant,
                    width: 68
                )
                Text (
                    text: ${ { let _ = fm.listing_revision(); fm.entry_date_at(item.index()) } },
                    font_size: text::SMALL,
                    text_color: ColorToken::OnSurfaceVariant,
                    width: 76
                )
            }
        } on Tap {
            fm.select(fm.entry_id(item.index()));
        } on Tap(2) {
            fm.select(fm.entry_id(item.index()));
            fm.open_dialog(DialogKind::GetInfo);
        }
    }
}
