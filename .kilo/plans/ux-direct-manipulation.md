# Direct-Manipulation UX Audit — mirui 0.51.0 file manager (crates/fm)

**Status:** read-only audit. No source was modified.
**Scope:** direct-manipulation concepts only — in-place editing, command-palette affordance, layout/placement, component richness. Drag-and-drop is explicitly out of scope.
**Files cited:** all under `/home/lufuno/mirui/crates/fm/src/` unless noted.

---

## 1. In-place editing — new folder currently a modal dialog

### Current behaviour

`new_folder_sheet` lives in `crates/fm/src/ui/dialog.rs:79-124`. It is a centred modal sheet behind a dimming backdrop (`dialog_layer`, `dialog.rs:22-76`), width `SHEET_W = 420` (`dialog.rs:18`). The sheet is built once and shown/hidden (`dialog.rs` module doc, lines 1-6). Opening is triggered from:

- `chrome.rs:159-190` `new_folder_button` — toolbar button, calls `fm.set_folder_text(""); fm.open_dialog(DialogKind::NewFolder);` (`dialog.rs:186-187`)
- `model.rs:670` `run_command(Command::NewFolder)` → `open_dialog(DialogKind::NewFolder)`
- `model.rs:753-768` `create_folder` — trims, rejects empty, pushes a `CreatedFolder`, bumps `names_revision`, closes dialog, recomputes, says "New folder created" (`model.rs:753-768`)

The sheet's `TextInput` is `id: "sheet_new_folder"` (`dialog.rs:98`), backed by `fm.folder_document()` (`dialog.rs:109`), placeholder "untitled folder" (`dialog.rs:110`). `buttons(fm, 1)` at `dialog.rs:121` maps action `1` → `create_folder` (`dialog.rs:334-337`). Sheet height for NewFolder/Rename is `168` (`model.rs:1154`).

`rename_sheet` (`dialog.rs:127-172`) is the same modal pattern for renames, action `2` → `fm.rename(id, &name)` (`dialog.rs:339-342`).

### Assessment

The new-folder flow forces a mode switch: the user leaves the grid, types in a centred dialog, then commits or cancels. This is the classic "sheet" pattern, not direct manipulation. On macOS/Windows, creating a folder highlights a new row in-grid with an editable name field and inline Save/Cancel — the user never leaves the context, and the empty state of "untitled" is discoverable by direct editing.

### Proposal: in-place rename flow for new folders

Introduce a light-weight in-place editing state alongside the existing modal (keep the modal as fallback/accessibility path, but make in-place primary):

1. **Model additions** (`model.rs`):
   - New field `editing_id: Option<u32>` — the id of the entry currently being renamed/created inline.
   - New field `edit_text: String` — the buffer for the in-place editor (reuse `folder_document` or add a dedicated one).
   - `begin_inline_edit(id)` — set `editing_id = Some(id)`, select the text, focus the field.
   - `commit_inline_edit()` — trim, reject empty, call `create_folder`/`rename`, clear `editing_id`.
   - `cancel_inline_edit()` — for a brand-new folder, remove the just-created `CreatedFolder`; for a rename, restore the old name; clear `editing_id`. Commit on Enter, cancel on Escape, commit on blur/outside tap.

2. **Grid tile change** (`content.rs:127-183` `cell`):
   - When `fm.editing_id() == Some(id)`, swap the static `Text` (`content.rs:167-173`) for a `TextInput` bound to `edit_text`, auto-focused, with `Placeholder("untitled folder")` for new folders.
   - Tile keeps `104 x 98` (`content.rs:132-133`) but the text field replaces the label; hover/selection/DropZone preserved.
   - For new folders, insert the new `CreatedFolder` id into `grid_entry`/`visible` immediately (so it is addressable), mark it `editing_id`, then commit/remove on confirm/cancel.

3. **Toolbar button change** (`chrome.rs:159-190`):
   - `new_folder_button` on Tap no longer opens the modal. Instead it calls `fm.begin_inline_create()` which appends an empty `CreatedFolder` at the end of the current listing and begins inline edit on it. The modal remains reachable via the command palette ("New Folder", `model.rs:145`) and keyboard.

