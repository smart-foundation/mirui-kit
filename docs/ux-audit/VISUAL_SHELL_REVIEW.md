# Visual shell audit — before another redesign

Date: 2026-10-24 · Branch: `arena/db7ace04-mirui-kit`

**This pass is audit-only.** No header, window control, font, icon, layout or
renderer was removed or restyled. Existing uncommitted application work is left
intact. Recommendations below are not claims of implemented or visually accepted
fixes.

## 1. What was actually investigated

- Viewed the historical desktop-window, context-menu and New Folder screenshots.
  Inspected the current sidebar, toolbar, shell, grid, menu, dialog, status,
  inspector and options sources rather than treating the screenshots as current.
- Read Mirui **v0.51.0** software rounded-quad implementation, DropShadow,
  output-dependency documentation, scoped path clipping and raster ppem selection.
- Inspected the actual TTF headers: source Regular = **400**, Medium = **500**,
  Semibold = **600**, Thin = **100**. These are static faces, not variable fonts.
  Inspected which faces the application registers and where it still uses 11px.
- Calculated the grid's declared geometry at multiple widths, with/without the
  inspector. Read PNG dimensions directly, without rescaling the images.

Reproduce metadata/arithmetic evidence:

```sh
python3 tools/audit-visual-contracts.py
```

Recorded output: [visual-contracts.json](evidence/visual-contracts.json).
This probes source and file metadata, **not live raster selection, packed MIRX
validity, pixel quality, performance or visual approval**.

The historical application PNGs are 1180×720; the decorated desktop screenshot
is 1215×780. Current SDL source requests **1400×900**. These files are therefore
not proof of the current larger host. Current source also still contains the
22px application `titlebar()`; I cannot substantiate “the header is absent now”
from this checkout. I can substantiate that its layout relationships are wrong.
No older, provenance-matched before-Inter render was available to prove which
family rendered better.

## 2. Reported issues: findings and recommended decisions

### R1 — Corners: curvature is not the same thing as antialiasing

**Confirmed source:** `quad-aa` is enabled. Mirui's rounded quad implementation
explicitly describes circle centers/radii: the default shape is a circular
rounded rectangle, not a continuous-corner/squircle primitive. Circular arcs
meet straight edges tangentially, but curvature changes abruptly there.

A hard-looking transition can coexist with correctly antialiased pixels. Simply
increasing every `border_radius` makes controls more pill-like; it does not
create a flowing corner transition. Conversely, increasing AA does not change
the shape of the curve. The user's concern should not be reduced to “more AA.”

**Decision:** aim for slightly generous, flowing corners without capsule buttons.
Use shared shape roles, not scattered radii. Initial *specimen candidates*, not
accepted final values: controls 8–10 at 40px height, menus 12, dialogs 16,
outer window 18–20. Avoid height/2 radii except genuine circular/pill controls.

Mirui already supports vector paths and `PaintCtx::with_clip_path`. If continuous
curves require a shared native-path surface, reuse those APIs; do not replace
the renderer. A scoped paint clip does **not** automatically give arbitrary
child subtrees the same clipping. Fill, border, focus outline, clipping and
shadow silhouette must agree before claiming a new shape works. Keep generous
rectangular hit areas where appropriate; do not shrink accessibility targets
just to match the decorative contour.

Validate actual-size button and dialog crops at 1×/1.25×/1.5×/2×, light/dark,
with default/hover/pressed/focused states. Do not assess only enlarged screenshots.

### R2 — Small menu symbols: use a true outline action family

**Confirmed:** shared `icons.rs` identifies the current set as Solar **Bold**.
`menu_row` renders those filled silhouettes at **14px**. `menu_icon` also maps
commands from their English labels, rather than stable action identities.

**Decision:** use a coherent outline/linear family for menu and toolbar actions,
starting with 16px optical size and testing sufficient stroke weight in both
themes. Keep filled, colored folder/file artwork in the content grid where it
communicates object identity. Do not globally replace every file icon.

Do not “outline” the existing filled silhouettes by drawing a border around
them. Import actual outline geometry. The present SVG converter assumes filled
paths: inspect stroke width, joins, caps, multiple paths, transforms and fill
rules before using it on linear SVGs. Expand strokes faithfully ahead of time
or use supported native strokes; do not lose them during conversion. Preserve
licenses/attribution. Map icons from command IDs, and share semantic mappings
between menus and the palette.

### R3 — The bigger SDL window should expose the whole app, not replace its UI

**Confirmed:** SDL is already configured as 1400×900, but the Files shell still
fills 100% of the host. Only its top chrome is inset. There is no single floating
application frame with a common rounded silhouette, gutter and shadow.

**Decision:** preserve all existing header functions. Separate:

