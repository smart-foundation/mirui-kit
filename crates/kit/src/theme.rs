//! Apple-flavoured palettes.
//!
//! The built-in `Theme` only knows a handful of roles, so the rest of the
//! chrome uses custom tokens resolved through the same theme switch.

use mirui::app::{App, RendererFactory};
use mirui::prelude::*;

/// Selected row / selected file background.
pub const SELECTION: ColorToken = ColorToken::custom("selection");
/// Hairline separators.
pub const SEPARATOR: ColorToken = ColorToken::custom("separator");
/// Toolbar background.
pub const TOOLBAR: ColorToken = ColorToken::custom("toolbar");
/// Traffic light — close.
pub const TRAFFIC_RED: ColorToken = ColorToken::custom("traffic_red");
/// Traffic light — minimise.
pub const TRAFFIC_YELLOW: ColorToken = ColorToken::custom("traffic_yellow");
/// Traffic light — zoom.
pub const TRAFFIC_GREEN: ColorToken = ColorToken::custom("traffic_green");
/// Fully transparent, for "no background" cases.
pub const CLEAR: ColorToken = ColorToken::custom("clear");
/// The dimming layer behind a modal sheet.
pub const DIM: ColorToken = ColorToken::custom("dim");
/// The highlight of a folder that would receive a dropped file.
pub const DROP: ColorToken = ColorToken::custom("drop");
/// The raised surface used by the context menu button.
pub const SURFACE: ColorToken = ColorToken::custom("raised");
/// The surface of a hovered item.
pub const HOVER: ColorToken = ColorToken::custom("hover");
/// The surface of a pressed item.
pub const PRESSED: ColorToken = ColorToken::custom("pressed");
/// The background of a hovered menu entry.
pub const MENU_HOVER: ColorToken = ColorToken::custom("menu_hover");
/// The background of a resting menu entry.
pub const MENU_REST: ColorToken = ColorToken::custom("menu_rest");
/// The thumb of a scroll bar.
pub const SCROLLBAR: ColorToken = ColorToken::custom("scrollbar");
/// The ring around the text field that has the focus.
pub const FOCUS: ColorToken = ColorToken::custom("focus");
/// The background of a small count badge.
pub const BADGE: ColorToken = ColorToken::custom("badge");
/// The text of a small count badge.
pub const BADGE_TEXT: ColorToken = ColorToken::custom("badge_text");
/// The base color of a loading skeleton placeholder.
pub const SKELETON: ColorToken = ColorToken::custom("skeleton");
/// The shimmer highlight of a loading skeleton placeholder.
pub const SKELETON_SHIMMER: ColorToken = ColorToken::custom("skeleton_shimmer");
/// The background of a segmented control.
pub const SEGMENTED_BG: ColorToken = ColorToken::custom("segmented_bg");
/// The selected segment of a segmented control.
pub const SEGMENTED_SELECTED: ColorToken = ColorToken::custom("segmented_selected");
/// The background of a tooltip bubble.
pub const TOOLTIP_BG: ColorToken = ColorToken::custom("tooltip_bg");
/// The text of a tooltip bubble.
pub const TOOLTIP_TEXT: ColorToken = ColorToken::custom("tooltip_text");

pub const SUCCESS: ColorToken = ColorToken::custom("success");
pub const WARNING: ColorToken = ColorToken::custom("warning");

/// The dark palette.
pub fn dark_theme() -> Theme {
    Theme::dark().with_many([
        (SUCCESS, Color::rgb(0x75, 0xD5, 0x98)),
        (WARNING, Color::rgb(0xF0, 0xC0, 0x60)),
        (ColorToken::Surface, Color::rgb(0x1C, 0x1C, 0x1E)),
        (ColorToken::SurfaceVariant, Color::rgb(0x2C, 0x2C, 0x2E)),
        (ColorToken::OnSurface, Color::rgb(0xF2, 0xF2, 0xF7)),
        (ColorToken::OnSurfaceVariant, Color::rgb(0x98, 0x98, 0x9F)),
        (ColorToken::Primary, Color::rgb(0x00, 0x5F, 0xCC)),
        (ColorToken::Error, Color::rgb(0xFF, 0x8A, 0x80)),
        (ColorToken::OnPrimary, Color::rgb(0xFF, 0xFF, 0xFF)),
        (ColorToken::Secondary, Color::rgb(0x64, 0x64, 0x6A)),
        (ColorToken::OnSecondary, Color::rgb(0xF2, 0xF2, 0xF7)),
        (ColorToken::Outline, Color::rgb(0x38, 0x38, 0x3A)),
        (SELECTION, Color::rgb(0x1E, 0x3A, 0x5F)),
        (SEPARATOR, Color::rgba(0xFF, 0xFF, 0xFF, 0x18)),
        (TOOLBAR, Color::rgb(0x2C, 0x2C, 0x2E)),
        (TRAFFIC_RED, Color::rgb(0xFF, 0x5F, 0x57)),
        (TRAFFIC_YELLOW, Color::rgb(0xFF, 0xBD, 0x2E)),
        (TRAFFIC_GREEN, Color::rgb(0x28, 0xC8, 0x40)),
        (CLEAR, Color::rgba(0, 0, 0, 0)),
        (DIM, Color::rgba(0, 0, 0, 0x99)),
        (DROP, Color::rgb(0x1E, 0x5A, 0x33)),
        (SURFACE, Color::rgb(0x45, 0x45, 0x4A)),
        (HOVER, Color::rgba(0xFF, 0xFF, 0xFF, 0x14)),
        (PRESSED, Color::rgba(0xFF, 0xFF, 0xFF, 0x24)),
        (MENU_HOVER, Color::rgba(0x0A, 0x84, 0xFF, 0x4D)),
        (MENU_REST, Color::rgba(0, 0, 0, 0)),
        (SCROLLBAR, Color::rgba(0xFF, 0xFF, 0xFF, 0x57)),
        (FOCUS, Color::rgb(0x0A, 0x84, 0xFF)),
        (BADGE, Color::rgb(0x38, 0x38, 0x3A)),
        (BADGE_TEXT, Color::rgb(0xF2, 0xF2, 0xF7)),
        (SKELETON, Color::rgb(0x38, 0x38, 0x3A)),
        (SKELETON_SHIMMER, Color::rgba(0xFF, 0xFF, 0xFF, 0x08)),
        (SEGMENTED_BG, Color::rgb(0x2C, 0x2C, 0x2E)),
        (SEGMENTED_SELECTED, Color::rgb(0x1C, 0x1C, 0x1E)),
        (TOOLTIP_BG, Color::rgb(0x1C, 0x1C, 0x1E)),
        (TOOLTIP_TEXT, Color::rgb(0xF2, 0xF2, 0xF7)),
    ])
}

