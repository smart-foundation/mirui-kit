//! The command palette.
//!
//! A Spotlight style overlay: a filter field on top, and the matching commands
//! underneath. Every command owns a row; a row is shown or hidden by reacting
//! to the filter, so the list never has to be rebuilt.

use mirui::prelude::*;
use mirui::render::font::FontManager;
use mirui::ui::widgets::{Button, Icon, Placeholder, Text, TextEditing, TextInput};

use crate::icons;
use crate::model::{COMMANDS, Command, FmState};
use crate::text;
use crate::theme;
use crate::ui::controls::{self, FieldFocus};
use crate::ui::hover::HoverSurface;
use crate::ui::kit::{pad, pad_h, pad_v};

/// The palette layer.
#[compose]
pub fn palette_layer(fm: model!(FmState)) -> Entity {
    // The filter field is as tall as the line of type it holds: mirui lays
    // the text of a field out from a fixed two pixel inset and never centres
    // it, so the height follows the metrics of the typeface.
    let text_height = res!(FontManager).with(|fonts| controls::field_height(&fonts));
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
                visible: ${ fm.palette() }
            ) [
                HitTarget
            ] on Tap {
                fm.close_palette();
            }
            Column (
                position: Position::Absolute,
                left: 0,
                top: 0,
                width: Dimension::percent(100),
                height: Dimension::percent(100),
                align: AlignItems::Center,
                justify: JustifyContent::Center,
                visible: ${ fm.palette() }
            ) {
                Column (
                    width: 520,
                    height: 360,
                    border_radius: 12,
                    bg_color: ColorToken::Surface,
                    border_color: theme::SEPARATOR,
                    border_width: 1,
                    row_gap: 8,
                    padding: pad(10, 10, 10, 10),
                    clip_children: true
                ) {
                    Row (
                        width: Dimension::percent(100),
                        height: 32,
                        align: AlignItems::Center,
                        column_gap: 8,
                        padding: pad_h(8)
                    ) {
                        TextInput (
                            id: "palette_filter",
                            width: Dimension::percent(100),
                            height: text_height,
                            font_size: text::SMALL,
                            bg_color: ColorToken::SurfaceVariant,
                            border_radius: 6,
                            // mirui draws a square ring of its own when the
                            // field has the focus. The ring is given the
                            // colour of the field, so it cannot be seen: the
                            // application draws its own, rounded.
                            focus_border_color: ColorToken::SurfaceVariant
                        ) [
                            TextEditing::new(fm.palette_document()),
                            Placeholder("Type a command"),
                            FieldFocus {
                                color: theme::FOCUS.into(),
                                radius: Fixed::from_int(8),
                                width: Fixed::from_int(3),
                            }
                        ] on TextEditing::Changed {
                            let text = fm.palette_text();
                            fm.set_palette_text(&text);
                        }
                    }
                    View (width: Dimension::percent(100), height: 1, bg_color: theme::SEPARATOR)
                    Scroll (width: Dimension::percent(100), grow: 1.0) {
                        Column (width: Dimension::percent(100), height: COMMANDS.len() as i32 * 27, row_gap: 1) {
                            command_rows (fm)
                        }
                    }
                }
            }
        }
    }
}

/// One row per command, shown when it matches the filter.
#[compose]
fn command_rows(fm: model!(FmState)) -> Entity {
    ui! {
        Column (width: Dimension::percent(100), height: COMMANDS.len() as i32 * 27, row_gap: 1) {
            walk (0..COMMANDS.len()).collect::<alloc::vec::Vec<usize>>() with index {
                command_row (fm, index)
            }
        }
    }
}

#[compose]
fn command_row(fm: model!(FmState), index: usize) -> Entity {
    let command = COMMANDS[index].2;
    let label = COMMANDS[index].0;
    let shortcut = COMMANDS[index].1;
    ui! {
        Button (
            width: Dimension::percent(100),
            height: 26,
            border_radius: 5,
            padding: pad_v(0),
            // The row rests in the colour of the panel, so it is invisible
            // until the pointer is on it.
            normal_color: ColorToken::Surface,
            pressed_color: theme::SELECTION,
            visible: ${ { let _ = fm.palette_revision(); fm.command_matches(index) } }
        ) [
            HoverSurface {
                hover: theme::MENU_HOVER.into(),
                pressed: theme::MENU_HOVER.into(),
                radius: Fixed::from_int(5)
            }
        ] {
            Row (
                width: Dimension::percent(100),
                height: 26,
                align: AlignItems::Center,
                column_gap: 10,
                padding: pad_h(8)
            ) {
                Icon (
                    path: command_icon(command).clone(),
                    color: ColorToken::OnSurfaceVariant,
                    width: 16,
                    height: 16
                )
                Text (text: label, grow: 1.0, font_size: text::SMALL, text_color: ColorToken::OnSurface)
                Text (text: shortcut, font_size: text::SMALL, text_color: ColorToken::OnSurfaceVariant)
            }
        } on Tap {
            fm.run_command(command);
        }
    }
}

/// Semantic icon for each command, independent of its translated label.
/// Keep this match exhaustive so new commands require an explicit icon choice.
fn command_icon(command: Command) -> &'static Path {
    match command {
        Command::OpenPalette => &icons::SEARCH,
        Command::NewFolder => &icons::NEW_FOLDER,
        Command::ToggleView => &icons::GRID,
        Command::ToggleTheme => &icons::MOON,
        Command::ToggleInspector => &icons::PANEL,
        Command::ToggleHidden => &icons::FOLDER,
        Command::SortByName | Command::SortByDate | Command::SortBySize => &icons::SORT,
        Command::GoFavourites => &icons::STAR,
        Command::GoDocuments => &icons::DOCUMENT,
        Command::GoDownloads => &icons::DOWNLOAD,
        Command::GoProjects => &icons::LAYERS,
        Command::GoTrash => &icons::TRASH,
        Command::TakeScreenshot => &icons::IMAGE,
        Command::About => &icons::INFO,
        Command::Dismiss => &icons::CLOSE,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_registered_command_has_a_semantic_icon() {
        for &(_, _, command) in &COMMANDS {
            let _ = command_icon(command);
        }
        assert!(core::ptr::eq(
            command_icon(Command::NewFolder),
            &icons::NEW_FOLDER
        ));
        assert!(core::ptr::eq(
            command_icon(Command::GoTrash),
            &icons::TRASH
        ));
        assert!(core::ptr::eq(
            command_icon(Command::OpenPalette),
            &icons::SEARCH
        ));
    }
}
