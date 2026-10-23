# UI Components Fix — Done

## Problem
The file manager (`crates/fm`, mirui 0.51.0) lacked tooltip/badge/empty-state/skeleton/segmented-control tokens in `theme.rs`, and `kit.rs` only had `pad*` helpers. The view switcher (`chrome.rs`) and sidebar place count badge (`sidebar.rs`) were hand-rolled.

## Changes Made

### `crates/fm/src/theme.rs`
Added color tokens and dark + light palette entries for:
- `BADGE` / `BADGE_TEXT` — small count badge background and text.
- `SKELETON` / `SKELETON_SHIMMER` — loading skeleton base color and shimmer highlight.
- `SEGMENTED_BG` / `SEGMENTED_SELECTED` — segmented control background and selected segment.
- `TOOLTIP_BG` / `TOOLTIP_TEXT` — tooltip bubble background and text.

### `crates/fm/src/ui/kit.rs`
Added five shared components:
- `badge(count: u32)` — small rounded pill with count text (`BADGE`/`BADGE_TEXT`).
- `empty_state(icon, title, body)` — centered icon + heading + body for empty grids.
- `skeleton` — `#[component]` + `#[view]` with a looping shimmer driven by mirui's `#[animate]` system (`SKELETON`/`SKELETON_SHIMMER`); a shimmer bar slides across a rounded base rect.
- `tooltip(text)` — hover text bubble (`TOOLTIP_BG`/`TOOLTIP_TEXT`).
- `segmented_control(fm)` — icon grid / list segmented control for view mode switching (`SEGMENTED_BG`/`SEGMENTED_SELECTED`).

Also kept the existing `pad`, `pad_h`, `pad_v` helpers.

### `crates/fm/src/ui/chrome.rs`
- Replaced the hand-rolled `view_switch` compose function with `segmented_control (fm)` in the toolbar.
- Removed the old `view_switch` implementation and its now-unused `ViewMode` import.
- Kept the same `toolbar_view_icons` / `toolbar_view_list` ids and `fm.set_view(...)` behavior, now themed via `theme::SEGMENTED_BG` / `theme::SEGMENTED_SELECTED`.

### `crates/fm/src/ui/sidebar.rs`
- Replaced the hand-rolled count `Text` in each place row with `badge (count)`.
- `count` is computed as `data::place_count(place) as u32` before the `ui!` block.

## Verification
- `cargo build -p fm`: compiles cleanly, no errors or warnings.

## Notes
- `empty_state`, `skeleton`, and `tooltip` are added to `kit.rs` and compile but are not yet wired into existing views; they are ready to be dropped into empty grids, loading states, and hover targets respectively.
