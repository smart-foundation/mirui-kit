# Current component catalog and OS-suite readiness

Audit date: 2026-10-24. Inspected the working source on
the active working branch, not just the screenshots or showcase specification.

Updated after the research/implementation pass: see [the complete baseline and
new-source evidence](COMPONENT_COMPLETION.md). The original eight catalog groups
are retained below as a baseline; the live catalog now has **19 grouped entries**, still not 19 accepted families.

## Verdict

**No: the current shared kit is not a complete foundation for an OS app suite.**
Mirui provides primitives from which more applications can be implemented, but
app authors would currently need to implement fundamental components themselves.
A toolkit that requires every app to build its own tree, forms, selection model,
button groups, tables and overlay lifecycle is not yet delivering that foundation.

The original Studio catalog had **eight grouped entries**, not eight independently
complete, behaviorally accepted components. Several entries combine multiple
concepts; two point at the same document/editing infrastructure, and others are
services or assets rather than controls. Six showcase routes and 17 surface IDs
are not evidence of a complete component library.

No complete-widget count or percentage is asserted here: there is no agreed
finite universe of all OS apps, and source presence is not runtime acceptance.

## Original eight-entry baseline

Source: `crates/studio/src/catalog.rs::COMPONENTS`.

| Existing entry | Actual implementation | Important limit |
| --- | --- | --- |
| Action / button / icon | `widgets::action` | One full-width icon-plus-label button composition, not three independent Button/IconButton/Action components; no public intent, disabled, pending or group configuration in this helper |
| Document field / placeholder | `widgets::field` | Native single-line document field with shared chrome, focus and submission; not a complete labelled/validated form-field family |
| Numeric slider / keyboard steps | `widgets::number` | Slider plus decrement/increment buttons, fixed range 0–100 and step 5; **not a numeric text input** |
| Boolean switch | `widgets::toggle` | Native Switch with label and some keyboard/focus behavior; whole-label interaction and accessible semantics still need acceptance |
| Draggable scrollbar | `scrollbar::ScrollBar` | Shared scrollbar source exists; not a complete generic collection or keyboard-scroll contract |
| Native text document editor | `widgets::Editor` | Document/appearance attachment wrapper using the same native editing foundation as the field; not a separate multiline/code editor |
| Input ownership / focus scope | `InputGate`, `FocusItem`, keyboard service | Binary background/overlay gating and ordered Tab traversal; not a general nested overlay/focus-scope stack |
| Semantic appearance / typography | `theme`, `font`, assets/features | Reusable assets and size helpers; not a complete component state/shape/elevation/accessibility system |

The new live entries add ChoiceGroup, Hierarchy/TreeView, tri-state Checkbox,
NumericEntry, labelled field/validation presentation, feedback and DataTable.
Their policies and several native input/layout contracts are now tested; full
workspace/no_std/SDL checks passed through `887dac3`. Views remain partial;
component-by-component runtime and accessibility acceptance is still required. See `COMPONENT_COMPLETION.md` for exact remaining limits.

Follow-up shared source also includes
`FieldFrame`, `FieldContent` and `selection` services. Their presence does not
close the larger gaps below. These additions are also awaiting full application
compilation, integration execution and fresh visual acceptance.

## Classification: do not mix these four layers

- **Shared partial:** public source in `crates/kit`; some reuse exists, but the
  required component contract or verification remains incomplete.
- **App-local:** implemented in Files/Studio, not exposed as a generic kit
  component. Some helpers are generic; others depend directly on FmState/Studio.
  Neither category should be advertised as a complete shared component.
- **Native primitive only:** Mirui offers building blocks, but the kit does not
  yet provide the higher-level component contract.
- **Missing shared component:** no corresponding generic implementation found in
  the inspected kit. This does not mean Mirui is incapable of supporting it.