1. **Host canvas:** the larger SDL surface, with native OS decoration retained.
2. **Application frame:** an initially 1180×720, centered specimen, including its
   header, sidebar, content and footer, with a common silhouette and shadow.
3. **Responsive app mode:** fit the frame to smaller hosts with intentional
   reflow; never just crop it or stretch a small framebuffer screenshot.

Resizing the host alone must not delete the title/header, controls, inspector,
status or actions. Nor should the SDL host paint fake close/minimize buttons
that compete with functional native OS controls.

All content measurements and popup clamps must use the **application frame's
actual rect**, not the larger host viewport. Otherwise centering the app simply
moves the grid/menu bugs into a new coordinate space.

### R4 — Sidebar header and window controls: qualified yes, not blind imitation

Current app “traffic lights” are three decorative Views, without host actions.
The decorated desktop screenshot already has functional native window controls.
This mixes two window-control authorities. The sidebar is always mounted and
has no real visibility-toggle model in the inspected Files source.

**Recommended arrangement:**

```text
larger SDL host canvas — keep its native title bar/controls
    ┌──────────────────────┬─────────────────────────────────────────┐
    │ sidebar cap          │ CONTENT TOOLBAR                         │
    │ title/controls*  [◧] │ [Back Forward] location   search  tools │
    ├──────────────────────┼─────────────────────────────────────────┤
    │ Places               │                                         │
    │ navigation           │ content                         inspector│
    │                      │                                         │
    │                      ├─────────────────────────────────────────┤
    │ sidebar surface      │ content status                          │
    └──────────────────────┴─────────────────────────────────────────┘
* App-owned OS-suite/custom-window mode only: functional window controls.
```

The sidebar toggle belongs at the **trailing edge of the sidebar cap** when the
sidebar is visible. Keep a stable leading-toolbar toggle when it is hidden, so
collapsing the sidebar does not remove its own recovery control. Focus must move
out of a collapsed pane and its hidden descendants must stop taking input.

Putting custom window controls in that cap can be space efficient: it shares
one aligned header band with the content toolbar instead of stacking unrelated
rows. It is appropriate only when the host delegates real window management to
the app. In ordinary decorated SDL, retain the native controls and use the cap
for application identity/navigation; don't pretend decorative dots are buttons.

