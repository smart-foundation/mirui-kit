# Overlay and transient-state wireframe atlas

These are **target-state Markdown wireframes**, not current renderer output.
All surfaces use one shared OverlayHost, scoped IDs, focus controller and command
registry. Shared rules apply to every route and Studio’s own chrome.

## Global ownership rules

One top modal owns keyboard, pointer, hover, wheel and assistive actions.
Menu/popover is transient; opening a modal closes incompatible transient layers.
Outside tap dismisses a transient only; it never activates content behind it.
A confirmation/unsaved decision does not dismiss on outside tap.
Escape closes the top cancellable surface; it never applies an operation.
An operation’s non-cancellable commit phase is explicitly represented.
Focus restores to the invoker if present, otherwise a surviving meaningful target.
Hidden surfaces have no shadows, semantics or input regions.
All panels clamp/flip against the app viewport, not the outer preview stage.

## O01 — Command palette

```text
+---------------------------------------------------------------+
| [search] Type a command…                                  [X] |
| Commands for Collections · shortcuts follow active bindings    |
|---------------------------------------------------------------|
| > [new folder] New folder                             Ctrl …   |
|   [rename]     Rename selected item                   F2*      |
|                Name the selected sample asset                  |
|   [grid]       Switch to list view                             |
|---------------------------------------------------------------|
| Navigation                                                    |
|   [settings]   Go to Controls                                  |
|   [document]   Go to Workbench                                 |
|---------------------------------------------------------------|
| ↑↓ Navigate · Enter Run · Escape Close                         |
+---------------------------------------------------------------+
```

`F2*`/`Ctrl …` illustrate optional binding slots, not claimed implemented shortcuts.
Production labels come from actual bindings only. Ctrl P is the proposed app-wide
open-palette binding. Do not show a row’s shortcut until it really dispatches.
Leading icon + action label are required, with optional concise description.
The selected result is visibly and semantically active; icon is decorative.
Up/Down/Home/End navigate, Enter executes, Escape restores focus.
Search remains focused while the active result is tracked separately.

Results derive from registered available commands, grouped by useful scope;
no hardcoded index list or hidden final command. Match title, alias and category
with a documented Unicode policy. Keep stable command ID while filtering.
Do not list “Open command palette” inside the already open palette as useful work;
exclude self-actions in this scope while retaining the registry entry elsewhere.
Maximum panel height fits viewport; short result sets do not leave a giant empty box.

```text
| [search] no_such_action____________________________             |
| No commands match “no_such_action”.                             |
| [Clear search]  Try a component or route name.                  |
```

Disabled command may appear if discoverability is useful, with a reason line:
“Rename — Select one sample item first”. Enter cannot execute it.
Async execution closes palette only after accepting the intent; failure then
belongs to the owning task, with visible recovery and no silent focus loss.

## O02 — Component inspector / recipe drawer

```text
+---------------------------------------------+
| [field] Field                       [Close] |
| Live instance · Controls / preset-name      |
| [Use] [States] [Recipe] [Evidence]           |
|---------------------------------------------|
| Labelled single-line text input.            |
| Use for a short editable value.             |
| Value: Review images                        |
| Focus: false · Validation: valid            |
|                                             |
| Public composition used by this instance    |
| [read-only code, scrollable]                 |
| [Copy recipe] [Open related example]         |
+---------------------------------------------+
```

Default wide mode can be nonmodal only if it fits minimum main-pane constraints;
otherwise use a modal drawer with explicit focus scope. The same action is never
sometimes click-through because width changed. Width mode/semantics are announced.
Inspect mode is explicitly activated, never steals ordinary specimen interaction.
Evidence lists actual build/scenario result or Not verified. Copy uses real clipboard.
Recipe references shared kit APIs; no Files-private FmState shortcuts.

## O03 — Context menu

```text
+--------------------------------------+
| [open]      Open                      |
| [rename]    Rename                    |
| [duplicate] Duplicate                 |
|--------------------------------------|
| [folder]    Move…                     |
| [trash]     Move to Trash             |
+--------------------------------------+
```

Target identity and selected set are snapshotted at invocation and validated at
execution. Target removal closes/disables the menu with a clear explanation.
Arrow navigation, type-ahead if supported, Enter, Escape and context-menu key route
through the same descriptor as pointer activation. Separators are non-focusable.
Empty-space menu offers location actions, not an arbitrary previously selected item.
Every executed row closes or transitions the menu according to one declared policy.

## O04 — Anchored options popover

```text
                      [View options]
                 +----------------------------+
                 | View options               |
                 | Density [Compact|Comfort]  |
                 | [x] Show item details       |
                 | Sort [Name v] [Ascending]   |
                 | [Restore view defaults]    |
                 +----------------------------+
```