Pinned upstream inventory was retrieved directly from
[Mirui v0.51.0 widgets/mod.rs](https://github.com/W-Mai/mirui/blob/v0.51.0/src/ui/widgets/mod.rs).
It includes Button, Checkbox, Slider, Switch, Text, TextInput/TextEditing,
TextDocument, TabBar/TabContent, LazyList, ProgressBar, Icon, Image and rendering
/effect primitives. That module does **not** expose a TreeView or ButtonGroup.
Low-level layout containers and gestures are not substitutes for their behavior.

## Required inventory, compared with current source

### Actions and selection controls

| Component | Current status | Missing fundamental contract |
| --- | --- | --- |
| Button | Shared partial: `widgets::action` | Intent/variant/size, icon optionality, disabled/pending, consistent activation, complete semantics |
| IconButton | Native Icon + ad hoc tappable Views | Label, tooltip association, target sizing, keyboard/disabled behavior in one public component |
| **ButtonGroup** | Shared partial: `controls::ChoiceGroup`, Actions mode | Joined-edge variants, intent/pending configuration, native semantics and runtime acceptance |
| ToggleButton / ToggleButtonGroup | Shared partial: `ChoiceGroup`, Single/Multiple modes | Standalone ToggleButton, pressed semantics and runtime acceptance |
| SegmentedControl | Shared partial `choice_group`; Files still has local selector | Finish component-specific geometry, semantics and migrate Files consumer |
| Radio / RadioGroup | Shared partial `ChoiceGroup` single mode | Dedicated radio indicator, accessible group/choice labels, native semantics and runtime acceptance |
| Checkbox | Shared partial: `controls::{Check,checkbox}` | Tested mixed state; disabled focus/semantics, validation association and native runtime acceptance pending |
| Switch | Shared partial: `widgets::toggle` | Complete labelled interaction and state/semantics contract |
| MenuButton / SplitButton | Missing shared components | Primary versus disclosure action, menu ownership, availability and keyboard routing |
| Link | Native text/action building blocks only | Link semantics, keyboard activation, host navigation policy and disabled/visited treatment where applicable |

A Row containing buttons is not a ButtonGroup. A mutually exclusive settings
choice is not the same thing as a group of independent commands. The catalog must
separate ButtonGroup, ToggleButtonGroup, RadioGroup and SegmentedControl rather
than treat them as interchangeable skins.

### Text entry and forms

| Component | Current status | Missing fundamental contract |
| --- | --- | --- |
| Label / Text roles | Native Text + shared size helpers | Consistent role/weight/wrap/ellipsis, association with controls, selectable/copyable display variants |
| TextField | Shared partial: `widgets::field` | Persistent label, required/disabled/read-only state, prefix/suffix, helper/error, validation lifecycle |
| SearchField | App-local Files search | Generic clear, scope, result state and keyboard focus API |
| **TextArea / multiline editor** | **Missing complete shared component** | Multiline layout, caret/selection, scroll, IME, clipboard, undo/redo; current Editor is not this |
| PasswordField | Missing shared component | Secure display/reveal policy, clipboard policy, labelled state and accessibility/host implications |
| NumberField / SpinBox | Shared partial: `forms::{NumericEntry,number_field}` | Integer draft/commit and bounds/step exist; decimals/units/localization, semantics and runtime acceptance pending |
| Select / Dropdown | Missing shared component | Option descriptors, selection, keyboard navigation and popup ownership |
| ComboBox / Autocomplete | Missing shared component | Input versus selected value, filtering, loading/empty/error results, acceptance/cancellation |
| Form / FormGroup / Field validation | Partial `form_field`, `Validation`, tested `FieldState` policy | Full form/group lifecycle, async errors, dependencies, summary, focus links and native label association pending |
| Date / Time / Calendar picker | Missing shared components | Locale/time-zone policy, keyboard entry, validation and selection views |
| Color picker | Missing shared component | Typed value, formats, keyboard alternatives, commit/cancel and contrast-aware preview |

Native editing selection actions are shared source, not proof of a full form or
editor framework. Undo/redo, read-only display selection and host IME/accessibility
acceptance cannot be inferred from a Copy button existing.

### Collections and hierarchy

| Component | Current status | Missing fundamental contract |
| --- | --- | --- |
| List / ListView | Native LazyList; app-local capped plain Files list | Generic items, identity, complete reachability, virtualization, focus, selection and row actions |
| Grid / GridView | App-local Files/Studio collections | Measured tracks, complete item access, responsive reflow, selection and virtualization |
| SelectionModel | Shared tested `behavior::collection::Selection` | Integrate consistently into all consumers; native pointer modifier/reveal and event acceptance pending |
| **TreeView** | **Shared partial: `controls::{Hierarchy,tree_view}`** | Virtualization, reveal, pointer modifiers, DnD, host semantics, typeahead event wiring and runtime acceptance pending |
| TreeGrid / outline table | Missing shared component | Tree behavior plus typed columns, sorting and aligned/virtualized rows |
| DataTable | Shared partial: `table::{DataTable,data_table}` | Header keyboard navigation/sort indicators, resize gestures, full grid navigation/editing, virtualization, semantics and runtime acceptance pending |
| Drag/drop controller | Native gestures + Files-specific tracking/model | Explicit capture, payloads, allowed operations, targets, cancellation, keyboard alternative and host bridge |
| Reorderable list | Missing shared component | Stable moves, insertion markers, keyboard reorder, undo and invalid-target behavior |

A flat Places sidebar is not a tree. A Session model containing parent IDs is not
a TreeView. A file list arranged as several Text widgets is not a DataTable.

### Navigation and spatial layout

| Component | Current status | Missing fundamental contract |
| --- | --- | --- |
| Tabs / TabPages | Native primitives + app-local inspector use | Named tab descriptors, keyboard selection, disabled state, focus, content lifetime and overflow |
| Sidebar / NavigationList | App-local Files/Studio | Generic destinations, selection, collapse/restore, badges, focus and responsive behavior |
| Breadcrumbs / location entry | Missing shared component | Stable ancestors, navigation, truncation/overflow, editable location and errors |
| Toolbar | App-local composition | Logical groups, overflow, command availability, focus and ownership of the affected view |
| MenuBar | Missing shared component | Top-level menus, keyboard traversal, command state, submenus and host policy |
| **SplitPane / resizable panes** | **Missing shared component** | Constraints, pointer capture, keyboard resizing, collapse/restore, state and focus handling |
| Disclosure / Accordion | Missing shared component | Expand/collapse, heading semantics, focus and child lifetime |
| AppShell / window frame | Separate app-local shells | Host versus app chrome, panes, overlay root, measured bounds, title/status ownership and constraints |
| StatusBar | App-local Files | Typed truthful status, selection/task summaries and host capability handling |

Raw Row/Column/Scroll containers remain useful Mirui primitives. They do not
supply the higher-level behavior and constraints in this table.

### Transient surfaces, feedback and assets

| Component | Current status | Missing fundamental contract |
| --- | --- | --- |
| Menu / ContextMenu / MenuItem | App-local Files/Studio | Generic command descriptors, availability, separators/submenus, keyboard, stable targets and focus restoration |
| Popover | App-local/hardcoded or sheet presentation | Generic trigger anchor, collision handling, nesting, dismissal and input ownership |
| Dialog / Alert / Confirm | App-local sheets | Typed intent/actions, stable targets, focus lifecycle, validation, default/cancel behavior and responsive layout |
| Tooltip | App-local passive bubble; Studio surface | Actual hover/focus delay, passive hit behavior, anchoring, dismissal and accessible association |
| CommandPalette | App-local Files/Studio | Shared registry/query/availability API, context, result navigation and execution/restore lifecycle |
| Toast / Notification / banner | App-local Files/Studio | Shared priorities, actionable messages, expiry policy, announcements and host notification boundaries |
| Badge / Counter | Shared partial: `feedback::badge` | Passive semantics, live counts, width/overflow and aligned numeric track |
| Progress / task feedback | Shared partial `feedback::progress`; tested Task policy | Wire task/cancel/retry lifecycle into consumers, live semantics and native visual acceptance |
| Empty / Loading / Error state | Shared partial `feedback::{empty_state,alert}` | Recovery actions, async integration, skeleton/spinner, reduced motion and semantics |
| File/Open/Save picker | Missing shared component | Real location/selection/filter state, validation, overwrite and explicit filesystem capability boundary |
| Image / Icon / Avatar / thumbnail presentation | Native Image/Icon + shared filled icon assets | Semantic labels, sizing/fallback/loading policy and outline action-icon family; a graphics primitive is not the whole family |
| Surface / Shape / Elevation roles | Local styles + native effects | Consistent shape, border/focus/clip/shadow contracts and accepted render output |

Catalog entries must link real shared APIs, and distinguish partial source from
accepted components. A generic Menu remains missing; the new group/tree/table
APIs exist in source but are not runtime accepted.

## Cross-cutting foundations also incomplete

These are necessary beyond the visible component list:

- General nested overlay ownership and focus scopes, not one background/modal bit.
- Roving focus, focus-visible policy, focus scroll-into-view and removal recovery.
- Accessible roles/names/states/actions and a host accessibility bridge.
- A shared command model with enabled/checked/pending states and shortcut routing.
- Async resource/task state, cancellation and stale-result protection available
  to all consumers, not only one Studio workflow.
- Typed host capabilities for filesystem, clipboard, drag/drop, window actions,
  notifications, storage, media and process/system data.
- A complete semantic style/state system, outline action assets, typography,
  reduced-motion behavior, responsive density and localization/RTL contracts.
- Lifecycle and performance acceptance for large data, render resources and idle
  frames. Bounded demo item caps are not a replacement for scalable collections.

Preserve `no_std + alloc`: component/domain crates must not import host I/O.
Adapt host services through explicit capabilities and request/result boundaries.
Rendering/controls reuse Mirui; the missing layer is not a license to replace it.

## Can representative OS apps be built mostly from this kit today?

| App archetype | Current gap that forces fundamental app-specific implementation |
| --- | --- |
| Settings / preferences | Generic forms, checkbox/radio/groups, select/number fields, sections, navigation and validation |
| File explorer / picker | Tree, complete virtualized collections, multiselection, table headers, split panes, pickers and filesystem bridge |
| Document editor | Real multiline editing, undo/redo, document commands, menus and open/save lifecycle |
| Terminal | Terminal text/selection/scroll behavior, monospace presentation, keyboard routing and PTY/process bridge; not just a TextField |
| System monitor / task manager | Data table, sorting/selection, charts/time series, async metrics and task/process capabilities |
| Installer / software center | Typed async task/progress/error/retry, forms, lists and actual package/download services |
| Calendar / scheduler | Calendar/date/time controls, localization, validation and time-zone/domain behavior |
| Media / image utility | Suitable preview/transport/selection controls plus explicit decoder/playback/storage capabilities |

**Result: not yet.** This is about reusable-kit coverage, not a claim that Mirui
cannot render such apps or that every specialized engine belongs in the UI kit.

## What the two specifically requested components must include

### ButtonGroup

- Action descriptors with stable IDs; joined versus separated layout; horizontal
  and vertical orientation; consistent sizing and overflow.
- Shared edges and matching outside corner geometry, hover/pressed/focus states.
- Per-action disabled/pending behavior without accidental double activation.
- An explicit Tab/arrow focus policy appropriate to group semantics.
- Separate selected-value contracts for ToggleButtonGroup/RadioGroup, rather than
  treating every group as mutually exclusive.
- Labelled group semantics, visible keyboard focus, RTL and text-scale acceptance.

### TreeView

- Stable node IDs, parent/child relationships and independent expansion/selection.
- Visible-row flattening/virtualization; collapsed descendants are neither rendered
  as active rows nor focusable; selected/focused-node recovery on removal/filter.
- Up/Down/Home/End, Right to expand/enter children, Left to collapse/go to parent,
  type-ahead and explicit Enter/Space behavior.
- Single/multiple selection policy; disabled nodes and lazy loading/error/retry.
- Tree/treeitem semantics, level/expanded/selected/disabled state and usable
  disclosure targets, not indentation alone.
- Pointer and keyboard behavior tests, large/deep hierarchy tests, cancellation
  of stale lazy-child results and truthful host/domain adapters.

## Correct delivery plan and completion gate

The existing blueprint already listed Tree, Table, SplitPane and group controls
as requirements. **A requirement written in an audit is not an implementation.**
The last visual audit did not close these gaps and must not imply suite readiness.

1. Establish shared command/state/selection, focus/overlay and semantics contracts.
2. Complete ordinary action/form families, explicitly including ButtonGroup,
   ToggleButtonGroup, RadioGroup, Checkbox, typed entry and validation.
3. Implement TreeView, virtualized List/Grid, DataTable, Tabs, SplitPane and
   Disclosure using those same contracts. Do not bind them to Files state.
4. Build generic menus/popovers/dialogs/tooltips/palette/notifications and app
   navigation/shell contracts; finish document editing and host-capability seams.
5. Exercise them through at least Settings and Explorer/picker consumers using
   public APIs, without copied implementations inside each app. Extend coverage
   for editor, monitor and other archetypes; specialized engines stay explicit.

This is sequencing, not a reduction of the requested suite scope.

A component is not accepted because a name, screenshot or demo route exists. Its
catalog record must name its actual public API/source, implemented states, input
and semantics behavior, consumers, tests, known limits and verification status.
Acceptance requires full native and no_std builds, real pointer/keyboard/lifecycle
regressions, fresh theme/scale captures and applicable performance/accessibility
checks. Existing domain/policy tests and macro lowering do not establish widget
runtime correctness. Full widget verification remains outstanding.

Until then the honest product description is **a partial shared kit plus two
application/showcase implementations**, not an OS-suite-ready component library.
