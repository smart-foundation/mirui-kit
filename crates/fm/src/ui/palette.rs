//! The command palette.
//!
//! A Spotlight style overlay: a filter field on top, and the matching commands
//! underneath. Every command owns a row; a row is shown or hidden by reacting
//! to the filter, so the list never has to be rebuilt.

use mirui::prelude::*;
use mirui::ui::widgets::{Button, Placeholder, Text, TextEditing, TextInput};

use crate::model::{COMMANDS, FmState};
use crate::text;
use crate::theme;
use crate::ui::hover::HoverSurface;
use crate::ui::kit::{pad, pad_h, pad_v};

/// The palette layer.
#[compose]
pub fn palette_layer(fm: model!(FmState)) -> Entity {
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
                            width: Dimension::percent(100),
                            height: 26,
                            font_size: text::SMALL,
                            bg_color: ColorToken::SurfaceVariant,
                            border_radius: 6
                        ) [
                            TextEditing::new(fm.palette_document()),
                            Placeholder("Type a command"),
                        ] on TextEditing::Changed {
                            let text = fm.palette_text();
                            fm.set_palette_text(&text);
                        }
                    }
                    View (width: Dimension::percent(100), height: 1, bg_color: theme::SEPARATOR)
                    Scroll (width: Dimension::percent(100), grow: 1.0) {
                        Column (width: Dimension::percent(100), height: 400, row_gap: 1) {
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
        Column (width: Dimension::percent(100), height: 400, row_gap: 1) {
            command_row (fm, 0)
            command_row (fm, 1)
            command_row (fm, 2)
            command_row (fm, 3)
            command_row (fm, 4)
            command_row (fm, 5)
            command_row (fm, 6)
            command_row (fm, 7)
            command_row (fm, 8)
            command_row (fm, 9)
            command_row (fm, 10)
            command_row (fm, 11)
            command_row (fm, 12)
            command_row (fm, 13)
            command_row (fm, 14)
            command_row (fm, 15)
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
                Text (text: label, grow: 1.0, font_size: text::SMALL, text_color: ColorToken::OnSurface)
                Text (text: shortcut, font_size: text::SMALL, text_color: ColorToken::OnSurfaceVariant)
            }
        } on Tap {
            fm.run_command(command);
        }
    }
}
