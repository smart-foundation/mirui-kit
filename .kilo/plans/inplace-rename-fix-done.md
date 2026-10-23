# In-place folder rename — done

## Scope
Implemented direct-manipulation folder creation/rename in the file manager (`crates/fm` on mirui 0.51.0). Creating a folder now appends an empty placeholder tile to the grid and opens an inline name editor (macOS/Windows style) instead of opening the modal sheet. The modal sheet is retained as a fallback (still reachable via the command palette).

## Changes

### `crates/fm/src/model.rs`
- `FmState` gains two fields:
  - `#[observe] pub editing_folder_id: Option<u32>` — the folder (a `CreatedFolder` placeholder) whose name is being edited inline.
  - `pub edit_document: model!(owned TextDocument)` — backing document for the inline editor.
- Constructor initialises `edit_document` and `editing_folder_id: None`.
- Accessors `edit_text()` / `set_edit_text()` (mirror `folder_text()`/`set_folder_text()`).
- New methods:
  - `new_folder_inline()` — appends an empty `CreatedFolder` in the current place, bumps `names_revision`, rebuilds the listing, then starts editing it.
  - `start_inline_folder_rename(folder_id)` — sets `editing_folder_id` and pre-fills the editor with the folder's current name.
  - `commit_inline_rename()` — names the folder (via `rename`), or removes an empty uncommitted placeholder; clears editing state.
  - `cancel_inline_rename()` — removes a never-committed placeholder (or reverts), clears editing state.

### `crates/fm/src/ui/chrome.rs`
- `new_folder_button` tap handler now calls `fm.new_folder_inline()` instead of `set_folder_text(""); fm.open_dialog(DialogKind::NewFolder)`. Removed the now-unused `DialogKind` import.

### `crates/fm/src/ui/content.rs`
- `cell` renders a focused `TextInput` (bound to `fm.edit_document()`, with `Placeholder`, `FieldFocus` ring) when `fm.editing_folder_id() == Some(id)`, otherwise the static `Text` label.
- The inline editor commits on `TextEditing::Submitted` (Enter) and on focus loss (`FocusChanged` with `!event.focused`); an empty name removes the placeholder. `TextEditing::Changed` keeps `edit_text` in sync.

## Verification
- `cargo build -p fm` — clean, no warnings.
- `cargo test -p fm --test features` — 19 passed, 1 failed. The single failure (`a_context_menu_row_can_trash_a_file`) is pre-existing in the working tree: it fails identically with my three edited files reverted, so it is unrelated to this change.