Anchor to actual invoker rect, flip/shift at edges, recompute on resize/scroll.
Changes apply to this route’s view settings, not Studio’s global density unless
explicitly labelled. Whole label toggles checkbox once. For Studio, options is a transient focus-scoped popover; Tab cycles its controls,
Escape/outside dismisses and restores focus, and background pointer input is blocked.

## O05 — Review / confirmation dialog

```text
+----------------------------------------------------+
| Review 2 local changes                              |
| Rename Draft to Review                              |
| Move Aurora into Ready                              |
| These changes affect the local example only.        |
|                           [Cancel] [Apply 2 changes]|
+----------------------------------------------------+
```

Heading, description and primary action name the operation and target count.
Long summaries scroll internally; footer stays reachable. Destructive mode uses
“Move to Trash” or the actual operation, not generic OK. No outside dismissal.
Dialog owns target/version context and does not consult arbitrary later selection.

## O06 — Conflict / validation failure

```text
+----------------------------------------------------+
| Could not rename “Draft”                            |
| An item named “Review” already exists here.          |
| Name [Review 2____________________________]         |
|                         [Cancel] [Rename]           |
+----------------------------------------------------+
```

Preserve draft, show error adjacent to affected field, focus sensible correction.
A missing target instead offers Close/Refresh; it does not substitute another ID.
Host retry is idempotent under an operation ID. No infinite spinner or lost text.

## O07 — Notification history

```text
+---------------------------------------------+
| Activity                            [Close] |
| [check] Preset preview ready                |
|         Local simulation · 3 samples        |
| [warning] Save did not complete             |
|         Draft is still available. [Retry]   |
| [info] Example changes applied      [Undo]  |
+---------------------------------------------+
```

Ephemeral success may expire; actionable errors remain until resolved/dismissed.
History carries timestamps/order only if a clock is available and meaningful.
An operation result is not lost because three newer cosmetic notices arrive.
Whole-card click does not silently discard a critical message; actions are explicit.
History is local session data unless persistence is intentionally implemented.

## O08 — Diagnostics / fault scenario drawer

```text
+---------------------------------------------+
| Scenario runner                     [Close] |
| Build: <actual build id>                    |
| Scenario [Overlay dismissal v]              |
| Expected: background remains unchanged      |
| [Run] [Stop] [Restore baseline]              |
| Last run: Not run                           |
| Trace [read-only events and state changes]  |
| Host capabilities [View]                    |
+---------------------------------------------+
```

No fabricated Passed status or guessed frame/memory numbers. Diagnostic jobs are
cancellable, scoped and bounded. A manual scenario reports observed assertions,
not an overall accessibility certificate. Fault injection touches fixture adapters
only; it must not drop real host input or corrupt preferences.

## O09 — Tooltip

```text
        [Inspect]
       +-------------------------------+
       | Inspect component · shortcut  |
       +-------------------------------+
```

Delayed hover and immediate/appropriate keyboard reveal according to shared policy.
Anchor without obscuring the control. Essential instructions remain available
elsewhere. Passive tooltip does not steal focus, pointer capture or wheel input.
Escape dismisses it without closing an unrelated modal underneath.

## O10 — Unsaved changes

```text
+----------------------------------------------------+
| Save changes to “Interface notes”?                  |
| Your latest edits are not in the saved session copy.|
| [Discard changes]             [Cancel] [Save copy]  |
+----------------------------------------------------+
```

Cancel leaves draft and route intact. Save waits for acknowledged completion;
failure leaves the dialog/draft recoverable. Discard affects this document only.
Closing the host window waits for this decision when host lifecycle permits it.
Forced host termination cannot be prevented; recovery capability is documented.

## O11 — Compact navigation drawer

```text
+-------------------------------+
| Studio navigation     [Close] |
| [home] Overview               |
| [settings] Controls           |
| [grid] Collections            |
| [document] Workbench          |
| [layers] Surfaces             |
| [appearance] Foundations      |
+-------------------------------+
```

Opened by labelled Menu action, not an unexplained icon-only rail.
Selecting route closes drawer and reveals heading. Escape/outside closes without
navigation. Layout change to persistent navigation preserves active route and
places focus on a surviving target, never a hidden drawer entity.

## O12 — Reset examples

```text
+----------------------------------------------------+
| Reset local examples?                              |
| This clears sample assets, presets and local drafts.|
| Appearance preferences will stay unchanged.         |
| Unsaved note: Interface notes                      |
| [Save note first]           [Cancel] [Reset examples]|
+----------------------------------------------------+
```

