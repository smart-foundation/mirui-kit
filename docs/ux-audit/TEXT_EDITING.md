# Text editing: screenshot audit and implementation follow-up

**Status:** shared-source implementation, not visual/runtime acceptance. Do not
call the suite finished from this report. Full Cargo compilation remains blocked:
the offline cache cannot resolve `mirx`; the earlier online attempt failed TLS.

## Evidence and UX decisions

Reviewed the actual historical `artifacts/08-dialog-new-folder.png` and
`artifacts/05-search.png`. The dialog field is a narrow strip; the search has
small type and a small hit area. These are existing screenshots, **not after
images**. No new screenshot has been produced in this follow-up.

Audited the pinned Mirui v0.51.0 sources, specifically TextEditing geometry,
selection, pointer drag, clipboard commands/acknowledgments, focus, App root
construction and frame lifecycle. Native `input_content_rect` hardcodes a 2px
inset; Style padding does not move its text or caret hit testing. Merely adding
padding to TextInput would therefore be another cosmetic non-fix.

Use a padded outer field and a measured native editor line box. Keep selection,
composition, cursor movement, hit testing and clipboard edits in Mirui. Do not
replace its renderer or manufacture selection from application strings.

Material's guidance calls for consistent field containers and persistent labels
[5](https://m3.material.io/components/text-fields/guidelines). It distinguishes
pointer selection from touch selection; pointer selection normally uses a context
menu [7](https://m3.material.io/foundations/interaction/inputs). The automatic
floating desktop action bar here is the user's explicit product requirement,
not a claim that all Apple/Google desktop editors show an identical bar. No fake
touch handles or unsupported formatting actions are added.

## Changes in source

| Finding | Implementation |
| --- | --- |
| Text-line-height dialog chrome | Shared `FieldFrame`, 40px Files fields / 44px Studio fields, 10px horizontal chrome padding plus the native inset |
| Text pinned to top left | Shared `FieldContent` measures actual font/size line height + native 4px inset; outer frame centers that line box |
| Incorrect focus outline | Rounded 3px outer ring; explicit dirtying only when focus changes, including cached/offscreen parents |
| Clicking padding does not focus | Outer Tap focuses its native editor; PointerFocus::Preserve avoids prematurely blurring inline names |
| No automatic selection actions | Topmost nonmodal layer observes the focused native TextSelection, appearing after drag release or keyboard selection |
| Selection disappears on action press | Preserve pointer focus; keep the mounted bar through its own press, rather than hide on every pointer-down |
| Copy/Cut/Paste are absent | Explicit-target native clipboard commands, plus native Select all; Ctrl shortcut labels; All selected replaces the redundant action |
| Optimistic clipboard success | Real ClipboardFinished notifications display success, permission/capacity/unsupported-host errors and CopiedWithoutCut; Cut remains acknowledgment-guarded upstream |
| Popup reopens immediately after Escape | Dismissal is remembered until selection/owner changes; Escape leaves the text selection intact |
| Stale action targets | Validate focused entity and exact selection at activation; reject hidden/disabled ancestors or missing entities; native clipboard snapshots retain their own revision guards |
| Outside-click behavior | Initial outside press dismisses without a full-screen backdrop; only the bar absorbs pointer input |
| Toolbar clipped by dialog | Append the layer after the main composition under the **same App root**, outside dialog clipping; prefer above field, flip below and clamp position to viewport |
| Background text keeps keyboard ownership | Files scopes disable inactive native editors, focus/select the new dialog or inline field and restore prior focus; Studio retains its native input gates |
| Text selection can start a file drag | Visible editing surfaces, including the action bar, latch Files press ownership until release; hidden retained fields do not block unrelated file drags |
| Search has nested/misaligned chrome | One outer frame covers icon, native editor and clear affordance; 14px body type |
| Clearing search can desynchronize state | Clear native document and filter together, preserving focus; filtering clears an invisible selected file |
| Enter does not confirm name dialogs | Native Submitted calls the same existing action as the confirmation button |
| Dialog controls too cramped | 40px dialog actions, readable body-size field/instruction text, enlarged sheets to accommodate the real controls |
| Workbench pretends a single-line input is a tall editor | Use the same correctly padded native field; full multiline editing is still a documented limitation, not silently claimed |
| Idle layout churn risk | Compare measured heights/focus before dirtying; do not clone selected document text each frame; no new host I/O or std dependency |
| Verification fixtures bypass scheduled systems | Tick before system-dependent assertions/captures; Mirui render/snapshot alone do not run application systems |

Main sources: `crates/kit/src/{field,selection,editing_policy}.rs`, kit `widgets.rs`,
Files `ui/{chrome,dialog,palette,content}.rs`, Files `sync.rs`, and Studio install/UI.

The bar uses native hover/pressed feedback, restrained DropShadow elevation and
40px action hit areas. Clipboard feedback expires after about four seconds when
the host supplies a clock; without a clock it remains dismissible. Ctrl C/X/V/A
retain native operation. Escape dismisses selection actions before the owning
Files/Studio task. No Cmd convention is introduced.

## Verification actually executed

- `sh tools/test-editing-policy.sh`: **11 passed**, then production policy compiles
  in a native `no_std` wrapper. Covers drag release, bar press survival, keyboard
  selection, collapse/removal, outside press, remembered dismissal, owner changes,
  viewport clamping and edge flipping. It does **not** simulate Mirui.
- `sh tools/test-studio-core.sh`: **33 passed**, then native no_std library compile.
- `python tools/check-mirui-syntax.py`: pinned upstream Mirui/xrune macro parsing
  and lowering pass, including new shared and changed Files sources. This is
  **not a generated-Rust typecheck**.
- Six new Files integration regressions are present for geometry/root ownership,
  padding focus, automatic selection/Copy focus preservation, dialog focus restore
  clear-search synchronization and nonmodal search during inline naming. **Not executed.**
- Full `cargo check --locked --offline -p studio`: **blocked**, missing `mirx`.
- No new render, clipboard platform round trip, pixel comparison or soak result.

## Required next acceptance checks

1. Full typecheck and all workspace integration tests on a dependency-enabled host.
2. Capture search, New Folder, Rename, palette and inline rename with/without focus
   and selection; verify every field's line box, caret baseline and clipping.
3. Exercise mouse drag, reverse selection, Ctrl A/C/X/V, IME composition, permission
   denial, unavailable clipboard, asynchronous Cut after document mutation and
   field unmount. Verify no file move or underlying button activation occurs.
4. Exercise Escape twice: first dismiss selection tools without losing highlight,
   then cancel the enclosing task; check focus restoration after every layer.
5. Fix remaining true multiline editing, precise caret/range anchoring (native
   editor geometry is private in 0.51), narrow-window toolbar reflow, persistent
   field labels/accessible semantics, focus scroll-into-view, and complete inline
   name validation/error presentation. The fixed-width bar clamps its position,
   but does not yet reflow below its own width.
6. Run the sustained interaction/render soak for the reported eventual freeze.
   Avoiding known idle churn is not proof that the freeze is resolved.
