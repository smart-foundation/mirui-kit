# UX plan: clickthrough / hover / layering for popups in crates/fm

**Scope (read-only):** `crates/fm/src/ui/menu.rs`, `crates/fm/src/ui/mod.rs`,
`crates/fm/src/sync.rs`, `crates/fm/src/ui/status.rs`. No files are modified.

This plan answers five UX questions about the context menu / toast layering,
then proposes fixes for macOS-like non-modal popups. Every claim is cited to
`file:line`.

---

## 1. When a context menu / settings popup is open, do clicks pass to items
behind, or are they swallowed?

**Short answer:** They pass through AND dismiss the menu. The backdrop does not
swallow hits, so a press outside the panel reaches whatever is underneath (a
tile, the chrome, …), while the synchroniser additionally dismisses the menu
when the press lands outside the panel. You can get both a background activation
and a menu dismissal on a single press.

### Evidence

- `crates/fm/src/ui/menu.rs:63` — the menu backdrop is `IgnoreHitTest`:
  ```
  // The backdrop covers the window but ignores hits, so a press
  // outside the menu reaches whatever is underneath (a tile, the
  // chrome, ...).
  IgnoreHitTest
  ```
  The backdrop spans the whole window (`menu.rs:45-57`) but only while
  `fm.menu_target().is_some()` (`menu.rs:57`). Because it ignores hits, a press
  outside the anchored panel falls through to widgets behind it.

- `crates/fm/src/ui/menu.rs:65-103` — the anchored panel `Column` is a normal
  (hit-testing) `View` positioned at `fm.menu_x()` / `fm.menu_y()`,
  `visible` only when a target is set (`menu.rs:78`). A press on the panel is
  captured by the panel / its `menu_row` buttons (`menu.rs:185` `on Tap`).

- `crates/fm/src/sync.rs:74-79` — on every new pointer sequence the
  `drag_tracking` system dismisses an open menu when the press is outside the
  panel:
  ```
  if state.menu_open() && !state.menu_contains(x, y) {
      state.close_menu();
  }
  ```
  So an outside press does two things: (a) passes through to the background
  (IgnoreHitTest), and (b) closes the menu via `drag_tracking`.

- `crates/fm/src/sync.rs:50-51` — the comment claims the backdrop "is a hit
  target of its own, so a press outside the menu is swallowed by it". This
  comment is stale/contradicted by the actual code: the backdrop is
  `IgnoreHitTest` (`menu.rs:63`), so it does NOT swallow; dismissal is done by
  `drag_tracking` (`sync.rs:74-79`), not by hit capture.

### Consequence

This is already close to non-modal clickthrough: clicking outside a file menu
activates the tile/chrome behind while dismissing the menu. A macOS-style
non-modal popup usually dismisses on outside click WITHOUT activating what is
behind, so the current pass-through may be more aggressive than intended.

---

## 2. Does dragging behind a popup move background content?

**Short answer:** Yes. Drag tracking runs continuously on pointer position and
does not consult whether a menu is open, so a drag that starts behind/under an
open menu still updates the drop target, starts a drag, and moves the drag
badge. The popup does not modally block background dragging.

### Evidence

- `crates/fm/src/sync.rs:52-149` — `drag_tracking` reads the cursor directly
  (`cursor: res!(mirui::input::event::PointerCursor)`, line 54) and the zones
  under it (`query!(Entity, DropZone)`, line 56). It never checks
  `state.menu_open()` before computing the drag / drop target. The drop target
  is set at line 148 (`state.set_drop_target(target)`) and the drag badge is
  driven by `fm.dragging()` / `fm.drag_x()` / `fm.drag_y()`.

- `crates/fm/src/ui/menu.rs:109-134` — `drag_badge` follows the pointer
  (`drag_x() + 12`, `drag_y() + 12`, lines 114-115) while `fm.dragging().is_some()`
  (line 123). It is rendered after `menu_layer` in the shell
  (`ui/mod.rs:56`), so it paints on top.

- `crates/fm/src/ui/mod.rs:56` — `drag_badge (fm)` is the last layer in the
  shell tree, so it sits above the menu and toast.

### Consequence

Dragging behind an open menu moves background content (drop target updates,
badge follows). There is no modal drag lockout while a popup is open.

---

## 3. Hover leakage — when hovering a menu, do items below get hovered?

**Short answer:** Yes, at the model level. `drag_tracking` unconditionally
calls `state.set_hovered(source)` (`sync.rs:114`) on every frame while the
pointer moves, regardless of whether a menu is open, so the "file under the
pointer" is updated even while a menu covers it. The menu rows themselves paint
their own hover via `HoverSurface` (`menu.rs:160-164`), but the background tile
hover state in the model still leaks.

### Evidence

- `crates/fm/src/sync.rs:112-114`:
  ```
  // The file under the pointer, whether the button is down or not: a right
  // click opens the menu of that file, not the menu of the selection.
  state.set_hovered(source);
  ```
  This runs on pointer move with no `state.menu_open()` guard, so hovering the
  menu backdrop area still updates the hovered file behind it.

- `crates/fm/src/ui/menu.rs:149-164` — each `menu_row` is a `Button` with a
  `HoverSurface` (`hover: theme::MENU_HOVER.into()`), so the menu panel paints
  its own hover. The panel `Column` is a normal hit-testing `View`
  (`menu.rs:65`), so hovering the panel captures hover locally; the backdrop
  area outside the panel passes hover through (IgnoreHitTest) to whatever is
  behind, consistent with `set_hovered` running unconditionally.

- `crates/fm/src/ui/menu.rs:83-102` — the menu only renders when
  `fm.menu_target().is_some()` (`menu.rs:78`), but nothing in `menu.rs` tells
  `drag_tracking` to stop updating `set_hovered` while the menu is open.