Scope is explicit and distinct from Restore appearance defaults or Reset preset.
Save note first enters the document save flow and returns with reset still pending;
failed save cannot trigger reset. Reset uses a new fixture session generation,
cancels outstanding jobs and restores focus to Overview. Late old results are ignored.

## Shared long-content and failure geometry

Maximum overlay size is app viewport minus safe insets, accounting for text scale.
Use internal scrolling for content; primary/cancel actions remain reachable.
No overlay relies on an exact 420×168 shape or fixed English copy height.
Position/size changes do not make a hovered action jump beneath the pointer.
Nested overlays preserve an explicit parent stack; arbitrary booleans are prohibited.

## O13 — Full component catalog / discovery browser

```text
+---------------------------------------------------------------+
| Components                                           [Close]  |
| [Find components…___________________________________________] |
| [All] [Input] [Navigation] [Collections] [Feedback] [Layout]     |
|---------------------------------------------------------------|
| [field]  Field             Labelled editable value    Available |
| [slider] Slider            Numeric range              Adapter  |
| [tree]   Tree              Hierarchical navigation    Planned  |
|                                                               |
| Selected: Slider                                              |
| Range input with keyboard and pointer capture requirements.    |
| [Open live recipe] [Inspect contract]                          |
+---------------------------------------------------------------+
```

Opened by View all, category chip or discovery Browse action; not a seventh route.
Results derive from the catalog. Filter/reset/no-results behavior matches discovery.
Open live recipe is unavailable for planned items with a reason; Inspect contract
still works and clearly distinguishes specification from executable code.
Keyboard list navigation and category selection preserve stable active identity.
Closing restores the invoker; opening a recipe closes the browser and reveals it.

## O14 — Help, keyboard guide and About

```text
+---------------------------------------------------------------+
| Studio help                                          [Close]  |
| [Getting started] [Keyboard] [Capabilities] [About]             |
|---------------------------------------------------------------|
| Try a recipe, inspect a component, then reuse its public API.   |
| [Start with Controls] [Browse components]                      |
|                                                               |
| Keyboard: actual active bindings, grouped by scope             |
| Capabilities: real host/editor/clipboard/save support          |
| About: actual kit version, build ID and third-party notices    |
+---------------------------------------------------------------+
```

This is a read-only help drawer with real shared Tabs/links/scroll semantics.
Default Getting started is short; no automatic blocking tour. Keyboard entries
come from the registry, not a second static shortcut table. Capability statements
include unsupported reasons. About contains real license/attribution notices and
build metadata, not a stale claimed version. Offline local docs remain usable.

## O15 — Appearance recovery banner

```text
+---------------------------------------------------------------+
| Appearance changed. Keep it?   Reverting in 15s. [Revert] [Keep] |
+---------------------------------------------------------------+
```

App-wide high-priority recovery region; it must not be clipped under the current
appearance or hidden behind an ordinary specimen inspector. Keep/Revert have
keyboard/assistive actions and announced remaining-time changes without spamming.
If no reliable timer exists, use explicit confirmation and guaranteed reset path
as documented in S06; no fictitious countdown. Host forced failure uses last valid
policy at next initialization if persistence/recovery support exists.

## O16 — Sample asset preview

```text
+---------------------------------------------------------------+
| [image] Aurora                                       [Close]  |
| Local sample · Image · fixture metadata                        |
|---------------------------------------------------------------|
|                                                               |
|                 real supported sample content                 |
|                                                               |
|---------------------------------------------------------------|
| [Inspect component]                     [Open in Workbench*]   |
+---------------------------------------------------------------+
```

Only show supported content; an icon alone is not an image preview. Unsupported
content gets a named state and metadata, not a broken panel. Workbench action is
available only for compatible text samples; image previews do not offer fake editing.
Opening text in Workbench validates dirty-work transitions through O10. No real host
file/application launch is implied by an in-memory sample action.

## O17 — Move destination chooser

```text
+---------------------------------------------------------------+
| Move “Aurora”                                                  |
| Destination: Samples /                                        |
| [folder] Illustrations                                        |
| [folder] Ready                                                |
| [folder] Drafts             [unavailable: source descendant]   |
|                                                               |
| Selected destination: Ready                                   |
| [Cancel]                                      [Move here]     |
+---------------------------------------------------------------+
```

Keyboard alternative to dragging uses the same validated parent-ID move intent.
Navigate real fixture hierarchy with breadcrumbs/Back and accessible list; a Tree
is optional and not fabricated before its shared implementation exists.
Self/descendant/read-only/missing destinations are unavailable with reasons.
Selection and committed destination are distinct; Move here is enabled only for
a valid different parent. Late domain changes revalidate before commitment.
