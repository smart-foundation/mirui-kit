//! The status bar at the bottom of the window.

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