4. **Interaction details:**
   - Inline edit should only start on a single click (or a dedicated "rename" affordance), never on the press that begins a drag — keep drag-start (`model.rs:802-806` `press_tile`) separate so direct-manipulation rename and drag-and-drop do not conflict.
   - Escape / click outside cancels; Enter commits. Show inline Save/Cancel as small text buttons at the right edge of the tile only while editing (a mini "buttons row" patterned on `dialog.rs:282-330`).

Rationale: one tap creates and names; no context switch; the new folder appears in its final grid position immediately; consistent with rename (same inline editor, two code paths). Keep the modal for accessibility and for users who prefer it, but default to in-place.

---

## 2. Command palette button — visible button already exists, but mislabeled

### Current behaviour

There **is** already a visible palette button in the toolbar: `palette_button` at `chrome.rs:124-155`, id `"toolbar_palette"` (`chrome.rs:128`), placed after the grow spacer (`chrome.rs:48`) and before `new_folder_button` (`chrome.rs:49`). On Tap it calls `fm.open_palette()` (`chrome.rs:152`). The palette itself is `palette_layer` (`palette.rs:20-109`), a centred 520x360 overlay (`palette.rs:57-58`) with a filter field (`palette.rs:74-98`) and command rows (`palette.rs:113-172`). Commands are defined in `model.rs:124-161` (`COMMANDS`, 16 entries).

### Problem

The button label reads **"Ctrl P"** (`chrome.rs:145`), but every shortcut in `COMMANDS` (`model.rs:144-161`) uses **Cmd** (Cmd N, Cmd 1, Cmd T, Cmd I, Cmd H, Ctrl Shift S, Esc). The platform is clearly macOS-flavoured (traffic lights `chrome.rs:61-69`, Cmd shortcuts), so "Ctrl P" is wrong on two counts:

1. It misstates the modifier (Ctrl vs Cmd).
2. There is no "Cmd P → open palette" entry in `COMMANDS` at all — the palette is opened by `fm.open_palette()` from the button (`chrome.rs:152`) or presumably a shortcut handler not visible in `COMMANDS`. So the label promises a shortcut that does not exist in the command table.

### Recommendation

**Keep the visible palette button — it is good convenience UX** — but fix it:

- Rename label to **"Cmd P"** (`chrome.rs:145`) to match platform convention, or better, drop the text and use an icon-only button to avoid hardcoding a modifier (the app is cross-platform in principle; `model.rs` hardcodes Cmd everywhere).
- Consider changing the icon from `icons::SEARCH` (`chrome.rs:143`) to a distinct palette/magnifying-glass-plus icon, since SEARCH is already used by `search_field` (`chrome.rs:256`) and the palette is "commands", not "search".
- Add an entry to `COMMANDS` (`model.rs:144-161`) like `("Open Command Palette", "Cmd P", Command::OpenPalette)` and a `Command::OpenPalette` variant so the shortcut is real and discoverable, not just button text.

A visible button is worth keeping: the palette has 16 commands (`model.rs:144-161`) with no visible entry point except the button. Keyboard-only hides discoverability. The button costs ~104px (`chrome.rs:129`) and is already implemented; just correct the label and register the shortcut.

---

## 3. Layouts & placement

### Toolbar (`chrome.rs:18-56`)

- Height 52 (`chrome.rs:21`), full-width row, `column_gap: 8` (`chrome.rs:25`), padding `pad(0, 12, 0, 12)` (`chrome.rs:26`).
- Grouping (left to right): traffic lights → separator → back/forward → **place name column (240 wide, `chrome.rs:32`)** → grow spacer → palette → new folder → view switch → search → options → inspector → theme.

**Critique:** The place-name column (240px, two lines of text `chrome.rs:33-46`) sits in the *middle* of the toolbar, breaking the conventional "navigation left, actions right" split. It pushes the action cluster (palette, new folder, view, search, options, inspector, theme) to the far right, far from back/forward. The two-line path ("place name" + "/Users/demo/Cloud", `chrome.rs:33-46`) is the demo hardcoded path, not a real breadcrumb.

