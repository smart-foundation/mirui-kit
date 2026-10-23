# Clickthrough fix — done

## Scope
Modified only `crates/fm/src/sync.rs` (the `drag_tracking` system).

## Changes

1. **Hover guard** — `state.set_hovered(source)` is now guarded with `if !state.menu_open()` (sync.rs:119), so background items are not hovered while a context menu is open.

2. **Drag suppression** — Captured whether a menu was open at the start of the pointer sequence (`menu_was_open`, sync.rs:76-79) and suppress drag initiation (`press_tile` / `start_drag` / `drag_to` / `set_drop_target`) when `menu_was_open && !state.menu_contains(x, y)` (sync.rs:137-160). A press outside an open menu still dismisses the menu (existing logic at sync.rs:80-82 unchanged) but no longer also starts dragging background content.

3. **Menu dismissal preserved** — The outside-press menu dismissal (`close_menu` at sync.rs:80-82) is untouched.

## Verification
- `cargo build -p fm` — succeeds.
- `cargo test -p fm --test interaction` — all 7 tests pass, including `the_context_menu_does_not_swallow_a_tap_outside_it` (selection via widget `on Tap` handlers is independent of `press_tile`, so it still reaches the tile).
