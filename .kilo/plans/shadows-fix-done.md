# Shadows/elevation fix — done

Added subtle `DropShadow` + `OffscreenRender` to elevated components across 4 files in `crates/fm/src/ui/`.

## Changes

**dialog.rs** — Sheet backdrops (all sheets)
- Imported `OffscreenRender` and `DropShadow`
- Wrapped the sheet backdrop (`View` with `border_radius: 12`, `bg_color: Surface`) in a centered container
- Added `id: "sheet_backdrop"` + `OffscreenRender::new()` to the backdrop
- Added a sibling `View` with `id: "sheet_shadow"` using `DropShadow::new(id("sheet_backdrop"))` with `blur_radius: 10`, `offset: (0, 3)`, `opacity: 80`
- Both shadow and backdrop use `position: Position::Absolute, left: 0, top: 0` inside the centered wrapper so they overlap

**menu.rs** — Context menu panel
- Imported `OffscreenRender` and `DropShadow`
- Added `id: "menu_panel"` + `OffscreenRender::new()` to the menu `Column`
- Added a sibling `View` with `id: "menu_shadow"` using `DropShadow::new(id("menu_panel"))` with `blur_radius: 10`, `offset: (0, 3)`, `opacity: 80`
- Both use matching absolute position (`left: menu_x`, `top: menu_y`, `width: 214`, `height: menu_height`)

**status.rs** — Toast notifications
- Imported `OffscreenRender` and `DropShadow`
- Inlined all three notification cards (index 0, 1, 2) in `notices()` with hardcoded string ids (`notice_0`/`notice_shadow_0`, etc.) because mirui's `id` property and `id()` require string literals, not variables
- Each card: `id: "notice_N"` + `OffscreenRender::new()` + `position: Position::Absolute, left: 0, top: 0`
- Each shadow: `id: "notice_shadow_N"` + `DropShadow::new(id("notice_N"))` with `blur_radius: 10`, `offset: (0, 3)`, `opacity: 80`
- Removed now-unused `notice_card` helper (inlined into `notices()`)

**chrome.rs** — Primary action button (New Folder)
- Imported `OffscreenRender` and `DropShadow`
- Wrapped `new_folder_button`'s `Button` in a parent `View`
- Added `position: Position::Absolute, left: 0, top: 0` to the Button and `id: "toolbar_new_folder"` + `OffscreenRender::new()`
- Added a sibling `View` with `id: "new_folder_shadow"` using `DropShadow::new(id("toolbar_new_folder"))` with `blur_radius: 10`, `offset: (0, 3)`, `opacity: 80`

## Shadow values
- blur_radius: 10 (within 8–12)
- offset: (0, 3) (within 0, 2–4)
- opacity: 80 (within 60–100)
- color: `Shadow` token (default)

## Verification
- `cargo build -p fm` passes

## Note on paint order
The `ui!` macro requires the source entity to be defined before the shadow that references it (no forward references in component literals). This means the shadow is defined after the source in the tree. The shadow uses `DropShadow` (priority 60) which reads the source's offscreen texture.