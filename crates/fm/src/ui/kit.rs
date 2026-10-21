//! Small shared building blocks for the chrome.

use mirui::prelude::*;

/// Padding with explicit edges.
pub const fn pad(top: i32, right: i32, bottom: i32, left: i32) -> Padding {
    Padding {
        top: Dimension::px(top),
        right: Dimension::px(right),
        bottom: Dimension::px(bottom),
        left: Dimension::px(left),
    }
}

/// Padding on the horizontal edges only.
pub const fn pad_h(v: i32) -> Padding {
    pad(0, v, 0, v)
}

/// Padding on the vertical edges only.
pub const fn pad_v(v: i32) -> Padding {
    pad(v, 0, v, 0)
}
