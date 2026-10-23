# Real component coverage, source truth and implementation boundaries

This inventory is bounded by the checked-in application. Mirui’s full upstream
catalog is not installed here; no claim is made to inventory every upstream API.
Studio must cover every shipped kit component, not pretend every planned control
already exists. Add registry entries and coverage scenarios as the kit grows.

## Existing source versus a reusable control

| Component/capability | Current source evidence | Studio use | Work required |
|---|---|---|---|
| Row/Column/View/layout | `fm/src/ui/*` | Entire shell and recipes | Constraint/density policy; measured layouts |
| Text/Paragraph | `text.rs`, content/dialog | Headings, labels, examples | Semantic role resolver and overflow/accessibility policy |
| Button | chrome/menu/dialog | App actions and specimens | Shared intent, focus, disabled/pending contract |
| Icon | `icons.rs`, UI modules | Nav, commands, kind and action cues | Semantic identity, labels and theme roles |
| HoverSurface | `ui/hover.rs` | Real interactive surfaces | Correct routing, focus/selected state composition |
| FieldFocus | `ui/controls.rs` | All actual editor frames | Fold into shared field; don’t repeat hidden engine-border hacks |
| TextInput/TextEditing | chrome/dialog/palette | Catalog search and Controls | Controlled binding, validation, focus/select-all |
| Multiline editor | No reusable app adapter verified | Workbench | Verify real upstream support, build adapter and input tests |
| Checkbox painter | `ui/controls.rs` | Metadata, resize and app settings | Remove FmState coupling; whole-label control/semantics |
| Slider | `ui/options.rs` | Quality setting | Repair emitted-value binding, keyboard and capture |
| Segmented view group | `ui/kit.rs` | Format/density/view choices | Generic value/options controller and group semantics |
| NumberField | No reusable implementation | Resize width | New adapter using real text editing and typed validation |
| Menu | `ui/menu.rs` | Context actions, app menu | Command-driven registry, keyboard/focus and availability |
| Command palette | `ui/palette.rs` | Global Commands | Current icons/row enumeration patch; remaining controller work |
| Tabs | `ui/inspector.rs` | O02 inspector tabs | Labelled descriptors, actual selection/content and keyboard |
| Scroll/ScrollBar | content/scrollbar | Routes, palette, collections | Shared thumb geometry, hit envelope, capture and full reachability |
| List/Grid | `ui/content.rs` | Collection recipe | Stable identity, full virtualized/paged reachability; no hidden caps |
| Table headers | Not implemented in file list | Collection list | Typed sort, headers/semantics and column policy |
| Inline editing | model/content | New folder and rename | Reactive editor lifecycle, reveal and stable target |
| Drag preview | `ui/menu.rs` | Asset movement | Reactive session identity; lifted real item, not text pill |
| Dialog | `ui/dialog.rs` | Review, unsaved, reset | Target snapshot, labels, focus, failure and sizing |
| Popover | `ui/options.rs` | View options | Anchoring/collision/ownership; no literal screen coordinates |
| Tooltip | `ui/kit.rs` | Icon/help affordances | Actual trigger/delay/focus/placement; current helper is only a bubble |
| Badge | `ui/kit.rs` | Passive support/count annotations | Reactive value and passive semantics |
| Empty state | `ui/kit.rs` | Collections/discovery/results | Connect to real state enum and recovery commands |
| Skeleton | `ui/kit.rs` | Explicit loading scenario | Register painter/systems correctly; reduced-motion and disposal |
| ProgressBar | status/inspector | Preview/review jobs | Actual task progress/indeterminate state; no decorative fixed percentage |
| Notice/history | `model.rs`, `ui/status.rs` | Task results + app activity | Severity persistence/actions/history; remove copied three-card implementation |
| DropShadow/OffscreenRender | menu/dialog/status | Shared elevation roles | Visibility/order/performance centralization |
| Theme/font registration | theme/font | App and specimen policies | Configurable roles, host preferences and safe fallback |
| AppShell/navigation | `ui/mod.rs`, sidebar/chrome | Studio itself | Extract independent composition; no Files-specific state |
| Clock/plugin | sync/desktop | Recovery/task timing | Capability-based monotonic scheduling and cancellation |
| Host capture | desktop/snapshot/timestamp | Diagnostics screenshots | Unique names, typed completion, truthful snapshot generation |

Paths in this table are relative to `crates/` unless prefixed with target/tool names.
For example `fm/src/ui/palette.rs` means `crates/fm/src/ui/palette.rs`.

## Capabilities deliberately not faked

