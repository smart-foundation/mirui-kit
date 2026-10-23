# Layout fix — done

## Scope
crates/fm file manager on mirui 0.51.0. Modified: `crates/fm/src/ui/chrome.rs`, `crates/fm/src/ui/sidebar.rs`, `crates/fm/src/ui/content.rs`, `crates/fm/src/model.rs`, plus `crates/fm/src/ui/mod.rs` (to wire the new titlebar into the shell).

## Problem 1 — Toolbar place column
`chrome.rs`: the 240 px place column is now left of the grow spacer. Toolbar order is: back/forward → place column → grow spacer → actions (palette, new folder, view switcher, search, options, inspector, theme). The nav/actions split is restored. Traffic lights were removed from the toolbar row (they are window chrome, not content actions).

## Problem 2 — Traffic lights placement
Evaluated titlebar vs sidebar. Sidebar placement was rejected (window controls don't belong in the places navigation pane). Implemented titlebar placement: a new `titlebar()` compose function in `chrome.rs` holds the traffic lights plus a "Files" title, and `mod.rs` renders it as a row above the toolbar. Window decoration is now separated from the content toolbar.

## Problem 3 — Sidebar
`sidebar.rs`: row gap increased from 2 to 4 (was tight). Width kept at 216 (fits place names, icon, and badge). The count badge already uses the shared `badge(count)` component from `kit.rs` (done by the components agent), so no further badge change was needed.

## Problem 4 — Content grid responsive sizing
`content.rs`: tile sizing is now consistent — cell uses `TILE_WIDTH`/`TILE_HEIGHT` constants instead of magic numbers 104/98, and `grid_row` uses `TILE_HEIGHT`. The fixed 8-column `COLUMN_INDEXES` was replaced with a responsive column count.
`model.rs`: added `pub const TILE_WIDTH: i32 = 104`; added `grid_columns()` which derives the column count from the window width (`viewport`), clamped to [4, `GRID_COLUMNS`] and falling back to `GRID_COLUMNS` when the size is unknown. `viewport` is now `#[observe]` so the grid reflows on resize. `grid_rows`/`grid_row_key` use `grid_columns()`.
At the test window width (1180 px) this still yields 8 columns, so existing grid expectations (`grid_rows() == 4`, `grid_height() == 4*98+3*10`, tiles of 104×98) are unchanged.

## Problem 5 — Command palette shortcut
`model.rs` already registers `("Open Command Palette", "Cmd P", Command::OpenPalette)` at the end of `COMMANDS` (array length 17), with the `OpenPalette` variant and `run_command` arm present. Verified — no change needed.

## Verification
- `cargo build -p fm` — succeeds.
- `cargo test -p fm --test features` — 19 passed, 1 failed.

## Note (pre-existing, not a regression)
`a_context_menu_row_can_trash_a_file` fails. menu.rs and the feature test are unchanged from HEAD (where the test passes), and the failure is unrelated to these layout edits: I confirmed it still fails with my titlebar, grid, sidebar, and viewport changes all reverted, and it fails identically with `viewport` both observed and plain. It is a pre-existing breakage in the working-tree baseline (introduced by earlier agents' app-source changes), not caused by this task's changes.
