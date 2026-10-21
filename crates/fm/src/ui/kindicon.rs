//! The icon of a file, chosen by kind.
//!
//! An icon path is not a reactive property, so a widget that can show any kind
//! of file keeps one icon per kind and shows only the matching one. The
//! selection is reactive, so the right icon appears as soon as the model
//! changes.

use mirui::prelude::*;
use mirui::ui::widgets::{Icon, ListItem};

use crate::data::Kind;
use crate::icons;
use crate::model::FmState;

/// The icon of one specific file.
#[compose]
pub fn kind_icon(fm: model!(FmState), id: u32, size: i32) -> Entity {
    ui! {
        Row (width: size, height: size) {
            Icon (path: icons::FOLDER.clone(), color: Kind::Folder.tint(), width: size, height: size, visible: ${ fm.entry_is_kind(id, 0) })
            Icon (path: icons::DOCUMENT.clone(), color: Kind::Document.tint(), width: size, height: size, visible: ${ fm.entry_is_kind(id, 1) })
            Icon (path: icons::IMAGE.clone(), color: Kind::Image.tint(), width: size, height: size, visible: ${ fm.entry_is_kind(id, 2) })
            Icon (path: icons::CODE.clone(), color: Kind::Code.tint(), width: size, height: size, visible: ${ fm.entry_is_kind(id, 3) })
            Icon (path: icons::ARCHIVE.clone(), color: Kind::Archive.tint(), width: size, height: size, visible: ${ fm.entry_is_kind(id, 4) })
            Icon (path: icons::AUDIO.clone(), color: Kind::Audio.tint(), width: size, height: size, visible: ${ fm.entry_is_kind(id, 5) })
        }
    }
}

/// The icon of the file a recycled list row currently points at.
#[compose]
pub fn kind_icon_at(fm: model!(FmState), item: model!(ListItem), size: i32) -> Entity {
    ui! {
        Row (width: size, height: size) {
            Icon (path: icons::FOLDER.clone(), color: Kind::Folder.tint(), width: size, height: size, visible: ${ { let _ = fm.revision(); fm.entry_is_kind(fm.entry_id(item.index()), 0) } })
            Icon (path: icons::DOCUMENT.clone(), color: Kind::Document.tint(), width: size, height: size, visible: ${ { let _ = fm.revision(); fm.entry_is_kind(fm.entry_id(item.index()), 1) } })
            Icon (path: icons::IMAGE.clone(), color: Kind::Image.tint(), width: size, height: size, visible: ${ { let _ = fm.revision(); fm.entry_is_kind(fm.entry_id(item.index()), 2) } })
            Icon (path: icons::CODE.clone(), color: Kind::Code.tint(), width: size, height: size, visible: ${ { let _ = fm.revision(); fm.entry_is_kind(fm.entry_id(item.index()), 3) } })
            Icon (path: icons::ARCHIVE.clone(), color: Kind::Archive.tint(), width: size, height: size, visible: ${ { let _ = fm.revision(); fm.entry_is_kind(fm.entry_id(item.index()), 4) } })
            Icon (path: icons::AUDIO.clone(), color: Kind::Audio.tint(), width: size, height: size, visible: ${ { let _ = fm.revision(); fm.entry_is_kind(fm.entry_id(item.index()), 5) } })
        }
    }
}

/// The icon of whatever is selected right now.
#[compose]
pub fn selected_icon(fm: model!(FmState), size: i32) -> Entity {
    ui! {
        Row (width: size, height: size) {
            Icon (path: icons::FOLDER.clone(), color: Kind::Folder.tint(), width: size, height: size, visible: ${ fm.entry_is_kind(fm.selected().unwrap_or(0), 0) })
            Icon (path: icons::DOCUMENT.clone(), color: Kind::Document.tint(), width: size, height: size, visible: ${ fm.entry_is_kind(fm.selected().unwrap_or(0), 1) })
            Icon (path: icons::IMAGE.clone(), color: Kind::Image.tint(), width: size, height: size, visible: ${ fm.entry_is_kind(fm.selected().unwrap_or(0), 2) })
            Icon (path: icons::CODE.clone(), color: Kind::Code.tint(), width: size, height: size, visible: ${ fm.entry_is_kind(fm.selected().unwrap_or(0), 3) })
            Icon (path: icons::ARCHIVE.clone(), color: Kind::Archive.tint(), width: size, height: size, visible: ${ fm.entry_is_kind(fm.selected().unwrap_or(0), 4) })
            Icon (path: icons::AUDIO.clone(), color: Kind::Audio.tint(), width: size, height: size, visible: ${ fm.entry_is_kind(fm.selected().unwrap_or(0), 5) })
        }
    }
}
