//! Shared component geometry and semantic colors. No per-widget palette literals.
use crate::theme;
use mirui::prelude::*;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Density {
    Compact,
    #[default]
    Comfortable,
    Touch,
}
impl Density {
    pub fn control_height(self) -> i32 {
        match self {
            Self::Compact => 32,
            Self::Comfortable => 40,
            Self::Touch => 48,
        }
    }
    pub fn inset(self) -> i32 {
        match self {
            Self::Compact => 8,
            _ => 12,
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Intent {
    Primary,
    #[default]
    Neutral,
    Destructive,
    Quiet,
}
impl Intent {
    pub fn background(self) -> ColorToken {
        match self {
            Self::Primary => ColorToken::Primary,
            Self::Neutral => ColorToken::SurfaceVariant,
            Self::Destructive => ColorToken::Error,
            Self::Quiet => theme::CLEAR,
        }
    }
    pub fn foreground(self) -> ColorToken {
        match self {
            Self::Primary => ColorToken::OnPrimary,
            Self::Destructive => ColorToken::Surface,
            _ => ColorToken::OnSurface,
        }
    }
}
pub const RADIUS: i32 = 6;
pub const GAP: i32 = 8;
pub const FOCUS_WIDTH: i32 = 2;

/// Compact desktop menu geometry, shared by placement and row composition.
pub const MENU_ROW_HEIGHT: i32 = 32;
pub const MENU_INSET: i32 = 4;
pub const MENU_GAP: i32 = 1;
pub const fn menu_height(items: i32, separators: i32) -> i32 {
    let nodes = items + separators;
    items * MENU_ROW_HEIGHT + separators + (if nodes > 0 {nodes - 1} else {0}) * MENU_GAP + 2 * MENU_INSET + 2
}
