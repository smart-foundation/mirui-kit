# Acceptance, robustness and delivery gates

A spec is not proof. These gates define evidence for a usable compact showcase.
The existing audit’s [validation plan](../ux-audit/05-validation-and-roadmap.md)
remains applicable. No gate is claimed passed by the Markdown wireframes.

## Delivery order

1. **Shared vertical slice:** configurable kit install + shell/nav + Button/Field,
   command/overlay/focus services; one working S02 form with a real adapter.
2. **Real discovery:** catalog + component inspector + generated compiled recipe;
   truthful support status and source links.
3. **Core task consumers:** S03 complete collection controller and S04 actual editor;
   resolve identity/operation defects rather than cloning Files internals.
4. **Layer/feedback stress:** S05 transaction, O01–O17 lifecycle/failure matrix.
5. **Policy adaptation:** S06 scoped/app appearance and safe rollback; text/contrast
   fixes, compact layouts and high contrast/reduced motion.
6. **Evidence and host integration:** diagnostics, fresh capture manifest, performance
   budgets and real Naledzi capability adapter verification.

Ship fewer verified recipes during development; leave other catalog entries honest.
Do not label the showcase complete while its shell uses private replacement controls.

## Required deterministic fixtures

| Fixture | Why it exists | Assertions |
|---|---|---|
| Default export preset | Normal form/submit flow | Valid values, meaningful format controls |
| Invalid preset | Error/focus flow | No discarded text; clear local errors |
| Cancelled/stale preview | Async race | Old result cannot overwrite new draft |
| Small collection 0/1/5 items | Empty/final-row behavior | No sentinel/duplicate actionable cells |
| Large collection 2,048 items | Full reachability and identity | First/middle/last can be activated |
| Long/Unicode labels | Text, search, wrapping | Documented fallback/search behavior; no silent glyph loss |
| Rename/move/trash | Data trust | One identity/parent, exact counts, recoverable origin |
| Dirty note + failed save | Document protection | Dirty state/draft retained; close prevented safely |
| Two concurrent app mounts | Scoped IDs/services | No cross-instance focus/command/shadow target |
| Missing clipboard/clock/font | Capability failure | Clear fallback and still-usable app |
| Overlay events batched/unbatched | Dispatch ordering | No background activation/hover/scroll/drag |
| Appearance failed/timeout | Recovery | Exact prior policy restored |

## Every screen/state must be captured

Capture generation is future work; do not link nonexistent PNGs as current evidence.
Each record names build, fixture, app size, host size, theme, scale, density, text
scale, motion policy, focus target and renderer. Pixel output plus semantic assertions
are required for significant behavior, not pixels alone.

| Route/surface | Required capture states |
|---|---|
| S01 | First launch, resumed note, discovery results/empty, restore failure, compact |
| S02 | Default, invalid, format-dependent, pending, cancelled, failed, success, compact |
| S03 | Grid, list, empty, no results, inline edit/error, drag allowed/denied, large final item |
| S04 | Clean, dirty, Find, save pending/failure, selection with palette, compact |
| S05 | Selected changes, review, conflict, pending, applied/undo, empty selection |
| S06 | Preview draft, applied recovery, reverted, missing font, larger text, reduced motion |
| O01 | Initial/grouped, filtered, no results, unavailable command, selected result |
| O02 | Use/States/Recipe/Evidence, clipboard unavailable, wide/narrow |
| O03/O04 | Every app edge, anchor moved, overflow, disabled item, focused row |
| O05/O06/O10/O12 | Normal, long content, failure, small viewport, focused safe action |
| O07/O08 | Long error/history, no evidence, actual scenario result, cancel/failure |
| O09/O11 | Pointer/keyboard tooltip; compact nav open/current route/close |
| O13/O14 | Catalog filtered/planned/no-results; active bindings, capabilities and notices |
| O15–O17 | Appearance keep/revert; supported/unsupported preview; valid/invalid move |

