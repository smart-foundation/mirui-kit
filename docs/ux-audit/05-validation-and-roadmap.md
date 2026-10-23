# Validation plan, user journeys and implementation roadmap

This is a proposed execution plan for [the findings](01-code-defects.md),
[visual/accessibility audit](02-visual-accessibility.md) and
[architecture audit](03-architecture-developer-experience.md).
No test in this document is marked passed without an execution record.

## 1. Current verification boundary

The sandbox has no `cargo`, `rustc` or installed Mirui source tree.
This audit uses source inspection, saved images and small independent Python
calculations. Those calculations establish arithmetic/structural facts, not
successful Rust compilation or current renderer behavior.

The preceding turn introduced input-isolation changes and tests. They remain
unexecuted and are included in scope, not assumed correct.
No new runtime fixes are part of this documentation-only audit turn.
A passing `git diff --check` establishes patch whitespace hygiene only.

## 2. Existing tests: useful evidence, insufficient claims

| Existing test area | What the code actually exercises | Important gap |
|---|---|---|
| `tests/data.rs` | Static table and selected state transitions; new ownership/cancellation assertions | No exhaustive identity/collection invariant suite |
| `tests/features.rs` | Many methods called directly plus some pointer scripts | Model success does not prove actual control/command behavior |
| Right-click feature test | Simulates pointer then manually calls menu-opening state method | Does not execute SDL right-button filtering or pairing |
| Slider/sort feature test | Calls `set_icon_size` and `set_sort` directly | Cannot catch no-op `Slider::ValueChanged` |
| Trash/restore feature tests | Method results and selected totals | No full Trash navigation/origin/created-item recovery journey |
| Drop-on-folder test | Asserts the item stays in the same Place | Encodes fixture behavior, not actual folder containment |
| View/count tests | Assert list_count/shown against total | Does not prove all rows exist or can be reached |
| `tests/interaction.rs` | Pointer tap/theme/navigation/basic selection and recent backdrop regressions | No broad keyboard, wheel, focus-loss or DPI suite |
| `tests/appearance.rs` | One tile pixel changes; edge pixel suggests a scrollbar | Neither establishes consistent hover states nor draggable thumb behavior |
| `tests/typeface.rs` | Assets decode, selected glyph coverage/raster kind, initial layout | Does not certify text smoothness, all-script shaping or fractional-DPI policy |
| `targets/snapshot` | Renders named fixture states to PNG | No automatic expected-image comparison or complete state matrix |
| `examples/bench`, `perf`, `leak` | Print timings, damage reports or entity counts | No bounded-memory, latency or freeze regression threshold |

Do not delete useful tests just because they are incomplete.
Correct tests that require the wrong behavior, then add the missing journey.
Use stable semantic locators; geometric helpers based on 104×98 tiles or
26/28px row heights break when density/accessibility work changes layout.

## 3. Domain invariant tests before visual refactoring

Use generated sequences where practical, with a deterministic seed.
Assertions apply after every public transition, not only the final state.

| Test | Invariant | Findings |
|---|---|---|
| D01 | Each existing object has one unique stable ID | F04/F37 |
| D02 | A collection contains an ID at most once | F01/F03 |
| D03 | A normal object has exactly one authoritative parent | F03/F13 |
| D04 | Trash membership and original parent apply equally to all kinds | F02 |
| D05 | Removing a provisional object removes stale references | F04/F39 |
| D06 | Duplicate preserves kind and obtains independent identity | F05 |
| D07 | Sort compares typed data with stable tie-breaking | F06 |
| D08 | Hidden/search filters produce truthful derived counts | F07/F09 |
| D09 | Invalid lookups cannot alias a real item | F11/F37 |
| D10 | Every declared available command is discoverable/renderable | F20/A04 |
| D11 | Successful move updates source/destination/metadata together | F01/F03/F36 |
| D12 | Cancellation never executes an operation or success notification | F17/F24 |
| D13 | Every state transition preserves valid overlay combinations | F22/F23/A05 |
| D14 | Selected/edited/dragged targets are existing valid identities | F10/F39 |
| D15 | Date fixtures format known days correctly | F41 |

Include adversarial sequences: create A/B/C, edit A, cancel A, create D;
move a created item twice, trash it, restore it; sort while a menu is open;
filter a selected object out, then request Rename; receive an old async result
following a newer navigation. Do not rely solely on happy-path screenshots.

## 4. Input-routing matrix

Test each of: menu, options popover, dialog and command palette.
Also test passive notifications, tooltip and drag preview as nonmodal surfaces.
For each interactive overlay run these cases:

1. Move pointer over panel, padding and gaps; background does not hover.
2. Tap a control once; it activates exactly once.
3. Tap panel padding; nothing behind it activates and panel does not accidentally close.
4. Tap outside; only documented dismissal occurs.
5. Hold down outside, dismiss, move onto a file, release; no file drag/activation.
6. Start slider drag, leave panel bounds, release; slider retains/cancels correctly.
7. Wheel over popup content, blank panel and backdrop; no unauthorized background scroll.
8. Open another overlay while dragging; pending drop cancels, not commits.
9. Press Escape; top dismissible layer closes and focus restores.
10. Lose window focus or unmount anchor; all captures cancel safely.
11. Replay down/move/up batched in one tick and separately over ticks.
12. Test mouse, touch and additional pointer IDs if supported by the release.
13. Open near edges, resize, change DPI and scroll the anchor.
14. Use keyboard traversal and assistive activation instead of pointer input.