### Consequence

Hover state in the model leaks past an open menu: the file beneath the pointer
is still marked hovered even though the menu covers it. This can produce stale
highlight / right-click-target confusion (a right click opens the menu of the
file under the pointer, per `sync.rs:112-113`).

---

## 4. Toast layering vs menu

**Short answer:** The menu paints on top of the toast. In `shell`, `toast` is
added before `menu_layer`, so `menu_layer` is a later child and wins both paint
order and hit test when both are visible. The shell comment confirms this
intent ("The message sits below the menu: when both are on screen the menu wins
the hit test").

### Evidence

- `crates/fm/src/ui/mod.rs:49-54`:
  ```
  options_layer (fm)
  // The message sits below the menu: when both are on screen the
  // menu wins the hit test, so its dismissal is not swallowed.
  toast (fm)
  menu_layer (fm)
  dialog_layer (fm)
  palette_layer (fm)
  drag_badge (fm)
  ```
  Later children render on top, so `menu_layer` (line 53) is above `toast`
  (line 52), which is above `options_layer` (line 49). `dialog_layer` (line 54)
  and `palette_layer` (line 55) are above the menu, and `drag_badge` (line 56)
  is topmost.

- `crates/fm/src/ui/status.rs:65-95` — `toast` renders `notices`, which spans
  the whole window (`position: Absolute`, `width/height: 100%`, lines 68-72)
  but uses `IgnoreHitTest` (line 80) so the layer does not swallow clicks;
  only the cards are hit targets (`HitTarget`, line 126). `visible` only when
  `fm.notice_count() > 0` (line 76).

### Consequence

Toast and menu can coexist; the menu is on top and its hit test takes
precedence. The toast layer itself is already non-swallowing (`IgnoreHitTest`),
so it does not block clicks behind it — consistent with a non-modal tone, though
it sits below the menu so a menu dismisses rather than being obscured by a
toast.

---

## 5. Proposed fixes for macOS-like non-modal popups

macOS context menus are non-modal but not fully clickthrough: an outside click
dismisses the menu without activating the item behind; dragging is generally
paused or redirected while a menu is open; hover does not leak to items beneath
the menu. The current code is partially there (backdrop already ignores hits)
but overshoots on clickthrough (background activates on outside press) and on
hover leakage (`set_hovered` runs unconditionally).

### Proposed changes (cite targets)

**A. Stop background activation on outside press while keeping dismissal.**
- In `crates/fm/src/sync.rs:74-79`, the outside-press dismissal already runs.
  Add a guard so that when the press is outside the menu panel, the event is
  marked consumed / not forwarded to background widgets, OR keep pass-through
  for the backdrop but prevent the underlying tile's `on Tap` from firing when
  a menu is open. Simplest: in `drag_tracking`, when `state.menu_open()` and the
  press is outside the panel, call `state.close_menu()` and suppress the
  background activation for that press (e.g. record the press seq as consumed so
  the tile handler treats it as handled).
- Target files: `crates/fm/src/sync.rs:74-79` (dismissal), and wherever the
  background tile handles `on Tap`/press (not in the four files read; would be
  in `crates/fm/src/ui/content.rs` or similar).

**B. Prevent hover leakage while a menu is open.**
- In `crates/fm/src/sync.rs:112-114`, guard `state.set_hovered(source)` so it
  does not run when a menu is open:
  ```
  if !state.menu_open() {
      state.set_hovered(source);
  }
  ```
  This keeps the menu's own row hover (`menu.rs:160-164`) but stops the model
  from marking a background file as hovered under an open menu.
- Target file: `crates/fm/src/sync.rs:112-114`.

**C. Decide drag behaviour under an open menu (macOS: drags typically do not
  start/complete through a context menu).**
- Add a `state.menu_open()` guard around drag initiation in
  `crates/fm/src/sync.rs:128-146` (start_drag / press_tile / drag_to), so a
  drag begun while a menu is open is suppressed or the menu is dismissed first.
  Alternatively dismiss the menu when a press transitions into a drag
  (`DRAG_THRESHOLD`, line 34) so the drag proceeds on a clean background.
- Target file: `crates/fm/src/sync.rs:128-146`.

**D. Keep toast below menu but make toast dismissal robust.**
- The current ordering already puts `menu_layer` above `toast`
  (`ui/mod.rs:52-53`). No change needed for layering; the toast already uses
  `IgnoreHitTest` (`status.rs:80`) so it never swallows clicks behind it.
- If a toast and menu overlap visually, the menu wins hit test, which matches
  the shell comment (`ui/mod.rs:50-51`).

**E. Align the stale comment.**
- `crates/fm/src/sync.rs:50-51` says the backdrop swallows presses, but the
  code uses `IgnoreHitTest` (`menu.rs:63`) and dismisses via `drag_tracking`
  (`sync.rs:74-79`). Update the comment to describe the actual behaviour.

### Summary of current vs desired behaviour

| Question | Current behaviour | macOS-like desired |
|---|---|---|
| Clicks behind open menu | Pass through AND dismiss (`menu.rs:63`, `sync.rs:74-79`) | Dismiss without activating background |
| Dragging behind popup | Moves background content / updates drop target (`sync.rs:112-148`) | Suppressed or menu dismissed first |
| Hover leakage | Model hover updated under menu (`sync.rs:114`) | No hover leakage under menu |
| Toast vs menu layering | Menu on top of toast (`ui/mod.rs:52-53`) | Keep; menu wins hit test |
| Stale comment | `sync.rs:50-51` contradicts code | Update comment |

All citations are to the four read-only files listed above; no files were
modified.