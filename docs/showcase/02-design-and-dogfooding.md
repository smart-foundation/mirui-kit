# Shared visual system, real components and dogfooding

## One implementation per behavior

AppShell, NavItem, IconButton, SearchField, CommandPalette, Field, Checkbox,
SegmentedGroup, Slider, ScrollArea, Menu, Dialog, Notice and Inspector are shared
kit components. The same field appears in the app’s own settings and the live form.
Do not introduce `ShowcaseButton` or custom overlay routing to make the demo look better.

Each specimen has an instance-scoped ID, model value and shared component type.
The component inspector reports its type/role/state and public recipe, not an
invented code snippet unrelated to what is mounted. Changes made through the app’s
own settings must exercise the same binding path as the specimen settings.

The app provides fixture data and task controllers. Shared components provide
geometry, painting, semantics, input, focus and error presentation. Host adapters
provide clock, clipboard, windowing, accessibility and persistence capabilities.

## Typography and readability

Proposed semantic defaults, to validate in rendering rather than bake in blindly:

| Role | Starting size | Weight intention | Use |
|---|---:|---|---|
| Body | 14 | Medium/readable default | Main copy and essential metadata |
| Label/Action | 14 | Medium | Fields, navigation, buttons, palette rows |
| Caption | 12 | Readable regular/medium | Secondary annotations only |
| SectionTitle | 16 | Semibold | Group headings |
| PageTitle | 22 | Semibold | Route headings |
| Code | 13–14 | Host code face | Read-only recipe/example code |

Line height derives from actual role metrics, not a fixed field inset workaround.
Caption is never used as an excuse to make essential task content tiny.
User text scale is independent of display DPI; reflow at increased scale is required.
No giant 48px marketing title consumes the usable workspace.
Inter remains the default UI family if supported; do not rebuild the renderer.
A code face needs an explicit supported asset/fallback, not a claim the current
Inter-only asset set already provides a monospace family.

## Metrics and alignment

Spacing ladder: 4, 8, 12, 16, 24, 32 logical pixels.
Default control height starts at 32; comfortable at 36, dense row at 28 where tested.
Painted icon size typically 16–20 with a larger interactive envelope.
Form labels align to their fields; helper/error text aligns to the editable content.
Focus clearance is reserved and may not be clipped by a rounded card.
One or two levels of section grouping are enough; avoid nesting every control in a card.

Surface roles: canvas, subtle navigation, content, raised popover/dialog.
Use a border plus restrained elevation when it communicates layer ownership.
Primary button shadow, if used, is a control role—not a miniature window shadow.
Selected, hovered, pressed and focused have distinct state combinations.
Color contrast is validated on actual foreground/background pairs and composites.

## Icon contract, including the palette

Icons use semantic identity, a consistent view box and optical size policy.
Do not select an icon from translated label text; command identity owns its icon.
Use existing vector assets where appropriate; missing icons are a tracked asset task.
Leading command icons occupy a stable 16–20px column, decorative in semantics;
the row’s accessible name is its text and optional description, not the icon name.
Color reinforces state, never distinguishes all commands by arbitrary rainbow colors.

| Command family | Icon meaning | Required accompanying text |
|---|---|---|
| Navigation | Section icon | “Go to Controls”, etc. |
| Create/edit | Add/document/edit | Object-specific action label |
| View/panes | Grid/list/panel | Target view or pane name |
| Appearance | Appearance/moon | Selected theme/appearance task |
| Sorting | Sort | Criterion and direction |
| Inspect/help | Info/code | Inspect selected component / Help |
| Recovery | Restore/reset/warning | Exact scope and consequence |
| Capture | Image/camera | Capture local example |

A shortcut column has stable alignment but occupies space only when bindings exist.
Disabled rows remain readable and explain why. Destructive items use a deliberate
semantic role, not a red icon alone. Shortcuts, icons, tooltips and labels come from
one descriptor. Current Files patch supplies icons only; a full registry is future work.

## Accessibility semantics by default

Every interactive component has name, role, value, state and available actions.
Checkbox label/padding toggle one shared control, not two nested controls.
Arrow keys navigate grouped controls, Tab moves between groups, Enter/Space act
according to role. Focus-visible exists for every control, not only editors.
Passive badges, metadata and decoration do not become fake focus/hit targets.
A tooltip appears for keyboard focus as well as pointer hover; essential information
is not tooltip-only. Live status announcements are concise and do not steal focus.

Reduced motion affects specimens and the app itself. High contrast may replace
soft shadow separation with stronger borders. Accessible defaults must remain
useful without a special “accessibility lab” being opened first.

## Live experiments versus global settings

Specimen state is isolated to its recipe session. A dark button specimen does not
make the entire app dark. Foundations offers two clearly labelled modes:
Preview specimen and Apply to Studio. Apply uses a recoverable settings transaction.
Host appearance changes, unavailable fonts and invalid presets have defined fallback.
The app must remain operable while demonstrating a disabled/invalid specimen.

## Truthful component availability

“Existing primitive” means imported/used in current source, not reusable and complete.
“Needs kit adapter” means behavior/semantics/installation must be completed once.
“Planned” means not usable yet; show a capability note, not a fake live switch.
The coverage table documents these states. A non-working specimen cannot wear a
Live badge just because it resembles a control in a wireframe.

## Component inspector presentation

The inspector is an O02 drawer containing four tabs built from the real Tabs control:
Use, States, Recipe, Evidence. Use explains when this component is appropriate.
States shows named combinations without mutating the app’s global policy.
Recipe shows the actual public composition used by the selected instance.
Evidence shows last measured/tested build, or Not verified—never a fabricated badge.
Copy works through the real clipboard capability; unavailable clipboard is explicit.
Avoid exposing incidental ECS internals as the default developer interface.