Do not silently delete the existing title/header to get this layout. Preserve
its identity and functions, then deliberately integrate them. Native header
space and app header space are different things. Apple's guidance distinguishes
navigation sidebars from view-relevant toolbars [1](https://developer.apple.com/design/human-interface-guidelines/toolbars)
[4](https://developer.apple.com/design/human-interface-guidelines/sidebars), and
warns that hiding a title bar must preserve window interaction and drag space
[5](https://developers.apple.com/design/human-interface-guidelines/components/presentation/windows/).
Those are useful principles, not a reason to import Cmd shortcuts or macOS-only
behavior into this suite.

### R5 — Toolbar and footer should belong to the content pane

**Confirmed:** `ui::shell` composes full-width titlebar/toolbar above the
sidebar/content Row, and a full-width status bar below it. Back/forward therefore
sit over sidebar navigation rather than over the view they affect.

**Decision:** make the sidebar and the main content vertical siblings. Put
history/location/search/actions in the main pane's toolbar, and file counts,
sort/selection status in that same pane's footer. Align toolbar, grid/list and
footer content edges. Keep window-management controls separate from view actions.

Use smaller gaps **inside** groups and larger gaps **between** groups, for example
8 within and 16–24 between, subject to actual width tests. Order groups as history
→ location → search → view/action controls. Don't make New Folder the dominant
visual anchor or retain an unexplained palette button solely for symmetry.
Handle constrained widths with deliberate overflow, not overlapping fixed widths.
The existing fixed 240px location column must not consume space regardless of need.

### R6 — Sidebar counts need an explicit alignment contract

**Confirmed source risk:** `badge(count)` uses Auto width, horizontal padding,
11px type and an independent HitTarget. Counts of different digit lengths have
no common reserved numeric track. They are static `data::place_count` values,
not observed counts derived from current operations.

The exact latest misalignment cannot be certified without a fresh layout render;
old screenshots are not after-state proof. The source lacks the constraints that
would prevent it.

**Decision:** use a trailing count track sized from supported digit metrics, with
aligned right edges, consistent vertical centering and symmetric internal padding.
Make the count decorative/noninteractive so the entire row owns navigation. Test
0/8/99/512/9999 and changed font scale; never let the count push through the row's
trailing inset. If displayed as badges, align their **outer** right edges too.
Use live model counts or explicitly label fixture counts.

### R7 — Grid: the calculations explain the uneven fill

`grid_columns()` uses host viewport minus a hardcoded 216px sidebar and 32px
padding, ignores the 236px inspector, clamps to 4–8 columns, and `grid_row` lays
out fixed 104px tiles with 6px gaps.

Source arithmetic, before any new frame gutters:

| Host width | Inspector | Declared row | Actual content width* | Consequence |
| --- | ---: | ---: | ---: | --- |
| 1180 | hidden | 874 | 932 | 58px unused, left-aligned surplus |
| 1180 | 236 | 874 | 696 | 178px more row than available |
| 1400 | hidden | 874 | 1152 | 278px unused; 10 current-size tracks could fit |
| 1122 | hidden | 764 | 874 | 7 columns calculated although 8 fit exactly |

*Based on currently declared sidebar/padding; these are not measured live rects.

**Decision:** measure the scroller's actual content rect after sidebar, inspector,
frame and scrollbar allocation. Derive equal-width tracks with a minimum usable
label width. For minimum slot `m`, gap `g`, available width `W`, start with
`n = max(1, floor((W + g)/(m + g)))`, then divide the remaining width evenly among
those tracks. Keep icon size and text size independent of slot stretching.

Keep columns aligned between rows; the final incomplete row remains leading-
aligned with empty tracks. Do not spread its last two files across the window.
Include width/inspector changes in reactive row invalidation, not just listing or
icon-size changes. Remove the unconditional four-column minimum on narrow panes.

### R8 — Fonts: investigate the rendering chain, not just the family name

**What is established:**

- Registered default/medium/heading faces are 400/500/600, not the bundled Thin
  face. Their source TTF weights were read, not inferred from filenames.
- Many important Files labels and menu actions still request **11px Regular**.
  Medium assets being available does not mean the interface uses them.
- The bake recipe includes exact coverage for common 11/14/22px roles at the
  nominal 1×/1.5×/2× scales. A claim that every common size lacks coverage would
  be false. The recipe does, however, omit **19ppem**, reachable by 19px at 1×
  or 15px at 1.25×. Its SDF fallback covers that range. This is a recipe finding,
  not a validation of the actual packed asset or runtime-selected representation.
- Larger coverage representations use **4-bit alpha** in the recipe; small ones
  use 8-bit. This needs an A4-vs-A8 comparison, not an unsupported claim that its
  quantization is invisible or that it is necessarily the only cause.
- OpenType lookup flags are patched before baking. That can change positioning
  semantics; it does not justify blaming the font's glyph outline thickness.
- The old decorated screenshot mixes native OS title text with application
  typography. Different sizes/weights/renderers and screenshot scaling make an
  uncontrolled comparison unreliable.

**Decision:** keep the family undecided until a matched comparison exists. First
use a semantic type scale with readable body/action roles and explicit intended
weights, then compare the exact same strings, sizes, colors and output scales.
Do not make everything semibold, sharpen PNGs, enlarge low-resolution offscreen
textures or substitute a font blindly.

Capture 11/12/14/16/22px at 1×/1.25×/1.5×/2×, Regular/Medium/Semibold, light/dark,
Framebuf and SDL. Record selected representation, output ppem, bit depth,
logical/physical size, baseline, offscreen scale and final texture scaling. Test
kerning, punctuation and diacritics as well as simple ASCII. Compare the older
face only if its actual asset/settings can be recovered. No current evidence
proves that Inter itself is inherently the failure.

### R9 — Shadows must follow the output graph and paint order

**Confirmed:** menu code declares `menu_panel` before `menu_shadow`, while the
native DropShadow uses SourceOver and a tinted blurred source-alpha image. This
order risks shadow tint painting **over** the panel, rather than beneath it.
The dialog's shadow is declared before its source panel; options have no matching
shadow at all. A greater opacity everywhere is not a coherent elevation system.

Mirui's actual contract matters:

- DropShadow declares `#[depends_on(WidgetTexture)]`; this requests a source
  output. Explicit OffscreenRender is **not** the only way to establish one.
- Missing/unprepared source output makes the shadow renderer return without
  drawing. A declared component alone is not evidence of visible pixels.
- Configure the offscreen pool budget before constructing output consumers.
- In the inspected effect loop, shadow RGB is tinted from color, while alpha
  derives from source alpha and `with_opacity`; don't rely solely on changing a
  theme color's alpha to tune strength.
- Earlier shadow siblings may reference a later named source in the same static
  sibling scope. This is explicitly supported by the Mirui DSL documentation.
- Do not clip the shadow inside the panel it is meant to surround. Budget its
  expanded bounds and ensure hiding/unmounting the source retires the effect.

**Decision:** shared elevation roles: no cast shadow on ordinary flat buttons,
restrained contact/ambient elevation for menus and floating tools, stronger but
soft elevation for dialogs and the outer application frame. Two layers require
two effect entities, not duplicate same-type components on one node.

Use native DropShadow, tune with real light/dark renders, and prefer a stable
surface silhouette as source where possible so caret/text changes do not require
rebuilding a whole-content shadow. Measure pooled memory, readbacks and frame
cost before applying layered blur to every surface. “Rich” should mean believable
depth, not heavy black halos or a return of the reported eventual freeze.

## 3. Exactly five additional issues beyond the requested visual corrections

| ID | Additional finding | Evidence | Required outcome |
| --- | --- | --- | --- |
| I1 | Forward navigation is a dead affordance | `chrome::forward_button` is a HitTarget with no Tap handler; the model only exposes back history | Implement real forward history and disabled feedback, or show it honestly disabled until supported |
| I2 | View Options is not actually anchored | `options_layer` uses `left: 610`, `top: 46` | Anchor to the trigger's measured rect; flip/clamp inside the app frame on resize, inspector/sidebar changes and text scaling |
| I3 | Storage status presents invented measurements | Footer hardcodes `128.4 GB available` and progress `0.62` | Supply a real host storage capability, explicitly label fixture data, or omit the unsupported measurement |
| I4 | Files screenshot capture bypasses scheduled updates | `targets/snapshot::shot` only calls render; Mirui render does not advance systems | Tick/settle before capture, check layout/input-gate state, and record viewport, scale and state provenance; historical pictures cannot certify new code |
| I5 | Most icon-grid results are unreachable, not merely uneven | Normal icon mode caps at 32 items, search at 12; no continuation mechanism in grid composition | Bounded virtualization or explicit continuation/pagination; every reported result must be reachable, not silently truncated |

I5 is a functional collection-access defect, separate from R7's spacing and track
geometry. These five are not extra names for corners, fonts or header placement.

## 4. Exactly five proposed new additions

These are proposals, **not shipped features**. They must use real model actions,
Mirui interaction, truthful session/host capabilities and Ctrl conventions.

1. **Interactive breadcrumbs and a Ctrl L location field.** Provide clickable
   parent navigation and a keyboard-editable location for the in-memory hierarchy;
   validate the destination and preserve the current location on error. Do not
   pretend the fixture hierarchy is a mounted OS filesystem.
2. **Sortable, resizable list column headers.** Name/type/size/modified headers
   show actual sort state and support keyboard access. Resizing columns must not
   start a file drag; minimum widths preserve readable names and aligned metadata.
3. **Real range and multiselection.** Shift ranges, Ctrl toggles, visible selection
   counts and selection-aware commands. Background-click behavior, focus versus
   selection and what a batch operation targets must be explicit.
4. **Undo for destructive file actions.** Trash shows an actionable Undo message
   backed by a bounded reversible operation record. Restore identity, location and
   selection where still valid; report conflicts rather than pretending success.
5. **Space-to-preview without changing location.** A lightweight file preview
   with Escape-to-close and focus restoration, plus genuine next/previous
   navigation. Use available file/fixture content and explicit unsupported states;
   don't add a decorative preview that never reflects the selected item.

## 5. Implementation sequence and acceptance gate

1. Preserve current functions and give all controls stable identities. Establish
   host canvas versus application frame and the sidebar/content ownership split.
2. Fix measured grid geometry, count tracks, popup anchors and footer scope.
3. Introduce shared typography, outline action icons, shape and elevation roles.
   Compare specimens before rolling them through every component.
4. Implement/repair I1–I5 and run interaction, keyboard, clipping and collection-
   reachability tests. Add the five new capabilities as separately testable work,
   not as a cosmetic redesign that silently invents data or drops old actions.
5. Capture the same states before/after, at identical physical scale. Inspect
   every corner, count, icon, baseline and shadow at native size. Verify opaque
   panel interiors are not darkened by their own shadows. Check idle work and
   sustained interaction, including repeated sidebar/inspector toggling.

**No approval by assertion:** full application compilation/runtime captures remain
unverified in this sandbox (the existing build is blocked by missing dependencies).
The read-only probe ran successfully, but it is not a substitute for rendering.
Existing after-state implementation gaps in [TEXT_EDITING.md](TEXT_EDITING.md)
and the showcase status remain open.

### Pinned Mirui references used

- [Output consumers, OffscreenRender and budgets](https://github.com/W-Mai/mirui/blob/v0.51.0/docs/backend-fallback.md)
- [Sibling ID references and declaration order](https://github.com/W-Mai/mirui/blob/v0.51.0/docs/state-management.md)
- [Native DropShadow implementation](https://github.com/W-Mai/mirui/blob/v0.51.0/src/ui/widgets/drop_shadow.rs)
- [Circular corner geometry](https://github.com/W-Mai/mirui/blob/v0.51.0/src/render/backends/sw/quad_aa.rs)
- [Scoped path clips](https://github.com/W-Mai/mirui/blob/v0.51.0/src/ui/view.rs)
- [Font raster output ppem](https://github.com/W-Mai/mirui/blob/v0.51.0/src/render/font/mod.rs)