Inspect resulting domain state and dispatch targets, not only cursor pictures.
Assert the next genuine click works after dismissal; blocking forever is also a bug.
Use event traces to verify system ordering instead of depending on frame timing.

## 5. End-to-end user journeys

### J01 — First launch and discoverability

Start at supported minimum size in both themes.
Identify current location, available navigation, primary content and search scope.
No fake window controls or fixed storage facts may appear as live capabilities.
Tab through the initial screen and verify visible focus and accessible labels.
Resize with inspector open; every necessary action stays reachable.

### J02 — Browse every item

Load 0, 1, 5, 32, 48, 512, 2,048 and 10,000 entries.
Reach first/middle/last by pointer and keyboard in both views.
Change sort/search while preserving stable object identity and sensible focus.
No duplicate, sentinel or permanently omitted object is actionable.
Record retained entity count and scroll latency alongside the journey.

### J03 — Search and recover

Type ordinary, no-match and Unicode queries; clear search by an obvious action.
Distinguish loading from no matches, empty directory and permission failure.
Search results/counts and selected-target policy agree.
A renamed matching item updates immediately; a nonmatching item leaves coherently.
Test slow/out-of-order search results if the domain adapter is asynchronous.

### J04 — New folder, commit and cancel

Invoke via toolbar, context menu and palette in grid and list.
Reveal a provisional object and focus/select its editable name.
Try valid, empty, duplicate, invalid, long and non-ASCII names.
Enter commits once; Escape cancels; blur follows the documented policy.
Cancel an older draft and create another; IDs remain unique.
Host rejection leaves a recoverable draft, not a false success.

### J05 — Rename existing object

Rename without changing sort position and while changing it substantially.
Verify grid/list labels, inspector and Info update together.
Selection, menu target and drag target remain the same object.
Changing selection while a rename dialog is open cannot rename the new selection.
Undo/cancel/failure semantics are documented and tested.

### J06 — Drag and organization

Drag different kinds successively; preview matches the lifted source.
Drop on valid/invalid folders, blank content, self, descendant and another place.
Cross-volume policy is explicit; accepted and denied feedback differ clearly.
Scroll near collection edges while dragging if auto-scroll is supported.
Focus loss, Escape, source removal and host failure cannot commit a stale move.
Immediate counts and source/destination collections match acknowledged results.

### J07 — Trash and recovery

Trash built-in and created items; navigate to Trash and verify original paths.
Restore one item and handle an origin that is missing or read-only.
Put Back is unavailable outside recoverable Trash state.
If permanent deletion is offered, require truthful destructive labeling and
appropriate confirmation; do not simulate deletion by dropping an ID silently.

### J08 — Settings and inspector

Click checkbox indicator, label and padding: each toggles once.
Drag slider and adjust by keyboard: icon, number, thumb and model agree.
Switch sort; order reflects numeric metadata rather than formatted text.
Every inspector tab is named, selectable and shows meaningful content.
No selection yields an intentional empty inspector, not file 0’s icon.

### J09 — Operation errors and notifications

Inject permission denied, offline, full disk, conflict and cancellation.
Critical errors remain readable and offer retry/details where meaningful.
Success is not announced before acknowledgement.
Repeated notifications do not discard the only explanation for data failure.
No card blocks keyboard focus indefinitely or dismisses through an unrelated action.

### J10 — Document/Settings reuse trial

Build an unrelated settings screen and a minimal document workflow from public APIs.
No FmState dependency, copied checkbox painter or manual focus workaround is allowed.
Change global font/density/theme once; both apps adapt.
Measure developer steps and duplicated code, not just runtime performance.

## 6. Accessibility and visual state matrix

Required states: normal, hover, pressed, focused, selected, selected+focused,
disabled, invalid, loading, pending, success and error where applicable.
Test active/inactive window and high-contrast policy.
Test dark/light, compact/comfortable, normal/larger text and reduced motion.
Test 1x, 1.25x, 1.5x, 1.75x and 2x output scale where host support is declared.

Start viewport fixtures with 800×480, 1180×720 and 1400×900 preview host.
800×480 is an existing bare example size, not proof it is a supported production
minimum. Set and document the actual product minimum after layout review.
The larger preview fixture must preserve its explicitly chosen inner app size.

Measure contrast on intended role pairs and rendered composites.
Check label/name/value semantics through the Naledzi accessibility adapter.
Verify keyboard order, group arrows, activation, focus trap/restoration and cancellation.
Test long filenames, long translation strings, combining marks, CJK and RTL strings
according to the declared support tier; record unsupported cases honestly.

