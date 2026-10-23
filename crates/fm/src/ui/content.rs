//! The content area: an icon grid and a virtualised list.
//!
//! A tile is built once and kept: its icon and its name are static, so a
//! selection costs one repaint instead of a re-layout of the whole grid. The
//! grid is keyed by the entry id, so a change of the listing rebuilds exactly
//! the tiles whose file changed.
//!
//! The background of a tile and of a row is painted by the widget kit: the
//! `SelectedSurface` view draws the hover, the press and the selection, so
//! the tree carries no reactive background colour at all — and the file kind
//! is drawn by one `KindIcon` view instead of six icons with a reactive
//! visibility.

use alloc::vec::Vec;

use mirui::prelude::*;
use mirui::render::font::FontManager;
use mirui::ui::widgets::{ParagraphStyle, Placeholder, Text, TextEditing, TextInput};

use crate::data::Kind;
use crate::model::{DialogKind, DropTarget, FmState, TILE_HEIGHT, TILE_WIDTH, ViewMode};
use crate::text;
use crate::theme;
use crate::ui::controls::{self, FieldFocus};
use crate::ui::hover::{HoverSurface, KindIcon};
use crate::ui::scrollbar::ScrollBar;
use crate::ui::kit::{pad, pad_h, pad_v};

/// The size of the icon of a list row.
const ROW_ICON: i32 = 14;

/// Marks a tile as a drop target, and says which file it holds.
#[derive(Clone, Copy, Debug)]
pub struct DropZone {
    pub id: u32,
    pub folder: bool,
    pub place: Option<crate::model::Place>,
}

impl mirui::ecs::Component for DropZone {}

/// The content pane.
///
/// The scroller and its scroll bar live in the same branch, so the bar can
/// name the container it reports on.
#[compose]
pub fn area(fm: model!(FmState)) -> Entity {
    ui! {
        Column (
            container: true,
            grow: 1.0,
            shrink: 1.0,
            bg_color: ColorToken::Surface
        ) {
            if ${ fm.view() == ViewMode::Icons } {
                Scroll (
                    id: "grid_scroller",
                    width: Dimension::percent(100),
                    grow: 1.0,
                    padding: pad(16, 16, 16, 16)
                ) {
                    Column (
                        width: Dimension::percent(100),
                        height: ${ Dimension::px(fm.grid_height()) },
                        row_gap: 10
                    ) {
                        // Keyed by a checksum of the row: any change to the
                        // listing rebuilds the rows whose files changed.
                        walk ${ { let _ = fm.listing_revision(); if fm.view() == ViewMode::Icons { fm.grid_row_indexes() } else { Vec::new() } } } with row by fm.grid_row_key(row).wrapping_mul(1_000_003).wrapping_add(fm.icon_size() as u64) {
                            grid_row (fm, row)
                        }
                    }
                }
                View (
                    position: Position::Absolute,
                    left: @width { width - Fixed::from_int(9) },
                    top: 0,
                    width: 6,
                    height: @height { height }
                ) [
                    ScrollBar { target: id("grid_scroller"), vertical: true, thickness: 6 }
                ]
            } else {
                Scroll (
                    id: "list_scroller",
                    width: Dimension::percent(100),
                    grow: 1.0
                ) {
                    Column (width: Dimension::percent(100), row_gap: 0) {
                        walk ${ { let _ = fm.listing_revision(); if fm.view() == ViewMode::List { fm.row_indexes() } else { Vec::new() } } } with index {
                            row_cell (fm, index)
                        }
                    }
                }
                View (
                    position: Position::Absolute,
                    left: @width { width - Fixed::from_int(9) },
                    top: 0,
                    width: 6,
                    height: @height { height }
                ) [
                    ScrollBar { target: id("list_scroller"), vertical: true, thickness: 6 }
                ]
            }
        }
    }
}

/// One row of tiles.
///
/// The column count is derived from the available width so the grid reflows
/// as the window is resized instead of being locked to eight columns.
#[compose]
fn grid_row(fm: model!(FmState), row: u32) -> Entity {
    let columns = fm.grid_columns();
    ui! {
        Row (width: Dimension::percent(100), height: TILE_HEIGHT, column_gap: 6) {
            walk (0..columns).collect::<Vec<u32>>() with column {
                cell (fm, row * columns + column)
            }
        }
    }
}

