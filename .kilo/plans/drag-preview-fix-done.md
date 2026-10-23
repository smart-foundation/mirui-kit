# Drag Preview Fix — Done

## Problem
When dragging a folder, the popup near the cursor was an abstract 190×26 blue text pill — no icon, no shadow, not the actual item being dragged.

## Changes Made

### `crates/fm/src/ui/menu.rs`

**`drag_badge` (lines 108-150)** — Replaced the abstract text pill with the actual dragged item's icon and label:

- **Removed**: fixed 190×26 pill with `ColorToken::Primary` background, centered `Text` with `drag_label()`, `OnPrimary` text color.
- **Added**: card-style badge containing `KindIcon` (20px) + `Text` label (`drag_label()`, `OnSurface`, `SMALL`).
- **Styling**: `ColorToken::Surface` background, 1px `theme::SEPARATOR` border, 6px `border_radius`, `pad(8, 10, 8, 10)` padding — reads as a physical card under the pointer.
- **Positioning**: Still absolutely positioned at `drag_x + 12, drag_y + 12`, following the cursor (unchanged behavior).
- **Visibility**: Still gated on `fm.dragging().is_some()`.

**Imports**: Added `crate::ui::hover::KindIcon` (was already importing `HoverSurface`).

### Drag state (reference only, no changes)
- `model.rs`: `dragging: Option<u32>` (drag state), `drag_label()` (dragged item name), `drag_x`/`drag_y` (cursor position) — all already existed and are used by the badge.

### Notes
- `mirui::ui::widgets::DropShadow` + `OffscreenRender` were considered for elevation/shadow, but `id("entity")` forward references are not supported inside component struct literals in the `ui!` macro, so the DropShadow wiring could not be completed. The badge is visually distinct as a physical card via surface background, separator border, rounded corners, and padding, satisfying the "visually distinct as a physical object" fallback.
- `cargo build -p fm`: compiles cleanly.
- `cargo test -p fm`: 9 tests pass.
- `cargo clippy -p fm`: no warnings.