Snapshots must have stable fixtures, theme/scale/text settings and state generation.
Store fresh artifacts with commit/source metadata and review them side by side.
A new baseline is not an automatic approval; document why a difference is correct.
Mask only genuine nondeterminism and keep semantic assertions alongside pixels.

## 7. Performance and Naledzi robustness gates

Measure independently:

- Static executable/assets, font representations and resource decode/cache.
- Framebuffer storage, renderer scratch, UI heap and retained entity count.
- Cold startup, first frame, warm startup and first usable input.
- Idle work, damage area, hover, selection and text input cost.
- Scroll, sort/filter, view/theme switch and overlay transitions.
- P50/P95/P99 frame and input latency under documented target hardware/workload.
- Memory plateau during a sustained operation/navigation/overlay soak.
- Focus loss, suspend/resume, display resize/DPI change and low-memory behavior.

Choose numeric budgets from measured Naledzi target hardware, not invented numbers
in a Markdown file. Record the hardware, build profile, fixture size, duration and
clock. A 60Hz cap does not guarantee a frame finishes within its time budget.

For bare/Naledzi targets, validate allocator alignment/failure accounting and the
real interrupt/multicore critical-section contract before treating rendering as safe.
Include IPC disconnect, malformed/stale response, permission revocation, operation
cancellation, missing resource and allocation failure. No silent panic-spin counts
as graceful failure. Run a real interactive port, not only the compile-only gate.

## 8. Ordered implementation milestones and exit evidence

### M0 — Establish the executable baseline

Owner boundary: host/tooling.
Provision the pinned toolchain and SDL dependencies in a reproducible environment.
Run existing tests, record failures without accepting stale “done” reports,
capture current states and reproduce the reported freeze with event/timing traces.
**Exit:** repeatable build/test/snapshot commands and a bounded freeze repro or
explicitly recorded inability to reproduce, not a claim that it is gone.

### M1 — Repair domain identity and operation integrity

Owner boundary: application/domain model.
Address F01–F07, F13, F36–F39 and F41; stop invalid identity fallback and false success.
Define one authoritative fixture store before an OS filesystem adapter.
**Exit:** invariant and generated-sequence tests; exact counts and safe targets.

### M2 — Repair interaction ownership and complete basic controls

Owner boundary: input/focus/overlay services and control adapters.
Address F10/F11/F14/F15/F20–F25/F28/F40/F42 and A04/A05/A14.
Verify the prior latch patch under actual dispatch ordering.
**Exit:** routing matrix, keyboard basics, real slider/scrollbar and registry completeness.

### M3 — Establish reusable appearance and layout

Owner boundary: foundation/controls/shell.
Implement semantic type/color/metrics/elevation roles and whole-app preview layout.
Correct V02/V03 contrast; fix V07–V09 sizing/anchoring with real measured bounds.
**Exit:** reviewed gallery, theme/density/text-scale matrix and current screenshots.

### M4 — Finish coherent app journeys

Owner boundary: Files composition/controllers.
Complete real activation, reveal/inline editing in both views, selection policy,
full result reachability, meaningful inspector/options and recoverable feedback.
**Exit:** J01–J09 executed without fixture-only shortcuts masquerading as controls.

### M5 — Extract and validate the reusable kit

Owner boundary: public kit API and docs.
Follow the [blueprint](04-timeless-kit-blueprint.md); migrate Files to be a consumer.
Build Settings and a small Document app to prove domain independence.
**Exit:** two-app reuse trial, scoped identities, configurable install, examples and gates.

### M6 — Integrate Naledzi and harden delivery

Owner boundary: platform/domain adapters and target runtime.
Integrate actual Naledzi window/input/clock/accessibility and filesystem IPC.
Replace unsafe smoke-target runtime assumptions; measure memory and scheduling.
**Exit:** target-specific failure tests, performance/soak record, documented support matrix
and no unresolved identity/input/memory-safety release blockers.

Do not implement these milestones as unrelated agents editing the same model and
shell concurrently. Use contract boundaries, integration ownership and acceptance
reviews. Reports alone do not close a milestone.

## 9. Build/test commands to establish, not claimed results

```sh
cargo fmt --all -- --check
cargo test --workspace
cargo test -p fm --no-default-features
cargo test -p fm --all-features
cargo clippy --workspace --all-targets -- -D warnings
(cd targets/bare && cargo fmt -- --check)
(cd targets/bare && cargo build --target riscv64gc-unknown-none-elf)
cargo run --release -p snapshot
cargo run --release -p desktop
cargo run --release -p fm --example bench
cargo run --release -p fm --example perf
cargo run --release -p fm --example leak
```

Feature unification must be inspected; a workspace build with desktop/std enabled
is not an independent no_std proof. Keep the bare workspace separate and add a
library-level no_std gate appropriate to supported targets.

## 10. Definition of done for any finding

Record finding ID, patch scope, reproducible scenario and tests added.
Record execution environment, exact command and observed result.
Attach fresh images for appearance/layout changes and traces for routing fixes.
Explain compatibility, performance and accessibility effects.
Update API documentation and remove stale claims.
If evidence is blocked, mark the finding patched/unverified, not closed.