**Recommendations:**
- Move the place/path indicator to the left, immediately after back/forward, or replace with a compact breadcrumb. Keep primary navigation (back/forward/place) grouped together on the left.
- Keep actions (palette, new folder, view switch, search, options, inspector, theme) grouped on the right — they already are, but tighten `column_gap` from 8 to 6 for visual cohesion.
- The grow spacer (`chrome.rs:47`) correctly separates nav from actions; preserve it.

### Window controls / traffic lights

`traffic_lights` (`chrome.rs:61-69`, three 12x12 dots at `chrome.rs:64-66`) is inside the toolbar row, at the leading edge. This is macOS-style but macOS places traffic lights in the *titlebar*, above/independent of the toolbar. Here they share the 52px toolbar row.

**Recommendation:** For a true macOS look, promote the window to a titlebar + toolbar split: traffic lights in a titlebar above the toolbar, toolbar starting below. If keeping unified, left-align traffic lights with a small left margin (currently flush at `left: 0` via row start) so they don't collide with back button. Low priority; acceptable as-is for a demo.

### Sidebar (`sidebar.rs:20-101`)

- Width 216 (`sidebar.rs:24`), full height, `bg_color: SurfaceVariant` (`sidebar.rs:26`), padding `pad(10, 8, 10, 8)` (`sidebar.rs:27`), `row_gap: 2` (`sidebar.rs:28`).
- Header "PLACES" (`sidebar.rs:30-36`), then `place_row` per place (`sidebar.rs:37-39`). Each row height 28 (`sidebar.rs:61`), border_radius 6 (`sidebar.rs:62`), padding `pad_h(6)` (`sidebar.rs:65`), icon 15x15 (`sidebar.rs:83-84`), count badge text at right (`sidebar.rs:92-96`).

**Critique:** 216px is narrow but workable. `row_gap: 2` is very tight — rows feel cramped. The count text ("N items", `sidebar.rs:92-96`) functions as a badge but is just plain `Text` with no badge styling (no background pill, no contrast role). Density: 28px rows with 2px gaps = ~30px pitch; at 5 places that's ~150px + header, fits the sidebar height but leaves little room for future sections (e.g., "TAGS", "DEVICES").

**Recommendations:**
- Increase `row_gap` from 2 to 6-8 for breathing room (`sidebar.rs:28`).
- Keep width 216; consider 240 if adding sections later.
- Promote the count into a real **Badge** component (see §4) — a small pill at the row's trailing edge, not end-aligned body text.
- Add a trailing disclosure chevron on place rows to signal navigability (currently tap goes straight to `set_place`, `sidebar.rs:97-99`, no affordance).

### Content grid (`content.rs:46-253`)

- Icon tile: 104x98 (`content.rs:132-133`), `column_gap: 6` within `grid_row` (`content.rs:117`), `row_gap: 10` (`content.rs:64`), grid padding `pad(16,16,16,16)` (`content.rs:59`).
- `GRID_COLUMNS = 8` (`model.rs:173`), `TILE_HEIGHT = 98` (`model.rs:179`), `GRID_ROW_HEIGHT = 108` (`model.rs:176`), `ICON_GRID_MAX = 32` (`model.rs:166`) = 4 rows of 8.
- List row: height 26 (`content.rs:199`), `LIST_ROW_MAX = 48` (`model.rs:182`), icon 14x14 (`content.rs:29`, `content.rs:214-215`).

**Critique:** Grid density is high and fixed: 8 columns regardless of window width; tiles are 104px wide so 8 columns = 832px + gaps ≈ 866px content width required before horizontal overflow. There is no responsive reflow — on narrow windows tiles clip. Row height 98px + 10px gap is comfortable (icon 36px `model.rs:346` + name). List rows at 26px are dense but standard.

**Recommendations:**
- Make column count responsive: derive `GRID_COLUMNS` from available width instead of a constant 8 (`model.rs:173`). Minimum 4 columns on narrow windows.
- Keep tile 104x98 but allow `icon_size` to scale (already supported, `model.rs:346` default 36, `set_icon_size` `model.rs:461-467`) — expose a view-option for icon size.
- List view: 26px row height is fine; consider 30-32px if adding a badge/avatar column later.

### Summary of layout recommendations

