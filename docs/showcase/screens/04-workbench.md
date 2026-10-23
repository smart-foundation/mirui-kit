# S04 — Workbench: an editing app made from the kit

**Route:** `/workbench` · **Primary task:** edit and save a local example note.  
**Scope:** plain text, not a fabricated full word processor. Persistence capability
is explicit: session-only by default; real local storage only through a tested host adapter.

## Default wireframe

```text
+------------------------------------------------------------------------+
| Studio      [Find components…                     ] [Commands] [Help]   |
+---------------+--------------------------------------------------------+
|   Overview    | Interface notes                         [Save] [More]  |
|   Controls    | Local example · Saved in this session                   |
|   Collections |--------------------------------------------------------|
| > Workbench   | [Find in note…                      ] [Prev] [Next] [X]|
|   Surfaces    |                                                        |
|   Foundations | Build interfaces people can trust.                     |
|               |                                                        |
|               | A control should explain what it does and preserve     |
|               | the user’s work when something goes wrong.             |
|               |                                                        |
|               |             real multiline editor                      |
|               |                                                        |
|               |                                                        |
|               | Line 3, column 12 · Plain text             [Inspect]   |
+---------------+--------------------------------------------------------+
| Session-only save · Not written to disk                      [Status]   |
+------------------------------------------------------------------------+
```

Find row appears only when invoked; it is not permanent toolbar clutter.
Save action explicitly saves a session snapshot unless real persistence is enabled.
Use “Save session copy” in the action description/help. Never show a disk path that
does not exist. Clipboard actions are disabled with a host reason if unsupported.

## Dirty and saving states

```text
| Interface notes · Unsaved changes                  [Save] [More]|
| Edits are kept in this session until you close Studio.          |
```

Save records draft version N under an operation ID. While saving, show Saving…,
allow further edits and track the newer dirty version. Completion for N does not
clear edits made at N+1. Failed save keeps the draft and offers Retry/Details.
Do not block the editor with a modal spinner for a routine save.

```text
| [error] Could not save the session copy. [Retry] [Details]        |
| Your text is still here.                                       |
```

A fixture-only injected save error is labelled as a scenario; it is not an actual
OS filesystem failure. Failure notification must not be the only persistent evidence.

## Editor contract

The real multiline editor supplies selection, caret, wrapping and scrolling.
Copy/cut/paste, undo/redo, IME composition and keyboard selection are supported only
when verified through the host/editor path. Unsupported capabilities are exposed
truthfully in the inspector. A single-line TextInput stretched tall is not TextArea.

Use clear plain-text semantics and word wrapping; no syntax-color rainbow or fake
format toolbar. Tabs/newlines and editor-local keybindings cannot trigger global
commands unexpectedly. Selection remains visible with a defined inactive style.
Font/text scale changes preserve content, caret position and readable line lengths.

## Find behavior

Ctrl F opens the route-local Find row and focuses its field. Enter goes next;
Shift Enter previous; Escape closes Find and restores editor focus/selection.
“No matches” is explicit, not a disabled pair of unexplained arrows.
Find searches the current note; header discovery still searches the catalog.
Ctrl P opens the global palette without destroying note selection.
Bindings are a proposed target contract until the registry/editor routing exists.

## Unsaved-work transition O10

Leaving the route preserves its draft and does not ask repeatedly.
Replacing/closing the note, resetting examples or closing the app requests a
save/discard decision if there is dirty work. Save failure keeps the app open.
Save, Discard and Cancel labels name their effect; generic OK is prohibited.
See the full dialog wireframe in the overlay specification.

## Component inspection

Inspect opens O02 for the Editor recipe, with text selection/scroll preserved.
Recipe exposes the actual mounted public binding and supported host features.
Do not copy the editor document into a parallel inspector model that silently drifts.
Changing a specimen-only state must not disable Studio’s only keyboard path.

## Compact layout

The editor gets the full main width. Find actions may wrap to a second line but
never cover the caret. Route toolbar retains Save and More; metadata becomes
shorter or wraps. Inspector is an overlay with a focus scope, not a pane that
shrinks the editor to an unusable column.

## Acceptance

- One real editor implementation, not separate “demo” and “actual” text controls.
- Save/dirty/error state follows document version, not elapsed animation time.
- A failed save cannot close the note or clear dirty state.
- Find, palette and editor shortcuts route by scope.
- Draft survives route navigation; destructive transitions use O10/O12.
- Session-only behavior is visible before the first Save click.
