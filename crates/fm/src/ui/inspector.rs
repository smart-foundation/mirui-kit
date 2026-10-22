//! The inspector panel.
//!
//! A right hand panel describing the selected item, with the same three
//! sections Finder uses: info, preview and tags.

use mirui::prelude::*;
use mirui::ui::widgets::{Button, ProgressBar, TabBar, Text};

use crate::model::FmState;
use crate::text;
use crate::theme;
use crate::ui::kindicon::selected_icon;
use crate::ui::kit::{pad, pad_v};

/// The inspector, shown when the state asks for it.
#[compose]
pub fn panel(fm: model!(FmState)) -> Entity {
    ui! {
        Column (
            width: 236,
            height: Dimension::percent(100),
            bg_color: ColorToken::SurfaceVariant,
            padding: pad(10, 10, 10, 10),
            row_gap: 10,
            visible: ${ fm.inspector() }
        ) {
            Row (width: Dimension::percent(100), height: 24, align: AlignItems::Center) {
                Text (
                    text: "Inspector",
                    grow: 1.0,
                    font_size: text::SMALL,
                    text_color: ColorToken::OnSurfaceVariant
                )
                Button (
                    width: 20,
                    height: 20,
                    border_radius: 5,
                    padding: pad_v(0),
                    normal_color: theme::CLEAR,
                    pressed_color: theme::SELECTION
                ) {
                    Text (
                        text: "X",
                        width: Dimension::percent(100),
                        height: 20,
                        paragraph: mirui::ui::widgets::ParagraphStyle::label(),
                        font_size: text::SMALL,
                        text_color: ColorToken::OnSurfaceVariant
                    )
                } on Tap {
                    fm.toggle_inspector();
                }
            }
            TabBar (
                id: "inspector_tabs",
                width: Dimension::percent(100),
                height: 26,
                bg_color: ColorToken::Surface,
                border_radius: 6,
                clip_children: true,
                count: 3,
                indicator_height: Fixed::from_int(2)
            )
            preview (fm)
        }
    }
}

/// A large preview of the selected item.
#[compose]
fn preview(fm: model!(FmState)) -> Entity {
    ui! {
        Column (width: Dimension::percent(100), row_gap: 10) {
            View (
                width: Dimension::percent(100),
                height: 130,
                border_radius: 8,
                bg_color: ColorToken::Surface,
                align: AlignItems::Center,
                justify: JustifyContent::Center
            ) {
                selected_icon (fm, 64)
            }
            Text (
                text: ${ { let _ = fm.selected(); let _ = fm.names_revision(); fm.info_name() } },
                width: Dimension::percent(100),
                font_size: text::SMALL,
                text_color: ColorToken::OnSurface
            )
            detail (fm, "Kind", 1)
            detail (fm, "Size", 2)
            detail (fm, "Modified", 3)
            detail (fm, "Where", 4)
            View (width: Dimension::percent(100), height: 1, bg_color: theme::SEPARATOR)
            Text (
                text: "Storage",
                width: Dimension::percent(100),
                font_size: text::SMALL,
                text_color: ColorToken::OnSurfaceVariant
            )
            ProgressBar (
                width: Dimension::percent(100),
                height: 6,
                value: 0.62,
                track_color: ColorToken::Outline,
                fill_color: ColorToken::Primary
            )
            Text (
                text: "128.4 GB available",
                width: Dimension::percent(100),
                font_size: text::SMALL,
                text_color: ColorToken::OnSurfaceVariant
            )
        }
    }
}

#[compose]
fn detail(fm: model!(FmState), label: &'static str, field: u8) -> Entity {
    ui! {
        Row (width: Dimension::percent(100), height: 18, column_gap: 8) {
            Text (text: label, width: 74, font_size: text::SMALL, text_color: ColorToken::OnSurfaceVariant)
            Text (
                text: ${ { let _ = fm.selected(); let _ = fm.listing_revision(); fm.info_field(field) } },
                grow: 1.0,
                font_size: text::SMALL,
                text_color: ColorToken::OnSurface
            )
        }
    }
}
