# Target blueprint: a kit for timeless Naledzi apps

This document is a **proposed contract**, not a description of implemented APIs.
See the code findings in [01](01-code-defects.md), [02](02-visual-accessibility.md)
and [03](03-architecture-developer-experience.md). Do not freeze these APIs until
Files and a second, unrelated app have exercised them.

## 1. What “timeless” means here

A timeless app is understandable, trustworthy and adaptable over years.
It does not depend on copying one current desktop’s colors or window buttons.
A premium surface is a consequence of strong hierarchy, readable text, precise
alignment, predictable behavior and appropriate material separation.

Non-negotiable properties:

1. A visible object and its action target are the same stable identity.
2. A control either works, explains why it cannot work, or is not presented.
3. Success means the operation completed, not that a button was clicked.
4. A modal layer owns its input and focus until it releases them.
5. A user can always cancel a reversible interaction before commitment.
6. User data is not hidden to keep a renderer benchmark low.
7. Readability takes precedence over thinness, density and decorative restraint.
8. Keyboard and assistive interaction are component features, not app patches.
9. Layout responds to available space and actual text metrics.
10. Global appearance policy does not require rewriting screens.
11. OS services remain outside reusable view/state code.
12. The default path is convenient; advanced customization does not bypass safety.

“Fast to build” means these guarantees come with the component.
It must not mean asking every app author to remember twenty invisible rules.

## 2. Architectural boundaries

Suggested responsibilities, not mandatory crate names:

```text
Mirui 0.51 rendering / ECS / composition capabilities
    |
Kit foundations: theme roles, typography, metrics, icons, motion, semantics
    |
Kit services: focus, overlays, input capture, commands, scoped identity
    |
Controls: field, button, checkbox, slider, menu, tabs, scrollable collection
    |
Patterns: settings form, master-detail, document shell, collection workspace
    |
Applications: Files, Settings, Editor, other Naledzi tools
    |
Application domain adapters: filesystem, documents, preferences
    |
Host adapters: Naledzi IPC/window manager/input/accessibility; SDL test host
```

Dependencies point toward shared contracts, not from the kit into Files.
Do not make the core kit depend on a filesystem model just because Files is the
first demonstration. Do not build a universal domain framework before two apps
show which abstractions are actually shared.

Keep `no_std` + `alloc` available for the reusable layers.
Require explicit host capability injection rather than hidden `std` calls.
Define which services are mandatory and which have a graceful unsupported state.
Do not replace Mirui’s renderer as a substitute for fixing composition/input.
Isolate confirmed engine limitations behind one adapter with an upstream issue
and a regression test; do not spread workarounds into every application.

## 3. Foundation contracts

### Typography

Use semantic roles rather than physical weights at component call sites.
A role resolves family/fallbacks, size, line height, weight and optional tracking.
Start with a small useful set: Body, BodyStrong, Caption, Label, Action,
SectionTitle, WindowTitle, DialogTitle and Code.

Body must be comfortably readable by default; Medium can be chosen centrally.
Headings need hierarchy, not necessarily a heavier version of every label.
Retain optional weight coverage without forcing nine atlases into every app.
Text scale is independent of physical DPI and must drive geometry.
Exact-size coverage rasters are a rendering strategy, not a typography API.
Atlas selection, shaping failures and fallback behavior need observable tests.
Document which scripts and glyph ranges are supported in the release.
Unknown glyphs must not silently disappear from user filenames.

### Color and surface

Provide roles for Canvas, Surface, SubtleSurface, ElevatedSurface, Border,
TextPrimary, TextSecondary, TextDisabled, Accent, OnAccent, Selection,
OnSelection, Focus, Error, Warning, Success and associated foregrounds.
Selection, focus and active-window state are different concepts.
Do not use the same blue for every link, small text, focus border and filled button.
Derived translucent states must be evaluated against their actual backgrounds.
A high-contrast policy may remove effects and strengthen borders without app edits.
Kind colors and icon colors must resolve through the same policy.
Never make color the only indicator for destructive, selected or invalid state.

### Metrics and density

Provide a small spacing ladder and semantic metrics for fields, buttons,
row height, icon size, group separation, panel insets and focus clearance.
Define compact and comfortable density; both remain keyboard-operable.
Separate painted size from hit target size for small icons and scrollbar thumbs.
Avoid a giant catalog of tokens named after every current literal.
Components accept constraints and content; they should not require app authors to
calculate a text editor’s internal two-pixel inset.

### Elevation

Use a small hierarchy: flat, raised control, popover, dialog and floating window.
Each role owns border, ambient/key shadow, clipping, backdrop and interaction policy.
Toolbar controls are usually flat until state warrants separation.
Primary actions may use restrained elevation, but not the same shadow as a modal.
The visual shadow is never a hit target.
A surface cannot keep a visible shadow when the surface is hidden.
Offscreen rendering and blur cost belong in measured budgets.

### Motion

