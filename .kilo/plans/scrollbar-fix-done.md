# Scrollbar thumb drag fix (done)

## Problem
The scrollbar thumb was purely decorative: its container had `IgnoreHitTest`, the thumb entity had no `HitTarget`, and there was no drag handling, so dragging the thumb did not update the content offset.

## Changes

### `crates/fm/src/ui/scrollbar.rs`
- Added `HitTarget` to the scrollbar (thumb) entity via an `attach` callback (`scrollbar_attach`), so the thumb can receive pointer input.
- Added drag handling through `input` callbacks:
  - `scrollbar_drag_start` records the content offset and pointer position where the drag began (stored in a new private `ScrollBarThumbDrag` component on the thumb entity).
  - `scrollbar_drag_move` translates the thumb delta into a content offset using the inverse of the paint mapping (`shift = position * travel / max`, so `Δ_content = Δ_pointer * max / travel`) and clamps to `[0, max_offset]`, then writes the new `ScrollOffset` on the scroll target.
  - `scrollbar_drag_end` / `scrollbar_drag_cancel` clear the drag state.
- Paint logic is unchanged when not dragging.

### `crates/fm/src/ui/content.rs`
- Removed `IgnoreHitTest` from both scrollbar track containers (grid and list) so the thumb can be hit-tested and dragged.
- Removed the now-unused `IgnoreHitTest` import.

## Verification
- `cargo build -p fm`: `scrollbar.rs` and `content.rs` compile with no errors or warnings.
- Remaining build errors are pre-existing in `status.rs`/`theme.rs` (files outside the allowed edit scope, already modified in the working tree before this session) and are unrelated to this change.
