# Complete application-kit baseline: research, scope and acceptance

2026-10-24 · Mirui 0.51 · no_std + alloc · implementation work in progress.

**This is the required delivery scope, not a list of shipped components.**
Completeness must cover behavior and shared services as well as visible widgets.
The user should not have to request each standard control individually.

## Online research

The references were inspected before this implementation pass. They inform
contracts; they are not dependencies and do not justify replacing Mirui.

| Primary reference | Implication for this kit |
| --- | --- |
| [W3C APG patterns](https://www.w3.org/WAI/ARIA/apg/patterns/) | Cover the full interaction-pattern inventory, not just a few attractive specimens. Tables and interactive grids are different contracts. |
| [W3C keyboard interface](https://www.w3.org/WAI/ARIA/apg/practices/keyboard-interface/) | Tab moves between composites; arrows operate inside them. Focus and selection must be independent. |
| [W3C radio groups](https://www.w3.org/WAI/ARIA/apg/patterns/radio/) | Standalone radio arrows select; toolbar radio arrows move without changing selection. Required and initially unselected groups need deliberate policies. |
| [W3C TreeView](https://www.w3.org/WAI/ARIA/apg/patterns/treeview/) | Stable hierarchy, expansion, parent/child arrows, visible traversal, selection and typeahead; indentation alone is insufficient. |
| [W3C combobox](https://www.w3.org/WAI/ARIA/apg/patterns/combobox/) | Editable and select-only variants need distinct commit/cancel behavior and popup ownership. |
| [W3C modal dialog](https://www.w3.org/WAI/ARIA/apg/patterns/dialog-modal/) | Constrain focus, prevent underlying interaction, and restore focus appropriately. |
| [W3C window splitter](https://www.w3.org/WAI/ARIA/apg/patterns/windowsplitter/) | Pointer resizing alone is insufficient: keyboard adjustment, bounds, naming and collapse/restore matter. |
| [React Aria packages](https://github.com/adobe/react-spectrum/tree/main/packages/@react-aria) | Reuse collections, selection, focus, overlays and form behavior across visual components. |
| [React Aria](https://react-aria.adobe.com/) | Separate style from interaction; include validation, accessible drag/drop and column resizing in complex-component acceptance. |
| [Fluent component inventory](https://fluent2.microsoft.design/components/web/react) | Include feedback, navigation, identity, forms and transient surfaces alongside collections. |
| [Fluent color](https://fluent2.microsoft.design/color) | Semantic interaction-state roles, not unrelated per-component color literals. |
| [Fluent toolbar](https://fluent2.microsoft.design/components/web/react/core/toolbar/usage) | Logical action groups, overflow and contextual grouping are part of a toolbar. |
| [Qt widget gallery](https://doc.qt.io/qt-6/gallery.html) | Desktop breadth includes calendar/date/time, editable combo, groups, tabs, tables and progress. |
| [Qt windows/dialogs](https://doc.qt.io/qt-6/application-windows.html) | A desktop app needs a shared shell, menus, toolbars, docking/status and standard dialogs, not only content controls. |
| [Qt model/view](https://doc.qt.io/qt-6/model-view-programming.html) | Share selection/header management across list, tree and table views. |
| [shadcn inventory](https://ui.shadcn.com/docs/components) | Cross-check omissions such as input groups, fieldsets, resizable panes, empty states, skeletons, command surfaces and notifications. |

APG specifies web accessibility semantics. Mirui is not a DOM: corresponding
native semantics need a real host accessibility bridge. Adding labels visually
or copying ARIA terminology does not establish assistive-technology support.

## Required inventory — no user enumeration needed

All items below are required unless explicitly marked as a specialized extension.
Grouping related items here does not allow counting them as separately complete.

| Family | Required reusable APIs and behavior |
| --- | --- |
| Action controls | Button (primary/secondary/quiet/destructive), IconButton with accessible name, Link, ToggleButton, ButtonGroup, ToggleButtonGroup (single/multiple), Radio/RadioGroup, SegmentedControl, MenuButton, SplitButton, repeat/hold policy where appropriate |
| Boolean/continuous input | Checkbox including mixed state, Switch, Slider, RangeSlider with non-crossing thumbs, NumberField/SpinBox with editable draft, bounds and step, Meter |
| Text input | TextField, SearchField/clear, PasswordField/reveal, TextArea, read-only selectable text, input prefix/suffix/actions, copy/cut/paste/select-all, undo/redo, IME/composition, length limits and validation, focus/selection preservation |
| Choice/pickers | Select, editable ComboBox, Autocomplete, ListBox single/multiple, option groups, disabled and asynchronous options, token/tag input, calendar/range calendar, DatePicker/DateField, TimeField, DateTimeField, ColorPicker/SwatchPicker, font picker |
| Forms | Label, description/error association, Field/FieldGroup/Fieldset/Legend, required/optional, touched/submitted validation, synchronous and async errors, validation summary with focus links, submit/reset/pending, read-only and disabled propagation |
| Collections | List, Grid, DataTable, TreeView, TreeGrid, column headers and cell renderers/editors, stable IDs, independent focus/selection, range/multiselection, filtering/sorting, column resize/reorder/hide, pagination, virtual windows and reveal-focused-row, empty/loading/error states |
| Direct manipulation | Drag source/drop target, typed payloads, move/copy feedback, valid/invalid targets, autoscroll, reorder indicators, cancellation, keyboard equivalents and no text/slider/file gesture interference |
| Navigation | Tabs/TabPanel (manual/automatic activation), closable/reorderable document tabs, Breadcrumbs/location entry, Sidebar/Nav groups, NavigationRail, back/forward history, pagination, stepper/wizard, disclosure/accordion and expanded-state persistence |
| Layout | Stack/Row/Column/Grid wrappers with semantic gaps, Container/Inset, Card/Panel, Separator, ScrollArea (both axes), SplitPane with pointer/keyboard sizing, collapsible panes, resizable inspector, docking/tabbed panes, responsive overflow and minimum-size behavior |
| App shell | AppShell, titlebar/window buttons with host commands, MenuBar, grouped Toolbar/overflow, search placement, Sidebar, content-local StatusBar, document dirty/close guards and command/shortcut routing |
| Surfaces | Menu/Submenu, ContextMenu, MenuItem/check/radio items and separators, Popover/anchoring, Tooltip, HoverCard, Dialog, AlertDialog, Sheet/Drawer, CommandPalette, nested ownership, dismissal rules and focus restoration |
| Feedback | Inline Alert/MessageBar, Toast/Notification center with actions and pauseable expiry, determinate/indeterminate Progress, Spinner, Skeleton, Badge/Counter, EmptyState, ErrorState/retry, job queue/cancel/retry and live announcements |
| Content | Text roles with wrapping/ellipsis/selection, Icon, Image with loading/failure and aspect ratio, Avatar/AvatarGroup, Tag/Chip/dismissible tokens, key hints, data description list, tooltips for truncated content |
| Standard workflows | File/folder/open/save picker, overwrite/delete confirmation, preferences form, find/replace bar, shortcut editor, About/help, multi-step progress and recovery; use typed host capabilities rather than inventing successful I/O |
| Complex editing | Shared multiline document foundation, selections/caret/IME, undo transactions, search/replace, code gutter and syntax-renderer hooks, rich-text renderer/editor hooks, large-document virtualization. Language servers, terminal emulation and document codecs are specialized engines behind these hooks, not fake kit widgets. |
| Visualization/extensibility | Canvas/custom paint, hit-testing and coordinate transforms, pan/zoom, accessible data alternatives, chart axes/legend/tooltip/series primitives. Domain-specific CAD, video timelines and chart packages are extensions, but extension/focus/theme contracts are core. |

This baseline is broader than the earlier eight catalog groups and includes
components the user did not name. "Any complexity" requires extensibility and
well-defined host interfaces; no finite catalog can embed every business domain.
That is not an excuse to omit the standard families above.

## Shared services required underneath

1. **Theme and metrics:** semantic color/typography/spacing/shape/stroke/elevation,
   density, disabled/read-only/invalid/pending/selected/hover/pressed/focus states,
   high contrast, reduced motion, runtime text scaling and RTL/direction.
2. **Commands:** stable typed IDs, availability, checked state, pending state,
   context, shortcut conflicts, menu/toolbar/palette consistency and Ctrl-based
   desktop bindings. No arbitrary string dispatch or silently dropped commands.
3. **Focus/overlays:** nested scopes, one active interactive layer, capture,
   focus restoration after deletion/unmount, clipping/placement, Escape/outside
   policies, tooltips that never take ownership, and blocked background hover.
4. **Collections:** stable identity, shared selection/range anchor, ordered
   visible traversal, reusable virtualization, focus reveal and model mutation
   recovery. Never use row indexes as persistent selection IDs.
5. **Editing/forms:** document transactions, draft versus committed values,
   validation lifecycle, clipboard/IME adapters and field-label associations.
6. **Async/lifetime:** cancellation tickets, stale-response rejection, bounded
   queues/backpressure, loading/error/empty models, retry and teardown.
7. **Accessibility/i18n:** role/name/state/relationships/value/action metadata,
   host tree publication and events, keyboard equivalents, localized formatting,
   bidi text, logical layout, date conventions and status announcements.
8. **Host capabilities:** clipboard, files/dialogs, window actions, preferences,
   timers, task execution, external links and platform accessibility. no_std
   consumers provide adapters; disabled capability must not look successful.

## Implemented in this pass (source, not release acceptance)

| Layer | Actual source | Verification / limits |
| --- | --- | --- |
| Selection, sort and virtual-window arithmetic | `crates/kit/src/behavior/collection.rs` | Tested; window arithmetic alone is not a virtualized widget |
| Action/single/multiple grouped selection | `behavior/group.rs`, `controls::{ChoiceGroup,choice_group}` | Tested policy, horizontal/vertical composition and one Tab stop; joined-edge variants, native semantics and runtime acceptance pending |
| Hierarchy | `behavior/tree.rs`, `controls::{Hierarchy,tree_view}` | Validated acyclic topology, atomic updates, stable rows, parent/child navigation, hidden selection preserved on collapse; non-virtualized view, pointer modifier handling, DnD, reveal and host semantics pending |
| Forms | `behavior/forms.rs`, `forms.rs`, `controls::{Check,checkbox}` | Required-field policy, tri-state check, persistent labels/description/error, integer draft/commit and stepping; decimal/localized entry and full native semantics pending |
| Date/time/color values | `behavior/forms.rs` | Tested Gregorian arithmetic, validated time and hex RGBA values; **not calendar/time/color picker widgets** |
| Overlay stack | `behavior/overlay.rs` | Tested nested ownership/dismissal and restoration tokens; not yet connected to the legacy binary `InputGate`/native focus service |
| Panel/anchor geometry | `behavior/layout.rs` | Tested bounds, impossible-minimum fallback, popover flip/clamp; **not a finished SplitPane/Popover widget** |
| Async and notification lifecycle | `behavior/task.rs` | Tested cancellation/stale result rejection and bounded notification storage; executor/host and notification surface integration pending |
| Data table | `behavior/table.rs`, `table::{DataTable,data_table}` | Typed numeric/text sorting, stable row selection, bounds, atomic replacement and small-table view; cell editing, header keyboard navigation, resizing gestures, virtualization and semantics pending |
| Feedback | `feedback.rs` | Alert, Badge, EmptyState, Progress, Separator compositions; full native build, layout review and announcements pending |
| Styling | `style.rs`, `theme.rs` | Shared intent/density/geometry definitions and status roles; not every legacy literal migrated and no fresh contrast/render acceptance |
| Consumers | Studio Controls and Collections | Shared group, checkbox, numeric input, hierarchy, table and alert specimens; source/macro checked, not freshly captured or runtime accepted |

The live catalog now has **15 groups**, including these partial APIs. No group
is promoted to accepted because its name exists or its macro expands.

## Dependency-ordered implementation and acceptance

- **Foundation gate:** complete theme/state tokens, command/focus/overlay and
  semantics services; integrate and test consumers, not parallel unused services.
- **Forms gate:** finish action and form families, pickers, validation/IME and
  composite focus behavior across light/dark, disabled and large-text states.
- **Collections gate:** finish list/grid/table/tree/treegrid, virtualization,
  pointer/keyboard selection, editing, resize/reorder and drag/drop.
- **Desktop gate:** finish navigation, app shell, split/docking layouts, all
  transient surfaces and standard dialogs on typed host adapters.
- **Workflow gate:** finish feedback, cancellation/retry, complex-editor hooks
  and representative multi-component application workflows.
- **Release gate:** actual workspace/no_std builds, event-driven tests, fresh
  native screenshots, keyboard-only traversal, host accessibility, scaling and
  high contrast, clipping/overlay/drag regressions, stress/idle/teardown tests.

Every component needs public API, explicit state/input contracts, an actual
shared consumer/specimen, tests and source-linked evidence. Accepted, partial,
policy-only and missing must remain different states. A release cannot pass
while the baseline above contains missing families or unverified interaction.

## Validation evidence from this pass

- **41 behavior tests passed**, and the exact policy sources compile in a
  no_std wrapper. This covers policy logic, not rendering or host services.
- Existing **11 editing-policy + 33 Studio-core tests passed** again.
- Genuine pinned Mirui 0.51 macros parsed/lowered all listed changed UI/model
  sources successfully. Macro expansion is not full Rust type checking.
- Rust 1.92 nightly restored under ignored `.cache` via npm-hosted toolchain
  distributions. Formatter is real rustfmt/WASM. No toolchain blobs tracked.
- `cargo check --locked -p mirui-kit` **failed before compilation** because
  `https://index.crates.io/config.json` could not be reached (TLS EOF). Do not
  describe the workspace or new widgets as compiling successfully.
- New Studio model regressions were added but **not executed** because they
  require the blocked dependencies. No fresh screenshots, accessibility or
  interactive runtime acceptance is claimed.

### Existing CI is red, not assumed passing

The pre-existing merged branch workflow was checked during this pass:
[master run 40074148391](https://github.com/smart-foundation/mirui-kit/actions/runs/40074148391)
failed at **Full workspace tests, including existing Files regressions** (exit
101); subsequent checks and captures were skipped. Rust setup and native package
installation succeeded. The failure's underlying compiler/test output could not
be retrieved: the log download host returned EOF in this environment, and the
check annotations expose only the exit code. This is **not** evidence that the
new work passes, and the precise old failure must not be guessed. Fixing and
rerunning the full build/CI remains a release blocker.

## Native verification update (supersedes the earlier local-only evidence)

The earlier blocked-build/red-CI section above is historical, not current status.
Native workspace tests, RISC-V no_std checks, SDL compilation and capture builds
passed on this branch in runs
[40080688593](https://github.com/smart-foundation/mirui-kit/actions/runs/40080688593),
[40080791046](https://github.com/smart-foundation/mirui-kit/actions/runs/40080791046)
and [40080850464](https://github.com/smart-foundation/mirui-kit/actions/runs/40080850464).
The policy suite now has **52 passing tests**. These results do not establish
full component/interaction/accessibility acceptance.

- Shared `sizing::ContentFit` now aggregates native text/container geometry with
  explicit overflow/depth/node-budget diagnostics. The native nested-layout and
  idle-stability regression passed. Native overview rendering confirms the old
  section overlap is gone. Visibility/wrapping/text-scale tests are newly added;
  their native results must be checked separately.
- `calendar::{Calendar,calendar}` is a real single/range date grid, with bounds,
  configurable week start, keyboard/month/year navigation and disabled input.
  Actual native pointer/keyboard/disabled tests passed. Studio now mounts it and
  the shared option list. Time/color pickers, localized date-entry/popup and
  host accessibility are still not supplied by this calendar.
- `Actions::activate` now returns explicit wrong-layer/backpressure errors;
  native FIFO regression passed. Most internal tap handlers still discard those
  errors; automatic diagnostics/recovery are not claimed.
- Six complete native route PNGs from commit `6ee8e69` were recovered through
  Checks annotations with every PNG chunk CRC and IEND verified. All six were
  visually reviewed. Collections exposes table/tree alignment and description
  clipping; Foundations exposes a clipped font explanation. Fixes now use native
  custom Button metrics, definite row geometry and wrapping paragraphs. New
  table regression covers stable-ID replacement and column resizing as well as
  actual header/cell geometry. Fresh native acceptance is required for these
  changes; the older six captures cannot prove them.

Still open: nested overlays/focus ownership, full semantics bridge, fundamental
picker/navigation/layout/collection families, editable/virtualized grid,
modifier/typeahead wiring, direct manipulation, semantic typography migration,
rich native elevation, freeze/stress investigation and complete scaled/theme
workflow coverage. This is an implementation checkpoint, **not kit completion**.
