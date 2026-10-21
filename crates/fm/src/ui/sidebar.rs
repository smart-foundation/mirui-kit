//! The sidebar: places, favourites and the current selection.

use alloc::string::ToString;

use mirui::prelude::*;
use mirui::ui::widgets::{Icon, Text};

use crate::data;
use crate::icons;
use crate::model::{DropTarget, FmState, Place};
use crate::theme;
use crate::ui::content::DropZone;
use crate::ui::kit::pad;

/// The sidebar pane.
#[compose]
pub fn sidebar(fm: model!(FmState)) -> Entity {
    ui! {
        Column (
            id: "sidebar",
            width: 216,
            height: Dimension::percent(100),
            bg_color: ColorToken::SurfaceVariant,
            padding: pad(10, 8, 10, 8),
            row_gap: 2
        ) {
            Text (
                text: "PLACES",
                font_size: 10,
                text_color: ColorToken::OnSurfaceVariant,
                padding: pad(0, 6, 4, 6)
            )
            walk data::PLACES.iter() with place {
                place_row (fm, *place)
            }
        }
    }
}

/// Stable identity of a sidebar row, so hosts and tests can find it.
pub const fn place_id(place: Place) -> &'static str {
    match place {
        Place::Favourites => "place_favourites",
        Place::Documents => "place_documents",
        Place::Downloads => "place_downloads",
        Place::Projects => "place_projects",
        Place::Trash => "place_trash",
    }
}

/// One sidebar row.
#[compose]
fn place_row(fm: model!(FmState), place: Place) -> Entity {
    ui! {
        Row (
            width: Dimension::percent(100),
            height: 28,
            border_radius: 6,
            align: AlignItems::Center,
            column_gap: 8,
            padding: crate::ui::kit::pad_h(6),
            bg_color: ${ if fm.drop_target() == Some(DropTarget::Place(place)) { theme::DROP } else if fm.place() == place { theme::SELECTION } else { theme::CLEAR } }
        ) [
            HitTarget
            DropZone { position: 0, folder: true, place: Some(place) }
        ] {
            Icon (
                path: data::place_icon(place).clone(),
                color: ColorToken::OnSurfaceVariant,
                width: 15,
                height: 15
            )
            Text (
                text: data::place_name(place),
                grow: 1.0,
                font_size: 13,
                text_color: ${ if fm.place() == place { ColorToken::Primary } else { ColorToken::OnSurface } }
            )
            Text (
                text: data::place_count(place).to_string(),
                font_size: 11,
                text_color: ColorToken::OnSurfaceVariant
            )
        } on Tap {
            fm.set_place(place);
        }
    }
}

/// The star used in the favourites header.
pub fn favourite_icon() -> &'static mirui::render::path::Path {
    &icons::STAR
}
