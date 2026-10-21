//! Modal sheets.
//!
//! A dimming backdrop over the whole window, and a centred panel. One sheet
//! shape serves every dialog; the model decides what it says and what its
//! buttons do. Every sheet is built once and shown or hidden, so opening a
//! dialog never rebuilds a widget tree.

use mirui::prelude::*;
use mirui::ui::widgets::{Button, ParagraphStyle, Placeholder, Text, TextEditing, TextInput};

use crate::model::{DialogKind, FmState, FmStateHandle};
use crate::theme;
use crate::ui::kit::{pad, pad_v};

const SHEET_W: i32 = 420;
const SHEET_H: i32 = 232;

/// The modal layer.
#[compose]
pub fn dialog_layer(fm: model!(FmState)) -> Entity {
    ui! {
        View (
            position: Position::Absolute,
            left: 0,
            top: 0,
            width: Dimension::percent(100),
            height: Dimension::percent(100)
        ) {
            View (
                position: Position::Absolute,
                left: 0,
                top: 0,
                width: Dimension::percent(100),
                height: Dimension::percent(100),
                bg_color: theme::DIM,
                visible: ${ fm.dialog().is_some() }
            ) [
                HitTarget
            ] on Tap {
                fm.close_dialog();
            }
            Column (
                position: Position::Absolute,
                left: 0,
                top: 0,
                width: Dimension::percent(100),
                height: Dimension::percent(100),
                align: AlignItems::Center,
                justify: JustifyContent::Center,
                visible: ${ fm.dialog().is_some() }
            ) {
                View (
                    width: SHEET_W,
                    height: SHEET_H,
                    border_radius: 12,
                    bg_color: ColorToken::Surface,
                    border_color: theme::SEPARATOR,
                    border_width: 1,
                    padding: pad(18, 18, 16, 18),
                    row_gap: 14,
                    clip_children: true
                ) {
                    new_folder_sheet (fm)
                    rename_sheet (fm)
                    about_sheet (fm)
                    info_sheet (fm)
                    confirm_sheet (fm)
                }
            }
        }
    }
}

#[compose]
fn new_folder_sheet(fm: model!(FmState)) -> Entity {
    ui! {
        Column (
            position: Position::Absolute,
            left: 0,
            top: 0,
            width: Dimension::percent(100),
            height: Dimension::percent(100),
            row_gap: 12,
            visible: ${ fm.dialog() == Some(DialogKind::NewFolder) }
        ) {
            Text (text: "New Folder", font_size: 15, text_color: ColorToken::OnSurface)
            Text (
                text: "Choose a name for the new folder.",
                font_size: 12,
                text_color: ColorToken::OnSurfaceVariant
            )
            TextInput (width: Dimension::percent(100), height: 28) [
                TextEditing::new(fm.folder_document()),
                Placeholder("untitled folder"),
            ] on TextEditing::Changed {
                let text = fm.folder_text();
                fm.set_folder_text(&text);
            }
            buttons (fm, 1)
        }
    }
}

#[compose]
fn rename_sheet(fm: model!(FmState)) -> Entity {
    ui! {
        Column (
            position: Position::Absolute,
            left: 0,
            top: 0,
            width: Dimension::percent(100),
            height: Dimension::percent(100),
            row_gap: 12,
            visible: ${ fm.dialog() == Some(DialogKind::Rename) }
        ) {
            Text (text: "Rename", font_size: 15, text_color: ColorToken::OnSurface)
            Text (
                text: "Type a new name for this item.",
                font_size: 12,
                text_color: ColorToken::OnSurfaceVariant
            )
            TextInput (width: Dimension::percent(100), height: 28) [
                TextEditing::new(fm.rename_document()),
                Placeholder("name"),
            ] on TextEditing::Changed {
                let text = fm.rename_text();
                fm.set_rename_text(&text);
            }
            buttons (fm, 2)
        }
    }
}

