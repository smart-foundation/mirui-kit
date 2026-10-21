//! The content area: an icon grid and a virtualised list.

use mirui::prelude::*;
use mirui::ui::widgets::lazy_list::CreateCell;
use mirui::ui::widgets::{Button, Icon, LazyList, ListItem, ParagraphStyle, Text};

use crate::data;
use crate::icons;
use crate::model::{FmState, ViewMode};
use crate::theme;
use crate::ui::kit::pad;

/// The content pane.
#[compose]
pub fn area(fm: model!(FmState)) -> Entity {
    ui! {
        Column (
            grow: 1.0,
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

/// Icon grid: one tile per file, wrapped over several rows.
#[compose]
fn icon_grid(fm: model!(FmState)) -> Entity {
    ui! {
        Scroll (
            width: Dimension::percent(100),
            grow: 1.0,
            padding: pad(16, 16, 16, 16)
        ) {
            Row (
                width: Dimension::percent(100),
                wrap: FlexWrap::Wrap,
                column_gap: 6,
                row_gap: 10
            ) {
                // Keyed by the entry index: when the place or the filter
                // changes, the keys change with it and the tiles are rebuilt
                // instead of being reused with stale content.
                walk ${ { let _ = fm.revision(); fm.grid_indices() } } with index by index {
                    cell (fm, index)
                }
            }
        }
    }
}

/// One file tile.
#[compose]
fn cell(fm: model!(FmState), index: u32) -> Entity {
    let entry = data::entry(index as usize);
    ui! {
        Button (
            width: 104,
            height: 98,
            border_radius: 8,
            padding: crate::ui::kit::pad_v(0),
            normal_color: ${ if fm.selected() == Some(entry.id) { theme::SELECTION } else { theme::CLEAR } },
            pressed_color: theme::SELECTION
        ) {
            Column (
                width: Dimension::percent(100),
                height: Dimension::percent(100),
                align: AlignItems::Center,
                justify: JustifyContent::Center,
                row_gap: 6
            ) {
                Icon (path: entry.kind.icon().clone(), color: entry.kind.tint(), width: 42, height: 42)
                Text (
                    text: entry.display_name(),
                    font_size: 11,
                    text_color: ColorToken::OnSurface,
                    paragraph: ParagraphStyle::label()
                )
            }
        } on Tap {
            fm.select(entry.id);
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

/// The icon of a recycled row.
///
/// A recycled cell can end up on any entry, and an icon path is not a reactive
/// property, so every kind gets its own icon and only the matching one stays
/// visible.
#[compose]
fn kind_icon(fm: model!(FmState), item: model!(ListItem)) -> Entity {
    ui! {
        Row (width: 14, height: 14) {
            Icon (path: icons::FOLDER.clone(), color: data::Kind::Folder.tint(), width: 14, height: 14, visible: ${ { let _ = fm.revision(); fm.entry_is_kind(item.index(), 0) } })
            Icon (path: icons::DOCUMENT.clone(), color: data::Kind::Document.tint(), width: 14, height: 14, visible: ${ { let _ = fm.revision(); fm.entry_is_kind(item.index(), 1) } })
            Icon (path: icons::IMAGE.clone(), color: data::Kind::Image.tint(), width: 14, height: 14, visible: ${ { let _ = fm.revision(); fm.entry_is_kind(item.index(), 2) } })
            Icon (path: icons::CODE.clone(), color: data::Kind::Code.tint(), width: 14, height: 14, visible: ${ { let _ = fm.revision(); fm.entry_is_kind(item.index(), 3) } })
            Icon (path: icons::ARCHIVE.clone(), color: data::Kind::Archive.tint(), width: 14, height: 14, visible: ${ { let _ = fm.revision(); fm.entry_is_kind(item.index(), 4) } })
            Icon (path: icons::AUDIO.clone(), color: data::Kind::Audio.tint(), width: 14, height: 14, visible: ${ { let _ = fm.revision(); fm.entry_is_kind(item.index(), 5) } })
        }
    }
}

/// A single row of the virtualised list.
#[compose]
fn row_cell(fm: model!(FmState), item: model!(ListItem)) -> Entity {
    ui! {
        Button (
            width: Dimension::percent(100),
            height: 26,
            padding: crate::ui::kit::pad_v(0),
            normal_color: ${ if fm.selected() == Some(fm.entry_id(item.index())) { theme::SELECTION } else { theme::CLEAR } },
            pressed_color: theme::SELECTION
        ) {
            Row (
                width: Dimension::percent(100),
                height: 26,
                align: AlignItems::Center,
                column_gap: 8,
                padding: crate::ui::kit::pad_h(12)
            ) {
                kind_icon (fm, item)
                Text (
                    text: ${ { let _ = fm.revision(); fm.entry_label(item.index()) } },
                    font_size: 12,
                    text_color: ColorToken::OnSurface,
                    grow: 1.0
                )
                Text (
                    text: ${ { let _ = fm.revision(); fm.entry_kind(item.index()) } },
                    font_size: 11,
                    text_color: ColorToken::OnSurfaceVariant,
                    width: 78
                )
                Text (
                    text: ${ { let _ = fm.revision(); fm.entry_size(item.index()) } },
                    font_size: 11,
                    text_color: ColorToken::OnSurfaceVariant,
                    width: 72
                )
                Text (
                    text: ${ { let _ = fm.revision(); fm.entry_date(item.index()) } },
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