/// One file tile.
#[compose]
fn cell(fm: model!(FmState), position: u32) -> Entity {
    let id = fm.grid_entry(position);
    let kind = fm.entry_kind(id);
    let size = fm.icon_size() as i32;
    let editing = fm.editing_folder_id() == Some(id);
    let text_height = res!(FontManager).with(|fonts| controls::field_height(&fonts));
    ui! {
        View (
            width: TILE_WIDTH,
            height: TILE_HEIGHT,
            border_radius: 8,
            padding: pad_v(0),
            bg_color: ${ if fm.selected() == Some(id) { theme::SELECTION } else { theme::CLEAR } }
        ) [
            HitTarget
            InteractionFeedback
            DropZone { id, folder: kind == Kind::Folder, place: None }
            HoverSurface {
                hover: theme::HOVER.into(),
                pressed: theme::PRESSED.into(),
                radius: Fixed::from_int(8),
            }
        ] {
            Column (
                width: Dimension::percent(100),
                height: Dimension::percent(100),
                align: AlignItems::Center,
                justify: JustifyContent::Center,
                row_gap: 6,
                border_radius: 8,
                // The drop target is drawn over the surface of the tile, so
                // it survives the hover of the pointer.
                bg_color: ${ if fm.drop_target() == Some(DropTarget::Tile(id)) { theme::DROP } else { theme::CLEAR } }
            ) {
                View (width: size, height: size) [
                    KindIcon {
                        model: fm.clone(),
                        index: position,
                        id: Some(id),
                        size,
                    }
                ]
                if editing {
                    TextInput (
                        id: "inline_rename",
                        width: Dimension::percent(100),
                        height: text_height,
                        font_size: text::SMALL,
                        bg_color: ColorToken::SurfaceVariant,
                        border_radius: 6,
                        focus_border_color: ColorToken::SurfaceVariant
                    ) [
                        TextEditing::new(fm.edit_document()),
                        Placeholder("untitled folder"),
                        FieldFocus {
                            color: theme::FOCUS.into(),
                            radius: Fixed::from_int(8),
                            width: Fixed::from_int(3),
                        }
                    ] on TextEditing::Changed {
                        let text = fm.edit_text();
                        fm.set_edit_text(&text);
                    } on TextEditing::Submitted {
                        fm.commit_inline_rename();
                    } on FocusChanged {
                        if !event.focused {
                            fm.commit_inline_rename();
                        }
                    }
                } else {
                    Text (
                        text: ${ { let _ = fm.names_revision(); fm.entry_title(id) } },
                        width: Dimension::percent(100),
                        font_size: text::SMALL,
                        text_color: ColorToken::OnSurface,
                        paragraph: ParagraphStyle::label()
                    )
                }
            }
        } on Tap {
            fm.select(id);
        } on Tap(2) {
            fm.select(id);
            fm.open_dialog(DialogKind::GetInfo);
        }
    }
}

/// One row of the list.
///
/// The list is a plain scrolling column rather than mirui's `LazyList`: the
/// lazy list touches every retained cell on every frame, which invalidates it
/// and repaints its text and its icon, so a list of a few hundred rows costs
/// milliseconds on every frame. A plain column has none of that machinery, and
/// the number of rows is capped so that laying out a large folder stays cheap.
#[compose]
fn row_cell(fm: model!(FmState), index: u32) -> Entity {
    let id = fm.entry_id(index);
    let kind = fm.entry_kind(id);
    ui! {
        Row (
            width: Dimension::percent(100),
            height: 26,
            align: AlignItems::Center,
            column_gap: 8,
            padding: pad_h(12),
            bg_color: ${ if fm.selected() == Some(id) { theme::SELECTION } else { theme::CLEAR } }
        ) [
            HitTarget
            InteractionFeedback
            DropZone { id, folder: kind == Kind::Folder, place: None }
            HoverSurface {
                hover: theme::HOVER.into(),
                pressed: theme::PRESSED.into(),
                radius: Fixed::ZERO,
            }
        ] {
            View (width: ROW_ICON, height: ROW_ICON) [
                KindIcon {
                    model: fm.clone(),
                    index,
                    id: Some(id),
                    size: ROW_ICON,
                }
            ]
            Text (
                text: ${ { let _ = fm.listing_revision(); fm.entry_label(index) } },
                font_size: text::BODY,
                text_color: ColorToken::OnSurface,
                grow: 1.0
            )
            Text (
                text: ${ { let _ = fm.listing_revision(); fm.entry_kind_at(index) } },
                font_size: text::BODY,
                text_color: ColorToken::OnSurfaceVariant,
                width: 78
            )
            Text (
                text: ${ { let _ = fm.listing_revision(); fm.entry_size_at(index) } },
                font_size: text::BODY,
                text_color: ColorToken::OnSurfaceVariant,
                width: 72
            )
            Text (
                text: ${ { let _ = fm.listing_revision(); fm.entry_date_at(index) } },
                font_size: text::BODY,
                text_color: ColorToken::OnSurfaceVariant,
                width: 80
            )
        } on Tap {
            fm.select(id);
        } on Tap(2) {
            fm.select(id);
            fm.open_dialog(DialogKind::GetInfo);
        }
    }
}
