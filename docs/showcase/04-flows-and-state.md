# User flows, state ownership and failure semantics

Identifiers S01–S06 and O01–O12 resolve through the [index](README.md).
All types below are conceptual contracts, not existing Rust APIs.

## F01 — Discover, try, inspect and reuse

```text
S01 header Find components
  -> catalog results (component IDs, not action strings)
  -> choose Slider
  -> S02 reveals quality specimen and labels its live capability
  -> adjust actual Slider
  -> value, label and preview-dirty state update together
  -> Inspect component
  -> O02 / Use -> States -> Recipe -> Evidence
  -> Copy recipe through host clipboard or show unsupported explanation
  -> Close inspector -> focus returns to Slider
```

The copied recipe must be generated from/versioned with the mounted public recipe.
No code example referencing a proposed-but-unimplemented API gets a working badge.
If Clipboard is unavailable, allow selection of read-only recipe text where supported.
No route or overlay is a dead end when the selected component is planned.

## F02 — Configure, validate, preview and recover

```text
S02 preset draft -> Submit
   invalid -> field errors + first-invalid focus -> correction -> Submit
   valid   -> Pending(operation id, preset version)
                  Cancel -> cancelled, retain prior successful preview
                  Edit   -> cancel old job, mark needs-preview
                  Error  -> retain draft, Retry/Details
                  Done   -> commit result only if operation+version still current
```

One Preview command is shared by button, Enter where appropriate and palette.
Submitting twice while pending does not launch duplicate jobs.
No success animation is used as a substitute for an operation result.

## F03 — Create, rename, move and restore

```text
S03 New folder -> provisional unique ID -> reveal editor -> focus/select name
  Enter valid -> commit draft -> visible object + truthful count
  Enter invalid -> keep editor -> explain correction
  Escape -> remove provisional object -> restore focus anchor

Select item -> Rename -> same editor controller
  commit -> same identity, updated name everywhere
  cancel -> old name restored

Drag stable selected identities -> allowed destination parent
  release valid -> fixture move acknowledged -> update both collections once
  release invalid/outside/Escape/focus loss -> cancel; no mutation

Trash -> recoverable fixture Trash collection -> Restore -> original parent
  original parent missing -> explicit destination decision; never lose item
```

Keyboard alternative uses Move command/dialog with equivalent target validation.
Direct manipulation does not make pointer dragging the only available path.
Selected set and active keyboard item are distinct; ordinary action labels name
one object or count clearly. Rendered cells never alias invalid IDs.

## F04 — Edit, navigate, save and close

```text
S04 clean note -> edit -> dirty(version N)
  route navigation -> draft retained; no repeated modal
  Save -> saving(N)
      edit -> dirty(N+1) while N pending
      result N -> saved N; N+1 remains dirty
      failure -> draft intact + persistent retry
  replace/reset/close -> O10
      Cancel -> original editing state
      Discard -> transition with explicit loss
      Save -> wait for success, then transition
```

Focus/caret/selection restoration is deterministic after palette, Find and inspector.
A pending save result from an old document generation cannot clear a new document.

## F05 — Commands across every route

```text
Ctrl P or Commands -> O01 focused search -> filter -> active result
  Up/Down -> active result changes, input caret stays in search
  Enter available -> accept intent -> close palette -> owning task runs
  Enter unavailable -> no execution; announce reason
  Escape -> close, restore invoker focus/selection
```

Registry descriptor includes ID, label, aliases, icon identity, scope, availability,
checked state, optional shortcut, target resolver and handler.
Header/menu/palette/keyboard are views of that registry, not separate implementations.
Commands that target selection snapshot the intended ID set at acceptance.
Remove self-open-palette results from the already-open palette context.
Search is not allowed to match an action that cannot be found as a rendered row.

## F06 — Apply appearance without losing usability

```text
S06 draft policy -> scoped specimen preview
  Apply -> validate required resources/constraints
      invalid -> no global change + explain
      valid -> temporary global policy + recovery banner
          Keep -> persist if available, otherwise session-only acknowledgement
          Revert / timeout / recovery -> exact previous policy
```

Changing global font/density/theme invalidates layout appropriately without
resetting draft data, selection or command identity. Missing glyph/font capability
never replaces the entire UI with blank text. Host changes are versioned events.

## F07 — Reset safely

```text
App menu Reset examples -> O12 affected-work summary
  Cancel -> unchanged
  Save note first -> F04 save -> return to summary
  Reset -> cancel fixture jobs -> increment session generation
           -> fresh examples -> Overview + focused heading
```

Appearance and host preferences survive example reset. Session generation guards
against late preview/save/fault responses. Reset is available without requiring
interaction with the intentionally broken specimen in a diagnostics scenario.

## Ownership model

| State owner | Owns | Must not own |
|---|---|---|
| Host adapter | Window/input/clipboard/clock/accessibility/persistence capability | Files/Studio domain rules |
| Kit services | Focus/capture/overlays/command routing/scoped identities | Fixture storage or recipe data |
| AppSession | Active route, route history, session generation, global preferences | Per-control painter behavior |
| Catalog | Stable component IDs, categories, recipes, actual support status | Guessed runtime pass/fail |
| ControlsSession | Preset draft, validation, preview job/result | Global policy unless explicitly applied |
| CollectionSession | Canonical objects/parents/trash/selection/query/view | Global root IDs or host paths |
| WorkbenchSession | Document ID/version/draft/saved version/editor state | Unrelated control fixture values |
| SurfaceSession | Change set, staged transaction, reversible result | Fake actual OS mutation |
| AppearanceSession | Draft/applied/prior policy and recovery state | Document reset or collection ownership |
| EvidenceStore | Actual scenario/build/timing records | Fabricated green completeness badges |

Use one source of truth for each value. Derived counts, labels and display metadata
are selectors, not separately mutable facts. Compare-before-write prevents idle
frames from continuously dirtying content. Subscriptions are disposed on unmount.

## Core state machines

```text
Overlay: Closed -> Opening -> Open -> Closing -> Closed
         any live state -> Cancelled on anchor/host teardown

Operation: Idle -> Validating -> Pending -> Succeeded | Failed | Cancelled
           Pending -> Committing -> Succeeded | Failed
           Cancel is accepted only before Committing

Edit: Idle -> Draft -> Invalid | PendingCommit -> Committed
                   -> Cancelled -> Idle

Component support: Planned | AdapterRequired | AvailableUnverified | Verified
                   host capability can add UnavailableOnThisHost(reason)
```

Transitions carry stable session/operation/object IDs; not label text or collection
indices. No independent booleans can make a menu, modal and incompatible palette
all logically active. A failed opening leaves focus/input ownership recoverable.

## Persistence and privacy

Default examples are session-only and visibly labelled. Persistence is optional,
capability-based and names its storage scope. No analytics/network required.
Clipboard/export/save is initiated deliberately; diagnostics never copy user drafts
by default. Evidence export redacts input content unless explicitly included.
If resource or persistence support fails, app navigation/help remains available.

## Host lifecycle

Focus loss cancels pointer capture according to interaction policy, never commits a
drop accidentally. Resize/DPI changes remeasure and re-anchor. Suspend pauses or
re-evaluates timers safely. Unmount cancels jobs and restores/removes focus scopes.
An IPC disconnect marks affected capabilities unavailable; stale responses cannot
revive dead app roots. A host-forced kill is acknowledged as outside UI prevention;
recovery depends on the declared persistence capability.
