//! The status bar at the bottom of the window, and the transient message.

use mirui::prelude::*;
use mirui::ui::widgets::{ProgressBar, Text};

use crate::model::FmState;
use crate::theme;
use crate::ui::kit::pad_h;

/// Item count and free space.
#[compose]
pub fn bar(fm: model!(FmState)) -> Entity {
    ui! {
        Row (
            width: Dimension::percent(100),
            height: 26,
            bg_color: theme::TOOLBAR,
            align: AlignItems::Center,
            column_gap: 10,
            padding: pad_h(12)
        ) {
            Text (
                text: ${ { let _ = fm.revision(); fm.count_label() } },
                font_size: 11,
                text_color: ColorToken::OnSurfaceVariant
            )
            Text (
                text: ${ fm.sort_label() },
                font_size: 11,
                text_color: ColorToken::OnSurfaceVariant
            )
            View (grow: 1.0)
            Text (
                text: "128.4 GB available",
                font_size: 11,
                text_color: ColorToken::OnSurfaceVariant
            )
            ProgressBar (
                width: 110,
                height: 5,
                value: 0.62,
                track_color: ColorToken::Outline,
                fill_color: ColorToken::Primary
            )
        }
    }
}

/// The transient message, shown above the status bar.
#[compose]
pub fn toast(fm: model!(FmState)) -> Entity {
    ui! {
        View (
            position: Position::Absolute,
            left: 0,
            top: 0,
            width: Dimension::percent(100),
            height: Dimension::percent(100),
            align: AlignItems::Center,
            justify: JustifyContent::FlexEnd,
            padding: crate::ui::kit::pad(0, 0, 38, 0),
            visible: ${ fm.toast_visible() }
        ) [
            HitTarget
        ] on Tap {
            fm.clear_toast();
        } {
            View (
                height: 28,
                border_radius: 8,
                bg_color: ColorToken::Primary,
                padding: pad_h(14),
                align: AlignItems::Center
            ) {
                Text (
                    text: ${ { let _ = fm.toast_id(); fm.toast_text() } },
                    font_size: 12,
                    text_color: ColorToken::OnPrimary
                )
            }
        }
    }
}
