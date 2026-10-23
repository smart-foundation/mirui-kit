# Deeper critique: prevent a polished showcase from teaching bad UX

This is an adversarial review of the proposed Studio and the current prototype.
It extends the [76-finding audit](../ux-audit/README.md); it does not claim those
findings are now fixed. The palette patch addresses two narrow presentation/
enumeration issues. The rest requires implementation and execution evidence.

## 1. Icons are necessary here, but not sufficient

Current source: [`ui/palette.rs`](../../crates/fm/src/ui/palette.rs).
The old row was two text widgets; the patch adds a semantic leading icon and
registry-sized row generation. Good. But the palette still lacks a selected-result
controller, Enter/arrow handling, focus restoration, useful grouping, no-results
surface and context availability. A beautiful row can still be an unusable command UI.

**Studio rule:** icon + label + state + availability + navigation + execution are
one row contract. Test first/last/filtered/disabled results through real input.
A semantic icon match is exhaustive, independent of translated label text.
Do not treat an icon as an accessibility name or show icons without readable labels.

## 2. Dynamic actions must describe the outcome

Current source: [`model::COMMANDS`](../../crates/fm/src/model.rs) uses “Toggle…”
labels. An icon representing a grid is ambiguous if the resulting view is List.
Likewise a moon icon alone cannot tell whether dark mode is active or available.

**Studio rule:** show “Switch to list view”, “Show inspector” or a checked state
that names the setting. Resolve label/icon/checked state from one descriptor and
current context. Never let visual state and command handler disagree after updates.

## 3. Two search fields can create two wrong mental models

Current Files already mixes palette/search in a dense toolbar.
Studio needs catalog discovery plus local collection/note searches, but they must
not all say Search or share an unclear result scope.

**Studio rule:** “Find components”, “Search sample assets”, “Find in note” and
“Type a command” are distinct scopes. Clear labels, matching empty states and
keyboard focus make scope visible. Catalog results navigate; command results act.
Test a novice choosing the correct field without reading the specification.

## 4. Inspect mode must not break the specimen

A developer gallery often grabs every click to open its inspector, making it
impossible to actually use a field, slider or drag interaction.

**Studio rule:** inspecting is an explicit action/mode with clear exit. Ordinary
clicks operate real controls. Keyboard and assistive users can inspect the focused
component without pointer-only overlays. Inspector state cannot steal a pending
edit or reroute the next control activation.

## 5. “All states” must not sabotage the app’s own controls

Disabling the shared Button specimen must not disable the navigation/Cancel that
lets a user leave it. Theme or focus experiments can otherwise trap the user.

**Studio rule:** state variants are scoped to specimen instances. Global policy
changes use a recoverable transaction and a guaranteed reset path. Test the
“disabled”, “invalid”, “no focus target” and “missing font” scenarios explicitly.
Dogfooding means shared implementation, not one unsafe global boolean for all instances.

## 6. Specimen status is not a quality certificate

A tag “Ready” based only on a function existing is misleading. Current helpers
include static tooltip/skeleton APIs without a complete installation/trigger path.

**Studio rule:** distinguish Planned, Adapter required, Available but unverified,
Verified for this build/host, and Unavailable on this host. Evidence is scoped to
actual conditions. No green “flawless” score, “100% accessible” or universal Passed.
A renderer snapshot does not prove keyboard behavior or OS robustness.

## 7. Short lists should not produce empty monuments

Current palette nested Columns use fixed content heights independent of filtered
result count. A single result floats in a large mostly empty panel in saved images.
The new length-based unfiltered extent is not a complete filtered-layout fix.

**Studio rule:** grow to content up to a viewport-bound max, then scroll. Derive
count/height/active result from the same filtered descriptors. Zero results has a
useful explanation and Clear action. Keep result row positions stable during typing.

## 8. Context and selection must remain visible

Current menu actions sometimes consult live selection at confirmation; file list
labels and captured IDs can diverge under retention. A generic “Rename” is unsafe
when the user cannot tell which object the action targets.

**Studio rule:** target identities are snapshotted and validated. Dialogs name the
object/count. Selection, active keyboard item and context target have explicit rules.
Filtering or removing a target cannot quietly redirect an action to file 0 or another
row. Test sort/filter while a command surface is open.

## 9. “Compact” is not a license for unreadable controls

Current essential labels/inputs commonly use 11px. Source token calculations show
contrast problems even when the font file is correct.

**Studio rule:** reduce chrome and content duplication first, not body size.
Use meaningful 14px body/action starting roles, measured metrics and usable hit
areas. At increased text scale, stack layout and scroll instead of clipping.
Measure real readability; Medium does not repair an insufficient contrast pair.