/// The light palette.
pub fn light_theme() -> Theme {
    Theme::light().with_many([
        (SUCCESS, Color::rgb(0x18, 0x68, 0x38)),
        (WARNING, Color::rgb(0x80, 0x50, 0x00)),
        (ColorToken::Surface, Color::rgb(0xFF, 0xFF, 0xFF)),
        (ColorToken::SurfaceVariant, Color::rgb(0xF2, 0xF2, 0xF7)),
        (ColorToken::OnSurface, Color::rgb(0x00, 0x00, 0x00)),
        (ColorToken::OnSurfaceVariant, Color::rgb(0x6E, 0x6E, 0x73)),
        (ColorToken::Primary, Color::rgb(0x00, 0x5F, 0xCC)),
        (ColorToken::Error, Color::rgb(0xB4, 0x23, 0x18)),
        (ColorToken::OnPrimary, Color::rgb(0xFF, 0xFF, 0xFF)),
        (ColorToken::Secondary, Color::rgb(0xD1, 0xD1, 0xD6)),
        (ColorToken::OnSecondary, Color::rgb(0x00, 0x00, 0x00)),
        (ColorToken::Outline, Color::rgb(0xC6, 0xC6, 0xC8)),
        (SELECTION, Color::rgb(0xD9, 0xE7, 0xFF)),
        (SEPARATOR, Color::rgba(0x00, 0x00, 0x00, 0x18)),
        (TOOLBAR, Color::rgb(0xF6, 0xF6, 0xF8)),
        (TRAFFIC_RED, Color::rgb(0xFF, 0x5F, 0x57)),
        (TRAFFIC_YELLOW, Color::rgb(0xFE, 0xBC, 0x2E)),
        (TRAFFIC_GREEN, Color::rgb(0x28, 0xC8, 0x40)),
        (CLEAR, Color::rgba(0, 0, 0, 0)),
        (DIM, Color::rgba(0, 0, 0, 0x66)),
        (DROP, Color::rgb(0xD9, 0xF2, 0xD9)),
        (SURFACE, Color::rgb(0xE6, 0xE6, 0xEA)),
        (HOVER, Color::rgba(0x00, 0x00, 0x00, 0x0D)),
        (PRESSED, Color::rgba(0x00, 0x00, 0x00, 0x17)),
        (MENU_HOVER, Color::rgba(0x00, 0x7A, 0xFF, 0x29)),
        (MENU_REST, Color::rgba(0, 0, 0, 0)),
        (SCROLLBAR, Color::rgba(0, 0, 0, 0x42)),
        (FOCUS, Color::rgb(0x00, 0x7A, 0xFF)),
        (BADGE, Color::rgb(0xC6, 0xC6, 0xC8)),
        (BADGE_TEXT, Color::rgb(0x24, 0x24, 0x28)),
        (SKELETON, Color::rgb(0xE6, 0xE6, 0xEA)),
        (SKELETON_SHIMMER, Color::rgba(0, 0, 0, 0x06)),
        (SEGMENTED_BG, Color::rgb(0xF2, 0xF2, 0xF7)),
        (SEGMENTED_SELECTED, Color::rgb(0xFF, 0xFF, 0xFF)),
        (TOOLTIP_BG, Color::rgb(0x00, 0x00, 0x00)),
        (TOOLTIP_TEXT, Color::rgb(0xF2, 0xF2, 0xF7)),
    ])
}

/// Register both palettes and activate the dark one.
pub fn install<B: Surface, F: RendererFactory<B>>(app: &mut App<B, F>) {
    app.register_theme(dark_theme());
    app.register_theme(light_theme());
    let _ = app.set_theme("dark");
}