Switch, RadioGroup, Select/ComboBox, Tree, SplitPane, rich text, date/time picker,
file picker, charts, media controls and cross-app drag are not claimed complete.
If adopted into the kit, give each a real implementation, state/keyboard contract,
registered recipe and acceptance evidence. Until then, catalog entries say Planned
or Unsupported, not a disabled decoration labelled as a live component.

A kit component may have multiple useful recipes without earning a separate route.
Keep six routes; grow the registry and recipe content, not the navigation chrome.

## Proposed implementation structure (not existing crates)

```text
shared kit
  foundations/{type,color,metrics,icons,motion}
  services/{commands,focus,overlays,input,identity}
  controls/{button,field,checkbox,slider,tabs,scroll,...}
  patterns/{shell,form,collection,editor,feedback}

showcase app
  catalog        component/recipe descriptors; truthful support state
  session        app route/history and recipe sessions
  screens        composes shared components only
  fixtures       safe data + deterministic async/failure adapters
  scenarios      named flows + assertions, not secret global mutation knobs

hosts
  desktop        SDL adapter, window/input/clock/clipboard/capture
  headless       deterministic surface and fixtures
  naledzi        explicit OS/window/accessibility/domain IPC adapters
```

Do not create all directories as an empty abstraction exercise. Implement one
vertical slice and extract the shared contract into the smallest sensible modules.
Files remains a consumer and regression app; Studio does not import its model.

## Component descriptor contract

Conceptual fields:

- Stable ComponentId, display name, category, aliases and short purpose.
- Shared kit API path/version and mounting recipe ID.
- Available/adapter-required/planned state plus per-host capability reasons.
- Supported interaction/state scenarios and accessibility semantics.
- Related recipes and commands.
- Last actual verification evidence by build/host/scale, or Not verified.

Registry drives discovery, coverage, inspector, deep links and scenario enumeration.
No hand-maintained duplicate lists of 16 rows for 17 registered entries.
Runtime controls report instance identity scoped under the app mount and recipe.
Language changes do not change component/command IDs or icon mapping.

## Command descriptor contract

Conceptual fields: CommandId, label, aliases, icon, category, active scope, binding,
availability reason, checked state, target resolver and dispatch handler.
Action context contains route, session generation and validated target IDs/versions.
Descriptors generate toolbar/menu/palette/help labels; handlers perform one intent.
Palette icons are a presentation of this semantic descriptor, not a second label map.
A stateful action such as theme switching names its resulting action or shows its
checked value; do not leave a static “Toggle something” where the outcome matters.

## New-app developer recipe

1. Install kit services once with explicit host capabilities and appearance policy.
2. Define domain model and typed commands; inject a fixture adapter for tests.
3. Compose AppShell, route layout and shared controls using public roles/constraints.
4. Bind values/intents through shared controllers; no app-private input hit testing.
5. Add catalog/recipe metadata if exposing the app as an example.
6. Test a complete user flow, including error/cancel/keyboard at supported sizes.
7. Run default/no-default/no_std and host-specific gates; inspect fresh screenshots.

Studio’s Recipe tab must expose the exact public version of this path, compiled
as an example/test. Do not publish speculative Rust code that appears copy-pastable
but cannot compile against the repository. API sketches remain labelled conceptual.

## Dogfooding acceptance

| Studio UI | Must reuse the same shared component as |
|---|---|
| Global discovery field | Controls Name/Search specimen |
| App navigation | Navigation recipe/group |
| Commands trigger/palette | Palette recipe, Files command surfaces |
| Appearance checkboxes/segments | Controls checkboxes/segments |
| O02 tabs and scroll | Tabs/ScrollArea specimens |
| Reset/unsaved dialogs | Review/dialog specimens |
| Activity/errors | Notification/feedback specimens |
| Diagnostics selectors/buttons | Ordinary Select/Button controls, not private widgets |

An automated dependency/import review plus interactive gallery confirms the claim.
A screenshot that looks similar is not proof of shared implementation.

## Current Files palette patch

`crates/fm/src/ui/palette.rs` now imports Icon, draws a leading 16px icon per row,
and selects a semantic asset with an exhaustive Command match. Sorting commands
share Sort; navigation commands use destination icons. Labels remain visible.
Rows are generated from `0..COMMANDS.len()` instead of a fixed index list; content
extent derives from the same registry length. Unit coverage exercises icon selection.

This is a limited current-app patch, not the complete future registry. It has not
been compiled/rendered here. Keyboard active-result behavior, empty-state sizing,
command availability, self-command filtering, focus restoration and command-state
icons remain explicitly required work; adding icons does not close those gaps.
