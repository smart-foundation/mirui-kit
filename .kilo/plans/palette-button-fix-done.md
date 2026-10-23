# Palette button fix (done)

## Changes

### crates/fm/src/ui/chrome.rs
- Changed the palette button label from `"Ctrl P"` to `"Cmd P"` (chrome.rs:145), matching the Cmd-based shortcuts used throughout COMMANDS.

### crates/fm/src/model.rs
- Added `OpenPalette` variant to the `Command` enum (model.rs:124).
- Registered `("Open Command Palette", "Cmd P", Command::OpenPalette)` at the end of `COMMANDS` (model.rs:160), bumping the array length from 16 to 17. Appending preserves existing command indices so the palette filter tests stay valid.
- Added `Command::OpenPalette => self.open_palette()` arm to `run_command` (model.rs:669).

## Verification
- `cargo build -p fm` — succeeds.
- `cargo test -p fm --test features` — 20 passed, 0 failed.
