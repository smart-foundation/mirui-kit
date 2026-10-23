# S05 — Surfaces: layering with a real review task

**Route:** `/surfaces` · **Primary task:** review and apply a safe local change set.  
**Recipe:** select sample changes, inspect a summary and confirm/cancel. The same
Dialog/Menu/Notice components used here also serve the app’s own reset/save flows.

## Default wireframe

```text
+------------------------------------------------------------------------+
| Studio      [Find components…                     ] [Commands] [Help]   |
+---------------+--------------------------------------------------------+
|   Overview    | Review changes                              [Inspect]  |
|   Controls    | Apply a reversible change to this local sample only.   |
|   Collections |                                                        |
|   Workbench   | [x] Rename “Draft” to “Review”                  [More]   |
| > Surfaces    | [x] Move sample illustration into Ready                 |
|   Foundations | [ ] Remove unused sample label                          |
|               |                                                        |
|               | 2 changes selected                        [Review…]    |
|               |                                                        |
|               | Feedback history                              [Open]   |
|               | Last action: No changes applied in this session.       |
|               |                                                        |
|               | Advanced states                              [Expand]  |
|               | Input ownership, long content, failure and cancellation|
+---------------+--------------------------------------------------------+
| Local review recipe · No OS changes                          [Status]   |
+------------------------------------------------------------------------+
```

Not a wall of “Show tooltip / Show toast / Show modal” buttons. The task gives
layers a reason to exist. Advanced states expose isolated tests when requested.
No duplicated nested fake desktop is required to test ordinary menus.

## Review O05

```text
|                    +------------------------------------------+|
|  background inert  | Review 2 local changes                    ||
|                    | Rename Draft → Review                    ||
|                    | Move illustration → Ready                ||
|                    | You can undo this change set.            ||
|                    |              [Cancel] [Apply 2 changes]   ||
|                    +------------------------------------------+|
```

Only offer Undo when the transaction has a recorded inverse and can be restored.
Confirm carries target/version context; stale target produces O06, not a silent
application to new objects. Review defaults focus to the safe contextual action;
destructive variants focus Cancel first. Escape cancels; outside taps do not
silently accept or discard a pending critical decision.

## More/contextual menu

The selected change has a real menu with leading icons: Inspect, Exclude, Details.
Disabled actions have a reason. Pointer and keyboard open it on the same target.
The menu and review dialog share OverlayHost. Opening Review closes the menu
atomically; dismissing it cannot click the checkbox behind it.

## Pending, complete and undo

```text
| Applying 2 local changes…      [====      ]             [Cancel] |
| [notice] 2 local changes applied.              [Undo] [Dismiss] |
```

Cancellation succeeds only before commit. At the commit point, explain finalizing
instead of showing a Cancel button that cannot do anything. The operation result
owns completion. Duplicate Apply dispatches are prevented by an operation state,
not only by fading a button.
Notification history keeps the outcome even if an ephemeral success disappears.

## Advanced states drawer

Expose named scenarios, not arbitrary mutation knobs:
- Long summary and translated labels.
- Conflict from an updated target version.
- Retryable local fixture failure.
- Nested confirmation invoked from a popover.
- Pointer capture interrupted by focus loss.
- Keyboard-only confirmation and focus restoration.

Each scenario has Start, current expected behavior and Reset scenario.
Diagnostic controls live outside the specimen’s focus trap, but opening a test
cannot leave the app without a safe way to stop/reset it.
An explicit Return to baseline remains available through host/app recovery.

## Surface hierarchy

Base canvas < local panel < menu/popover < modal < system-critical host prompt.
Passive notices do not sit above and obscure the modal action footer.
Shadows are shared elevation roles and never receive input.
Child tooltips cannot escape to cover another higher-priority modal.
Focus/wheel/hover follow the same owner, not independent per-layer guesses.

## Error / empty states

No selected changes: Review is disabled with “Select at least one change”.
All changes already applied: show a completed state with Restore sample, not a
fake empty panel. Failed transaction leaves the sample unchanged and shows retry.
Partial application is not supported in this fixture recipe; any future partial
operation must display per-item results and truthful recovery options.

## Compact layout

Review uses min/max width and internal content scrolling, keeping actions visible.
Long summaries wrap. Dialog height fits within safe app viewport insets and larger
text. Controls do not move under the pointer when validation text appears.

## Acceptance

- Menu, dialog, notice and the app’s own reset/save overlays are shared components.
- Transaction applies once, cancels safely and rolls back truthfully where offered.
- Background is inert for pointer, keyboard, wheel and assistive actions.
- Focus returns to the invoker or a sensible surviving action after completion.
- Review can be completed in a compact viewport without hidden footer actions.
