# Window sizing fix — done

## Problem
On GNOME the mirui chrome (titlebar + toolbar) was sized edge-to-edge with square corners and filled the whole SDL window, so it did not look like a floating desktop window.

## Changes (three files only)

### `targets/desktop/src/main.rs`
- Increased the SDL window size so it is larger than the internal chrome: `WIDTH` 1180 → 1400, `HEIGHT` 720 → 900. The chrome now floats inside the window instead of filling it.

### `crates/fm/src/ui/chrome.rs`
- `titlebar()`: added `border_radius: 12` (modern rounded corners), kept its padding.
- `toolbar()`: added `border_radius: 12` (modern rounded corners), kept its padding.

### `crates/fm/src/ui/mod.rs`
- Imported `crate::theme` and `crate::ui::kit::pad`.
- Wrapped `titlebar` + `toolbar` in a padded, centered `View` (`padding: pad(10, 24, 0, 24)`, `align: Center`) so the chrome has space around it and does not touch the window edges.
- The wrapping `Column` uses `bg_color: theme::TOOLBAR` and `border_radius: 12`, giving a single clean rounded floating chrome bar (titlebar/toolbar junction is filled by the same-color column, so corners stay seamless).

## Verification
- `cargo build -p fm` — succeeded.
- `cargo build -p desktop` — succeeded.

No other files were modified.