| Area | File:line | Issue | Recommendation |
|------|-----------|-------|----------------|
| Toolbar grouping | `chrome.rs:32-46` | Place column in middle breaks nav/actions split | Move place/path left of grow spacer, keep actions right |
| Traffic lights | `chrome.rs:61-69` | In toolbar row, not titlebar | Split titlebar/toolbar for macOS fidelity (low priority) |
| Sidebar width | `sidebar.rs:24` | 216px, tight | Keep 216, or 240 if adding sections |
| Sidebar density | `sidebar.rs:28` | `row_gap: 2` too tight | Raise to 6-8 |
| Sidebar badge | `sidebar.rs:92-96` | Count is plain text | Real Badge pill component |
| Grid columns | `model.rs:173` | Fixed 8, not responsive | Derive from width, min 4 |
| Grid row | `model.rs:176` / `content.rs:64` | 108 pitch, fine | Keep; allow icon-size scaling |

---

## 4. Component richness — what's missing for an OS suite

### theme.rs (`theme.rs:43-109`)

Defines custom color roles: SELECTION, SEPARATOR, TOOLBAR, TRAFFIC_*, CLEAR, DIM, DROP, SURFACE, HOVER, PRESSED, MENU_HOVER, MENU_REST, SCROLLBAR, FOCUS (`theme.rs:10-40`). Dark + light palettes (`theme.rs:43-102`).

**Missing role tokens:**
- No **tooltip** role (background/text/border).
- No **badge** role (background/foreground for counts, status).
- No **empty-state** role (muted text/icon for "no items").
- No **skeleton/placeholder** role (shimmer or pulse color for loading states).
- No **segmented-control** role (active/segment background, border).
- No **success/warning/error** semantic colors beyond `NoticeKind` in `model.rs:51-57` (only used for notifications, not theme tokens).

### kit.rs (`kit.rs:1-23`)

Only three helpers: `pad`, `pad_h`, `pad_v` (`kit.rs:6-22`). No reusable composite views.

**Missing components (no views exist anywhere in `ui/`):**
- **Tooltip** — no `Tooltip` view; hover info (e.g., file size/date on tile) has no hover-layer mechanism beyond `HoverSurface` (`content.rs:142-146`).
- **Badge** — sidebar count is plain text (`sidebar.rs:92-96`); no pill/badge view.
- **Empty state** — when a folder is empty there is no illustration/muted "Empty folder" view; grid just draws nothing (`content.rs:64-71` with zero rows).
- **Skeleton / loading** — no shimmer view; creation/rename has no loading state.
- **Segmented control** — `view_switch` (`chrome.rs:194-236`) is hand-rolled with two `View`s and manual active bg (`chrome.rs:211`, `chrome.rs:226`); no reusable `SegmentedControl`.
- **Menu / popover primitives** — context menu exists (`menu.rs`) but no generic popover/tooltip anchor mechanism.

### Recommendation

Add to `kit.rs` (or new files `ui/kit/`): `Tooltip`, `Badge`, `EmptyState`, `Skeleton`, `SegmentedControl`. Add theme tokens in `theme.rs` for tooltip, badge, empty-state muted text, skeleton shimmer, and segmented-control active state. Then refactor `view_switch` (`chrome.rs:194-236`) onto `SegmentedControl` and the sidebar count (`sidebar.rs:92-96`) onto `Badge`.

---

## 5. Consolidated proposal priority

1. **Fix palette button label** (`chrome.rs:145`) + register `Cmd P` in `COMMANDS` (`model.rs:144-161`). Quick win, high discoverability value.
2. **In-place new-folder/rename** — biggest UX lift; replace modal as primary path with inline grid editing (`content.rs` tile + `model.rs` editing state), keep modal as fallback.
3. **Layout** — move place/path left of grow spacer (`chrome.rs`), loosen sidebar `row_gap` (`sidebar.rs:28`), make grid columns responsive (`model.rs:173`).
4. **Component kit** — add Tooltip, Badge, EmptyState, Skeleton, SegmentedControl to `kit.rs` + theme tokens in `theme.rs`; refactor `view_switch` and sidebar count onto them.

No source files were modified. This document is the plan only.