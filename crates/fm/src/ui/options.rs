//! The view options popover.
//!
//! A small panel anchored under the toolbar that holds the controls the file
//! manager needs: icon size, hidden files and the sort order.
//!
//! The check box is the application's own: mirui's own is a filled rounded
//! square without an outline and without a tick.

use mirui::prelude::*;
use mirui::ui::widgets::{Button, Slider, Text};

use crate::model::{FmState, SortMode};
use crate::text;
use crate::theme;
use crate::ui::controls::CheckBox;
use crate::ui::hover::HoverSurface;
use crate::ui::kit::{pad, pad_v};

/// The popover, shown while the toolbar button is held down.
#[compose]
pub fn options_layer(fm: model!(FmState)) -> Entity {
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
                visible: ${ fm.options_open() }
            ) [
                HitTarget
            ] on Tap {
                fm.set_options_open(false);
            }
            View (
                position: Position::Absolute,
                left: 610,
                top: 46,
                width: 250,
                height: 186,
                border_radius: 10,
                bg_color: ColorToken::Surface,
                border_color: theme::SEPARATOR,
                border_width: 1,
                padding: pad(12, 12, 12, 12),
                row_gap: 12,
                visible: ${ fm.options_open() }
            ) {
                Column (width: Dimension::percent(100), row_gap: 12) {
                    Text (
                        text: "View Options",
                        font_size: text::SMALL,
                        text_color: ColorToken::OnSurfaceVariant
                    )
                    Row (width: Dimension::percent(100), height: 22, align: AlignItems::Center) {
                        Text (text: "Icon size", grow: 1.0, font_size: text::SMALL, text_color: ColorToken::OnSurface)
                        Text (
                            text: ${ alloc::format!("{}", fm.icon_size()) },
                            width: 34,
                            font_size: text::SMALL,
                            text_color: ColorToken::OnSurfaceVariant
                        )
                    }
                    Slider (
                        width: Dimension::percent(100),
                        height: 18,
                        min: Fixed::from_int(24),
                        max: Fixed::from_int(72),
                        value: ${ Fixed::from_int(fm.icon_size() as i32) },
                        track_color: ColorToken::Outline,
                        fill_color: ColorToken::Primary,
                        thumb_color: ColorToken::Surface
                    ) on Slider::ValueChanged {
                        let value = fm.icon_size();
                        let _ = value;
                    }
                    Row (
                        width: Dimension::percent(100),
                        height: 22,
                        align: AlignItems::Center,
                        column_gap: 8,
                        border_radius: 4,
                        // The row rests in the colour of the panel, so it is
                        // invisible until the pointer is on it.
                        bg_color: ColorToken::Surface
                    ) [
                        HitTarget
                        InteractionFeedback
                        HoverSurface {
                            hover: theme::MENU_HOVER.into(),
                            pressed: theme::MENU_HOVER.into(),
                            radius: Fixed::from_int(4)
                        }
                    ] {
                        View (
                            width: 16,
                            height: 16,
                            border_radius: 4
                        ) [
                            HitTarget
                            CheckBox {
                                model: fm.clone(),
                                size: 16,
                            }
                        ] on Tap {
                            fm.toggle_hidden();
                        }
                        Text (text: "Show hidden files", font_size: text::SMALL, text_color: ColorToken::OnSurface)
                    }
                    Text (
                        text: "Sort by",
                        font_size: text::SMALL,
                        text_color: ColorToken::OnSurfaceVariant
                    )
                    Row (width: Dimension::percent(100), height: 24, column_gap: 4) {
                        sort_button (fm, 0, "Name")
                        sort_button (fm, 1, "Date")
                        sort_button (fm, 2, "Size")
                        sort_button (fm, 3, "Kind")
                    }
                }
            }
        }
    }
}

#[compose]
fn sort_button(fm: model!(FmState), sort: u8, label: &'static str) -> Entity {
    ui! {
        Button (
            width: 52,
            height: 24,
            border_radius: 5,
            padding: pad_v(0),
            // The button rests in the colour of the panel, so it is invisible
            // until the pointer is on it.
            normal_color: ColorToken::Surface,
            pressed_color: theme::SELECTION
        ) [
            HoverSurface {
                hover: theme::MENU_HOVER.into(),
                pressed: theme::MENU_HOVER.into(),
                radius: Fixed::from_int(5)
            }
        ] {
            Text (
                text: label,
                width: Dimension::percent(100),
                height: 24,
                paragraph: mirui::ui::widgets::ParagraphStyle::label(),
                font_size: text::SMALL,
                text_color: ColorToken::OnSurface
            )
        } on Tap {
            match sort {
                0 => fm.set_sort(SortMode::Name),
                1 => fm.set_sort(SortMode::Date),
                2 => fm.set_sort(SortMode::Size),
                _ => fm.set_sort(SortMode::Kind),
            }
        }
    }
}