Motion explains a state transition or object relationship, not idle decoration.
Provide durations/easing by purpose, interruption behavior and reduced-motion mode.
A pressed state must be immediate, even when a longer transition is running.
Loading animation stops when hidden/unmounted and can become a static placeholder.
Do not let a looping skeleton keep an otherwise idle application repainting.

## 4. Common interactive-control contract

Every interactive component declares:

- Stable scoped identity and accessible name.
- Role, value/state, enabled state and supported actions.
- Pointer, keyboard and assistive activation behavior.
- Focus-visible, hover, press, selected and disabled visuals where applicable.
- Capture/release/cancel rules, including focus loss and unmount.
- Controlled value or explicit internal state ownership, never ambiguous ownership.
- Validation/pending/error behavior where applicable.
- Layout constraints, text-scale behavior and overflow policy.
- Test hooks independent of English text and pixel coordinates.

Defaults should satisfy this contract without additional flags.
An application should not have to attach `HitTarget`, `InteractionFeedback`, a
custom view and a handwritten focus ring just to create one ordinary button.

## 5. Component inventory and minimum behavior

| Component | Contract needed for a useful first release | Current application evidence |
|---|---|---|
| Text/Label | Semantic role, wrap/ellipsis policy, selectable variant, fallback | Raw Text plus three sizes; F37/V01/V14 |
| Button | Label/icon, intent, disabled/pending, keyboard/focus, activation once | Mixed Button and tappable Views; V05/A03 |
| IconButton | Accessible label, tooltip association, target envelope | Many 20–28px Views lack a shared contract |
| Checkbox | Checked/mixed, whole-label hit target, Space, disabled | FmState-coupled painter; F07/F40 |
| Switch | Boolean setting semantics, keyboard, labelled state | Not implemented as a kit control |
| Radio/SegmentedGroup | Selected value, group semantics, arrow-key behavior | Hardcoded Files view switch and sort helpers |
| Field | Label, prefix/suffix, helper/error, padding, validation, submit | Repeated editor workaround; V04/A09 |
| TextArea | Multiline geometry, selection, scroll, IME/clipboard policy | No application-level reusable contract |
| SearchField | Clear action, search state, scope, results feedback | Toolbar document/filter wiring only |
| Slider | Numeric value, step, range, label, keyboard, capture | ValueChanged does not commit; F14 |
| Number/Select input | Typed value, invalid intermediate text, keyboard | Not implemented as reusable kit controls |
| Menu/MenuItem | Command-driven content, availability, keyboard, focus | Handwritten item IDs; F22/F23/F42 |
| Popover | Anchor, collision avoidance, ownership, dismissal | Options fixed at 610,46; V09 |
| Dialog | Stable target, labelled intent, initial/restore focus, validation | Generic OK/Cancel and current selection target |
| Tooltip | Trigger, hover/focus delay, placement, Escape, passive content | Static bubble only; V16 |
| ScrollArea/Scrollbar | Capture, thumb/rail distinction, keyboard, wheel routing | Custom bar with six-pixel rail; F28/F29 |
| List/Grid | Stable keys, full reachability, virtualization, focus, selection | Capped collections; F08/F10/F11 |
| Table | Header labels, typed sort, column alignment/resize, row identity | File list has no reusable table contract |
| Tree | Stable hierarchy, expand/collapse, keyboard, lazy children | Flat places are not a tree |
| Tabs | Labelled descriptors, selected value, keyboard, content lifetime | Count-only inspector TabBar; F30 |
| SplitPane | Min/max, pointer/keyboard resize, persistence | Sidebar/inspector fixed widths |
| Disclosure/Section | Heading, expanded state, focus, content lifecycle | No shared component contract |
| Badge | Passive status/count, overflow and accessible description | Static value and HitTarget; F31/F33 |
| Empty/Loading/Error state | Contextual explanation and useful recovery | Helper exists but collection does not use it |
| Progress | Determinate/indeterminate, task identity, cancel when safe | Static storage percentages; F30 |
| Notification | Priority, persistence, actions, readable content, announcement | Timed three-card queue; F35 |
| AppShell | Content constraints, host chrome, overlays, status, resize | File-specific composition; V06/V12 |
| CommandPalette | Registry, filtering, navigation, execution, context | Static rows, missing command, no key controller |

This is a coverage map, not an instruction to implement every component at once.
Ship a smaller verified Tier 1 before adding a broad incomplete catalog.

## 6. Service contracts that remove app-author work

### Focus service

Register logical focus order and group navigation.
Give overlays focus scopes and restoration targets.
Mount/unmount must transfer focus to a sensible surviving element.
Separate a file’s active keyboard item from a selected set.
Do not trap keyboard users in a dead control or behind a modal.
Expose semantic actions to the host accessibility bridge.

### Overlay host

Own one stack of typed overlay sessions with parent/invoker identity.
Define transient, modal and nonmodal behavior explicitly.
Resolve topmost input before dispatch, not from a later rectangle scan.
Outside press, outside tap, Escape and focus loss are separate events/policies.
An event consumed to dismiss a popup never activates underlying content.
Anchor geometry is relative to the application viewport, not arbitrary host pixels.
Focus, wheel, drag, hover and keyboard respect the same layer ownership.