Compare fresh captures to the Markdown intent, not to stale Files screenshots.
A baseline update requires a reason; a changed PNG is not automatic approval.

## Interaction gate

For each interactive overlay: pointer move/down/up, scroll wheel, drag, keyboard,
assistive actions, outside dismissal, focus loss and anchor unmount.
Test same-tick batched events as well as one-event-per-frame scripts.
Assert both the desired foreground result and unchanged background state.
After dismissal, assert the next legitimate action works; permanent blocking fails.
Every control supports its role’s keyboard actions and visible focus.
Global Ctrl P must not corrupt an editor selection or IME composition.

## Data/operation gate

No duplicate IDs, reused live IDs or wrong-target actions after sorting/filtering.
Typed metadata determines order. Invalid IDs return unavailable, never file 0.
Pending jobs carry operation/version/session identity and cancel safely.
Success is acknowledged; failure preserves recoverable drafts and explains scope.
Reset invalidates pending jobs before replacing fixtures. No cross-mount state leaks.

## Accessibility/readability gate

Normal-text role pairs meet the chosen 4.5:1 benchmark in intended states;
other graphical/focus indicators receive their own relevant review.
Measure rendered composites and verify semantics through the host accessibility path.
Typography remains readable at normal and increased text scale.
Every icon-only action has an accessible name and discoverable text guidance.
Focus, selection and hover are distinguishable without color-only cues.
Reduced motion affects live app/recipes; hidden animations do not keep repainting.
Screen-reader announcements are concise and errors are not only transient toasts.
No global “accessible” badge is earned from a token calculation alone.

## Compactness gate

1180×720 app inside 1400×900 preview keeps intended app dimensions and margins.
At 960×640 content stacks without losing action labels or status meaning.
At 800×480 compact navigation and bounded scrollable dialogs remain operable.
These are proposed support fixtures; revise the supported minimum if actual text
metrics cannot satisfy access goals. Do not shrink text to make a screenshot fit.
Inspector opening respects measured main-pane limits, not hardcoded window subtraction.

## Developer-experience gate

A second app imports shared components without FmState or copied paint/input code.
Each shipped component has a real registered recipe and examples compile.
Catalog/support/status and command enumeration derive from one source of truth.
Copy recipe uses real host clipboard or a truthful alternative.
Install/mount/unmount and two-instance tests prove resource/focus/identity isolation.
Public APIs state failure/capability behavior, no_std support and stability tier.

## Performance and robustness gate

Measure actual cold/warm startup, frame/input latency distributions, idle repaint,
scroll/filter, task progress, overlay transitions and long-session memory plateau.
Set target-specific budgets from Naledzi hardware measurements; do not invent pass
numbers in the UI. Frame cap alone is not a responsiveness test.
Keep default fixtures small; use explicitly selected stress fixtures for scale work.
No artificial endless animation on Overview or global state inspector per-frame cloning.
Validate target allocator/critical sections and resource budgets before OS claims.
Exercise host disconnect, OOM, missing assets, suspend/resume and stale IPC results.

## Evidence record template

```text
Scenario ID:
Finding/contract IDs:
Build and host adapter:
Fixture/session seed:
Viewport/theme/DPI/density/text-scale:
Input script or manual steps:
Expected foreground result:
Expected unchanged background state:
Observed state/assertions:
Screenshot/trace/measurement paths:
Result: Passed | Failed | Blocked | Not run
Known limitation / follow-up:
```

## Commands for the current repository

The future Studio binary does not exist yet; do not invent `cargo run -p showcase`.
Current commands remain in the root Makefile and UX audit. The palette icon patch
should first pass `cargo fmt --all -- --check`, `cargo test --workspace` and Clippy,
then receive a fresh palette capture and keyboard/row-completeness regression.
This environment currently has no Rust toolchain, so those gates remain blocked.
Documentation integrity and `git diff --check` are separate, narrower checks.