#[compose]
fn about_sheet(fm: model!(FmState)) -> Entity {
    ui! {
        Column (
            position: Position::Absolute,
            left: 0,
            top: 0,
            width: Dimension::percent(100),
            height: Dimension::percent(100),
            row_gap: 8,
            visible: ${ fm.dialog() == Some(DialogKind::About) }
        ) {
            Text (text: "Files", font_size: 17, text_color: ColorToken::OnSurface)
            Text (
                text: "A no_std file manager built on mirui.",
                font_size: 12,
                text_color: ColorToken::OnSurfaceVariant
            )
            Text (
                text: "The whole application is no_std plus alloc.",
                font_size: 12,
                text_color: ColorToken::OnSurfaceVariant
            )
            Text (
                text: "Every icon is a vector path in the program image.",
                font_size: 12,
                text_color: ColorToken::OnSurfaceVariant
            )
            Text (
                text: "Drag a file onto a folder to move it.",
                font_size: 12,
                text_color: ColorToken::OnSurfaceVariant
            )
            buttons (fm, 0)
        }
    }
}

#[compose]
fn info_sheet(fm: model!(FmState)) -> Entity {
    ui! {
        Column (
            position: Position::Absolute,
            left: 0,
            top: 0,
            width: Dimension::percent(100),
            height: Dimension::percent(100),
            row_gap: 8,
            visible: ${ fm.dialog() == Some(DialogKind::GetInfo) }
        ) {
            Text (text: "Info", font_size: 15, text_color: ColorToken::OnSurface)
            info_row (fm, "Name", 0)
            info_row (fm, "Kind", 1)
            info_row (fm, "Size", 2)
            info_row (fm, "Modified", 3)
            info_row (fm, "Where", 4)
            buttons (fm, 0)
        }
    }
}

#[compose]
fn confirm_sheet(fm: model!(FmState)) -> Entity {
    ui! {
        Column (
            position: Position::Absolute,
            left: 0,
            top: 0,
            width: Dimension::percent(100),
            height: Dimension::percent(100),
            row_gap: 12,
            visible: ${ fm.dialog() == Some(DialogKind::Delete) }
        ) {
            Text (text: "Move to Trash", font_size: 15, text_color: ColorToken::OnSurface)
            Text (
                text: "The item can be put back from the Trash.",
                font_size: 12,
                text_color: ColorToken::OnSurfaceVariant
            )
            buttons (fm, 3)
        }
    }
}

/// A label and a value, as used by the info sheet.
#[compose]
fn info_row(fm: model!(FmState), label: &'static str, field: u8) -> Entity {
    ui! {
        Row (width: Dimension::percent(100), height: 20, column_gap: 10) {
            Text (text: label, width: 90, font_size: 12, text_color: ColorToken::OnSurfaceVariant)
            Text (
                text: ${ { let _ = fm.revision(); fm.info_field(field) } },
                grow: 1.0,
                font_size: 12,
                text_color: ColorToken::OnSurface
            )
        }
    }
}

/// The buttons at the bottom of a sheet. The id says what the button does.
#[compose]
fn buttons(fm: model!(FmState), action: u8) -> Entity {
    ui! {
        Row (
            width: Dimension::percent(100),
            height: 28,
            column_gap: 8,
            justify: JustifyContent::FlexEnd
        ) {
            Button (
                width: 82,
                height: 26,
                border_radius: 6,
                padding: pad_v(0),
                normal_color: theme::CLEAR,
                pressed_color: theme::SELECTION
            ) {
                Text (
                    text: "Cancel",
                    width: Dimension::percent(100),
                    height: 26,
                    paragraph: ParagraphStyle::label(),
                    font_size: 12,
                    text_color: ColorToken::OnSurface
                )
            } on Tap {
                fm.close_dialog();
            }
            Button (
                width: 82,
                height: 26,
                border_radius: 6,
                padding: pad_v(0),
                normal_color: ColorToken::Primary,
                pressed_color: ColorToken::Primary
            ) {
                Text (
                    text: "OK",
                    width: Dimension::percent(100),
                    height: 26,
                    paragraph: ParagraphStyle::label(),
                    font_size: 12,
                    text_color: ColorToken::OnPrimary
                )
            } on Tap {
                confirm_dialog (&fm, action);
            }
        }
    }
}

fn confirm_dialog(fm: &FmStateHandle, action: u8) {
    match action {
        1 => {
            let name = fm.folder_text();
            fm.create_folder(&name);
        }
        2 => {
            let name = fm.rename_text();
            if let Some(id) = fm.selected() {
                fm.rename(id, &name);
            }
        }
        3 => {
            if let Some(id) = fm.selected() {
                fm.trash(id);
            }
            fm.close_dialog();
        }
        _ => fm.close_dialog(),
    }
}