## 10. Creative showcase means meaningful tasks, not novelty navigation

An animated carousel of controls, draggable floating windows or surprise mode
switches might look impressive but makes comparison and keyboard use difficult.

**Studio rule:** six stable routes; tasks reveal combinations naturally. Creativity
comes from the export, asset-organization and editing recipes, not invented controls
for established actions. Each recipe has one clear next step and a reversible result.
Do not make the app require a tutorial just to find the components.

## 11. Error demonstrations must be real but safe

A timed red toast is not a failed save. A button that pretends to delete a file
teaches false confidence. Unbounded fault injection could corrupt actual host work.

**Studio rule:** deterministic fixture adapters return real failure/cancel results
to the same controllers that production adapters use. The mode is labelled Local
simulation. Save failure preserves a versioned draft; transaction failure preserves
the prior sample state. Faults never mutate host filesystem or appearance secretly.

## 12. Documentation cannot drift from the running recipe

A Recipe tab with a pretty invented API makes the app easy to admire but hard to
build with. Current README already contains stale virtualized-list/token claims.

**Studio rule:** compile public recipe examples and associate them with mounted
instances and kit version. Runtime props/state and code come from the same recipe
metadata. Unsupported/pseudocode examples are explicitly labelled, not copy-ready.
The app proves ease of use by building Settings/Document consumers without FmState.

## 13. Component breadth must not hide incomplete behaviors

A count-only TabBar is not working tabs. A 20px tappable square is not a reusable
IconButton. A bubble is not a tooltip and a six-pixel rail is not a complete scrollbar.

**Studio rule:** component coverage enumerates behavior, state, keyboard, semantics,
installation, failure and lifecycle. Add fewer complete components before a huge
catalog. Planned components have no deceptive live controls.

## 14. Passive information should not promise activation

Current badges and inspector detail rows carry HitTarget/press feedback without
a useful action. This increases hit-test ambiguity and teaches a false affordance.

**Studio rule:** status/counts/metadata are passive unless an explicit Copy/Details
action exists. Hover styling is not added indiscriminately to make everything feel
responsive. Audit the focus/hit map, not just the list of visual controls.

## 15. A preview must not secretly change the app

A theme specimen that changes global policy without warning confuses users;
a responsive preview that shrinks a screenshot is not responsive UI.

**Studio rule:** scoped preview versus Apply to Studio is visibly distinct. Whole-app
host preview preserves an actual inner app viewport. Token changes remeasure real
components and preserve their models, not replace them with static pictures.
Prove default, compact, larger-text and inspector-open layouts with fresh output.

## 16. “Undo” is a domain promise

A notice offering Undo after a fixture move is only legitimate if original parent,
name/version and conflicting state are known. A generic global restore can erase
unrelated subsequent work.

**Studio rule:** inverse operation is tied to a completed transaction and validated
before execution. Expired/conflicting undo explains why; it never overwrites newer
work. Do not offer Undo for operations the adapter cannot reverse.

## 17. Progress needs an actual task model

A static 62% bar or arbitrary spinner does not communicate real progress.
Simulated local processing shown as real export/storage data damages trust.

**Studio rule:** use determinate progress only for known work units; otherwise
label indeterminate work. Pending actions have cancellation/commit boundaries.
No time-based animation alone creates success. Real measurements and simulation
estimates are clearly distinguished in labels and evidence.

## 18. Nested layers need a policy, not more z-indices

Menu, popover, palette and dialog booleans can coexist invisibly and reappear later.
Current open_palette clears menu but not menu_target; adding icons changes none of this.

**Studio rule:** one typed overlay stack with parent/invoker/capture/focus state.
Modal opening closes incompatible transient layers atomically. Hidden layers have
no hit regions or shadows. Test hover/wheel/keyboard, not just click-through.

## 19. Reuse has to survive two instances

Global IDs such as toolbar_search and inline_rename cannot safely identify two
simultaneous app mounts without scoping. Global shortcut flags lose window ownership.

**Studio rule:** scoped component identities, per-window event queues, mount-owned
subscriptions and explicit service installation. Run two instances with independent
forms, palettes, scrollbars and themes. Closing one cannot break the other.

## 20. Completion must be evidence-led

The environment still lacks Rust tools, and Studio has not been implemented.
Markdown completeness, a semantic icon match and clean Git whitespace are not a
proof of correct layouts, control behavior, target safety or Naledzi readiness.

**Studio rule:** each shipped screen/state has code, a reproducible fixture, semantic
assertions, fresh rendered output and host-specific interaction results. Keep
unverified items visible as unverified. “Timeless” is maintained by these contracts
and regression discipline, not by declaring a design perfect once.