### Command registry

Commands have stable IDs, labels, icons, scope, availability and typed context.
Surface shortcuts are generated from active bindings, not duplicated strings.
Menus, toolbar, palette and keyboard dispatch the same user intent.
Naledzi defaults use Ctrl conventions; bindings are host-remappable.
Editor-local commands take precedence over global commands when appropriate.
When unavailable, expose a reason or omit the command deliberately.
Do not bind shortcuts that conflict with text composition or platform policy.

### Collection/selection controller

Use stable item identity independent of sort order and recycled index.
Separate selected set, active item, range anchor and visible window.
Provide reveal/scroll-to-item and keyboard grid/list navigation.
Virtualization must retain identity and restore scroll/focus after mutations.
Collection states include loading, empty, content, filtered-empty and error.
Background refresh can preserve old content with explicit freshness state.
One canonical data source derives counts, metadata and operation targets.

### Drag/edit controller

Represent a drag session with source IDs, preview data, grab offset and operation.
Give pointer capture to the session until release/cancel; block incidental click.
Resolve allowed destinations through domain capabilities, not file color or kind alone.
Support Escape, focus loss, source removal and drop failure.
Inline editing owns draft text, validation, provisional identity and commit/cancel.
A new folder is revealed and focused wherever the command originated.
These policies are separate from overlay click-through prevention.

### Host capability adapter

Define input, clock/scheduling, window actions, clipboard, open-file requests,
accessibility events, preferences, notifications and resource-loading boundaries.
Use per-window queues rather than process-global pending booleans.
Define logical/physical coordinates and DPI conversion exactly once.
Handle suspend/resume, focus loss, display change and resource exhaustion.
Let the host reject unsupported actions with a visible, recoverable result.
Filesystem adapters remain application-domain services, not core-control dependencies.

## 7. Fast developer workflow

A new app should follow one supported path:

1. Choose the Naledzi or desktop/headless host adapter.
2. Install kit services once with a theme/type/density/accessibility policy.
3. Define app/domain state and typed commands.
4. Mount AppShell and compose controls using roles and constraints.
5. Bind async operations through request/result adapters.
6. Add fixtures and scenario tests using semantic/scoped test hooks.
7. Run a component gallery plus app-specific state/flow matrix.
8. Run no_std, feature, accessibility, visual and performance gates.

Public entry points need a minimal working example and a clear failure mode.
Avoid broad preludes that conceal mandatory setup without documenting it.
Document which APIs are stable, experimental or renderer-version adapters.
Every component example must compile and actually exercise its behavior.
A “copy this screen and replace FmState” recipe is not a reusable kit API.

## 8. Composition recipes worth shipping

### Settings app

Sectioned labelled forms, search, boolean/select/numeric preferences,
validation, restart-required state, per-setting persistence result and reset.
Use real radio/checkbox/switch semantics rather than decorated passive rows.
Distinguish immediate changes from Apply/Cancel forms deliberately.

### Document app

Document lifecycle, title/dirty indicator, Open/Save/Save As, unsaved-close
confirmation, find, undo/redo and host clipboard/input services.
A failed save must not clear the dirty indicator or close the document.
Multi-document support must not require process-global IDs or state.

### Collection workspace / Files

Location/history, content actions, search, view mode, full collection navigation,
selection, contextual inspector, operation progress, inline rename and recovery.
Domain adapter owns permissions, real hierarchy and filesystem result semantics.
The UI must not pretend a flat Place enum is an actual folder tree.

These apps validate reuse better than extracting every current helper in advance.

## 9. Staged public API promise

Tier 1: foundations, button/field/checkbox, focus/commands/overlays, basic shell,
notifications and a fully reachable collection.
Tier 2: tables/trees, rich editor integration, resizing, complex selection/drag,
contextual menus, task progress and host accessibility polish.
Tier 3: advanced pickers, rich content widgets and optional animation/materials,
only where real app demand and measured resource budgets justify them.

A component is stable only after defaults, states, input routes, host boundaries,
no_std behavior and failure cases have tests and documentation.
A missing Tier 2 widget must not be advertised as complete because it has a painter.

## 10. Guardrails against overengineering

- Keep the working renderer and asset pipeline unless measured evidence requires change.
- Fix state/identity defects before extracting them into reusable infrastructure.
- Prefer one complete control over five nominal controls with handwritten app wiring.
- Share behavior first, visual presets second; do not freeze the current Finder skin.
- Avoid magic global service state and strings as identity.
- Do not use truncation as virtualization or a frame cap as a freeze diagnosis.
- Make customization explicit, constrained and testable.
- Preserve simple applications: no mandatory filesystem, giant command enum or async runtime.
- Introduce generics/traits only where a second consumer or host proves the boundary.
- Require evidence for “done”: test, fresh image, interaction trace or target measurement.
