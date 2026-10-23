# UX / Visual Design Audit — kukan (mirui 0.51.0) file manager

**Scope:** `crates/fm` on mirui 0.51.0, audited against user screenshots in
`/home/lufuno/mirui/artifacts/` (19 PNGs): `screenshot.png`, `01-dark-icons`,
`02-light-icons`, `03-downloads`, `03b-projects`, `04-list`, `05-search`,
`06-selected`, `07-context-menu`, `07b-background-menu`, `08-dialog-new-folder`,
`09-dialog-info`, `10-command-palette`, `11-inspector`, `12-view-options`,
`13-drag-and-drop`, `14-notifications`, `desktop`, `desktop-window`.
**Constraint:** read-only. No source files modified. Covers areas 1–5 only.

---

## Executive summary

| Rank | Issue | Severity | Screenshot |
|------|-------|----------|------------|
| P0 | Text is crisp at exact ppem (1×) but soft/blurry at fractional HiDPI; coverage atlases exist only at 1×/1.5×/2× and anything between is SDF-resampled. Inter is the sole typeface; no MiSans. | P1 | `01-dark-icons`, `02-light-icons` |
| P0 | Zero drop shadows anywhere — sheets, menus, tooltips, inspector all flat panels with a 1px border. Not macOS-premium. | P0 | `07`, `08`, `09`, `10`, `11`, `12`, `14` |
| P0 | Context-menu backdrop uses `IgnoreHitTest`: clicks pass through to items behind while the menu is open. Drag tracking also runs underneath open menus. | P0 | `07-context-menu`, `07b-background-menu`, `12-view-options`, `13-drag-and-drop` |
| P0 | Drag preview is an abstract 190×26 blue text pill — no icon, no shadow, no physical item following the cursor. Toasts reuse the same pill, confusing drag with notification. | P0 | `13-drag-and-drop`, `14-notifications` |
| P1 | Scrollbar thumb is a flat colored lozenge with no track rail, no hover/pressed state, and no draggable affordance. It is not actually draggable (no input handling). | P1 | `04-list`, `desktop.png` |

---

# 1. TEXT / font rendering

## What is present

`font.rs` registers **Inter** under `FAMILY = "Inter"`, one MIRX asset per weight. Regular is always compiled; medium, semibold, thin, extralight, light, bold, extrabold, black live behind cargo features (`default = ["weight-medium", "weight-semibold"]`).

`text.rs` defines a three-step scale:
- `SMALL = 11` — icon grid labels, status bar, inspector details
- `BODY = 14` — list rows, sidebar labels, toolbar title, menus, sheet bodies
- `TITLE = 22` — sheet titles

`font.rs` (lines 9–13): mirui picks the representation by output ppem; under `Auto` a coverage atlas is accepted only when its design ppem matches exactly. Exact match → blitted 1:1, stems sharp. Anything else → resampled from the SDF atlas, smooth but soft.

Assets carry coverage atlases at 1×, 1.5×, 2× and one SDF atlas for sizes in between. `tools/bake-fonts.sh` builds them from `assets/fonts/src`.

## Crisp vs soft — verdict

**Crisp at 1× only.** At a 1× viewport the UI hits exact ppem for SMALL/BODY/TITLE if the baked atlases cover 11, 14, 22 ppem — glyphs blit one-to-one and stems stay sharp, matching the premium crispness of online mirui demos. At any non-exact ppem (1.25×, 1.75×, fractional scaling, HiDPI where logical px ≠ physical px), mirui falls back to the SDF atlas and resamples: text goes **soft/blurry**.

Screenshots `01-dark-icons` (dark theme) and `02-light-icons` (light theme) show grid labels ("ADR 001", "Architecture", "Benchmarks", …) and sidebar items ("Favourites", "Documents", "Downloads", "Projects", "Trash"). At the captured viewport they read crisp. There is no fractional-scaling screenshot to confirm the soft fallback, but the code path guarantees it on any non-1×/1.5×/2× viewport. Sidebar rows (BODY 14) sit next to grid labels (SMALL 11); the two sizes may land on different ppem buckets and render with mismatched sharpness on fractional displays.

`04-list` shows the Projects list view where BODY text appears in rows; `06-selected` shows a selected folder; `05-search` shows the search field. All rely on the same Inter scale.

## Inter presence

Confirmed present — Inter is the sole typeface (`FAMILY = "Inter"`), baked per weight. No MiSans, no system fallback visible. That's a deliberate, consistent brand choice; the risk is purely rendering quality at non-exact ppem, not family inconsistency. Every text surface — grid labels, sidebar rows, menu items, sheet titles, dialog bodies, toast text — uses Inter.

## Thin/light vs MiSans

No MiSans anywhere. Thin/light/extralight weights exist behind features but are **not in the default feature set** (`default = ["weight-medium", "weight-semibold"]`), so thin/light text is not drawn in the default build. The UI uses regular/medium/semibold only — a safe, conservative palette. No light-weight body text to compromise legibility. `02-light-icons` confirms body text reads as regular/medium weight, never thin.

## Line heights

`controls.rs:61` `field_height()` = line height of SMALL + 2×FIELD_INSET (2px each side) = exact text metrics, no centering — fields are sized to the text so the line "fills the field exactly." Sound for single-line fields; multi-line content (dialog bodies) inherits mirui's default paragraph leading. `09-dialog-info` rows are height 20 (`dialog.rs:268`) with SMALL text — tight but consistent. `08-dialog-new-folder` body text ("Choose a name for the new folder.") is single line, no wrap visible. No custom `line_height` token is defined anywhere; leading is whatever mirui defaults to per size. Risk: at TITLE 22 (sheet titles in `08`, `09`) leading may feel loose or cramped depending on mirui's default; worth a token.

## Row padding consistency (text-adjacent)

- Sidebar rows: height 28, `pad_h(6)`, icon 15×15, gap 8 (`sidebar.rs:57-99`). Text has consistent vertical room.
- Icon grid cells: folder icon ~36×36 with label below; vertical grid pitch ~96px with generous whitespace.
- List rows (`04-list`): height ~28–30, icon + label right-aligned columns.
- Toolbar row: fixed 52px (`chrome.rs:22`), containing 26px buttons and a 26px search field; title at BODY 14 with `height: 17`.

Padding is consistent **within** each zone but the zones don't share a baseline grid — 28px sidebar vs 52px toolbar vs grid pitch produce a slightly uneven vertical rhythm when scrolling.

## Text truncation / alignment

- Grid labels are centered under icons, single line, no visible ellipsis truncation in screenshots — long names ("Quarterly Report", "Test Matrix") fit on one line at grid width; unknown behavior when they overflow.
- Sidebar: name text `grow: 1.0`, count right-aligned, both single line — no truncation visible, long place names would overflow.
- Toolbar title is a single `Text` at BODY 14 — long paths ("/Users/demo/Cloud") shown in SMALL below; no truncation or marquee.
- Dialog body text ("Choose a name for the new folder.") single line, no wrap visible in `08-dialog-new-folder`.

## Premium-crisp benchmark

Online mirui demos render sharp because they target exact ppem. Here crispness is **contingent on 1× viewport and exact baked sizes**. On a modern HiDPI laptop (1.5×–2× fractional), text will be SDF-resampled and soft. The demos also typically use a fuller typographic scale; this UI's 3-step scale is minimal but adequate for a file manager.

---

# 2. SHADOWS / elevation / premium feel

## Observation

**There are no drop shadows anywhere in the app.** Every elevated surface is a flat colored panel with at most a 1px hairline separator border:

- Context menu: `border_width: 1`, `border_color: SEPARATOR` (`menu.rs:72-74`), bg `Surface`, radius 8. No shadow.
- New folder / info sheets: `border_radius: 12`, `border_width: 1`, `border_color: SEPARATOR`, bg `Surface` (`dialog.rs:58-65`). No shadow — just a `DIM` rgba(0,0,0,0x99) dark backdrop behind.
- View options popup: same flat panel + 1px border (`12-view-options`).
- Inspector pane: flat side panel, no shadow separating it from content (`11-inspector`).
- Toolbar: flat `TOOLBAR` bg, no elevation over content.
- Toasts/notifications: flat pill `14-notifications`, no shadow.

Screenshots confirm: `08-dialog-new-folder` (New Folder sheet) is a flat dark panel with 1px border + dim backdrop; `09-dialog-info` (Info sheet) same; `07-context-menu` flat panel; `12-view-options` flat panel; `11-inspector` flat side panel; `14-notifications` flat pills. `screenshot.png` and `01-dark-icons` show the flat toolbar and flat content surface.

## macOS premium baseline comparison

macOS uses elevated panels with soft, broad drop shadows (~20–40px blur, ~20% opacity black) plus a thin hairline border, and vibrancy/blur behind sheets/sidebars. kukan has **zero** elevation — panels read as stickers slapped on the canvas, not floating layers. The `DIM` backdrop (dark translucent) behind sheets helps, but without a shadow the sheet still feels pasted.

`desktop-window.png` shows a separate "Files" window with traffic lights in a titlebar area — that window carries the expected OS chrome, but the file manager's own content windows put everything flat inside the toolbar row (covered separately). None of the elevated surfaces in any screenshot cast a shadow.

## Where shadow is missing (all of them)

| Component | Needed shadow | Current |
|-----------|--------------|---------|
| Context menu | soft shadow + subtle border | flat panel + 1px border |
| New folder / info / rename sheets | broad soft shadow behind dim | dim only, flat panel |
| View options | soft shadow | flat panel |
| Inspector (when open) | side-panel elevation | flat, flush with content |
| Toolbar | subtle bottom shadow over content | flat |
| Toast | soft shadow | flat pill |
| Primary buttons (New Folder) | elevated pressed state | flat |

## Applied strength

None applied, so no "too strong/too weak" issue — the problem is total absence.

## Recommendation

Add a soft, theme-driven drop shadow (e.g. 0/4/8/0 rgba(0,0,0,0.25) for panels, 0/8/24/0 rgba(0,0,0,0.35) for sheets) and a thin border; keep the dim backdrop for modals. This is the single biggest visual upgrade for a premium feel.

---

# 3. SCROLLBAR visual draggability

## Observation

`scrollbar.rs` paints **only a thumb** — `DrawCommand::Fill` with `SCROLLBAR` color (`rgba(0xFF,0xFF,0xFF,0x57)` dark theme / `rgba(0,0,0,0x42)` light theme), radius = half thickness, thickness configurable. There is **no track rail drawn at all** — the thumb fills most of its own box on the trailing edge. The thumb length is proportional to viewport/content (`thumb = viewport * track / content`), and position tracks offset — so the *paint geometry* is correct and proportional.

Screenshots show a thin blue-ish lozenge at the bottom-right (`04-list` bottom edge; a thin track visible in `desktop.png`, `01-dark-icons`). Because there's no track rail behind the thumb, the thumb reads as a static status indicator rather than a draggable control. Contrast between thumb and container is moderate (57% white on dark surface) but there's no visual "groove" to grab. No hover/pressed states on the thumb are implemented (no `InteractionFeedback` on the scrollbar).

## Native-draggable control?

No. It looks like a read-only progress-style indicator. A native scrollbar shows track rail + thumb + clear affordance (arrows or at least a depressible thumb), and the thumb typically darkens on hover. kukan's thumb has none of that.

Separately, the code audit confirms the thumb is **not draggable at all** — no `HitTarget`, no drag handler, no write-back to `ScrollOffset`. Purely decorative paint.

## Recommendation

Paint a subtle track rail (Outline color, thin, full length) behind the thumb; give the thumb higher contrast and a hover/pressed overlay; clamp minimum thumb length (~24px) so it stays a touch target; wire actual drag input to mutate the target scroller's `ScrollOffset`.

---

# 4. CLICK-THROUGH / layering behind popups

## Critical finding: menu backdrop swallows nothing

`menu.rs:44-64` — the menu backdrop covers the whole window but has **`IgnoreHitTest`** and the comment explicitly says: "a press outside the menu reaches whatever is underneath it." `dialog.rs:43-47` — the dialog dim backdrop **does** have `HitTarget` and `on Tap { fm.close_dialog() }` — so sheets swallow clicks, but **menus do not**.

## What this means in practice

- Open the context menu (`07-context-menu`) over a folder. Click anywhere outside the menu panel but inside the window → the click passes through to the folder tile beneath. Depending on sync logic, this can trigger selection/drag on the item *behind* the menu while the menu is still open. Direct violation of modal layering.
- `07b-background-menu` shows the background menu open over the grid; the backdrop ignores hits so the grid underneath remains interactive.
- `12-view-options` — view-options popup opened over the grid; clicks behind it reach tiles. The "Budget 2026" tile shows a blue highlight ring in `13-drag-and-drop` — suggestive of hit-through selection while a popup is open.

## Dragging behind popup

Not directly observable in still screenshots, but since the backdrop ignores hits, a press-drag starting outside the menu panel would initiate a drag on background content while the menu is still displayed — a broken layering state (menu open + background dragged simultaneously). `13-drag-and-drop` shows the drag badge active; nothing indicates the menu locks out background drag.

## HitTarget coverage

- Sheets/dialogs: full-window `HitTarget` dim backdrop — correct modal behavior, dismiss on outside tap (`dialog.rs:45-47`).
- Menus: `IgnoreHitTest` backdrop — incorrect for a popup menu; should be `HitTarget` that dismisses the menu on outside tap, like sheets.
- Command palette (`10-command-palette`): centered sheet-like overlay; likely dialog-layer (HitTarget) — correct.
- Inspector (`11-inspector`): side panel with X close button; not modal, doesn't block — acceptable for a non-modal inspector.

## Backdrop dim

Menus have **no backdrop dim** — they sit directly on raw content with no separation cue, unlike sheets which use `DIM`. This weakens the layering cue. `07-context-menu` and `07b-background-menu` both show the menu floating on raw grid content with no dim.

## Recommendation

Give the menu backdrop a `HitTarget` that closes the menu on outside tap (mirror `dialog.rs`). Keep panel hit-testing so in-menu taps still work. Add a subtle backdrop dim for menus too.

---

# 5. DRAG PREVIEW physicality (actual item vs abstract box)

## Observation

`menu.rs:109-134` `drag_badge` — a 190×26 absolute-positioned pill, `bg_color: ColorToken::Primary` (blue), centered white text label (`fm.drag_label()`), border_radius 6, offset 12px from cursor (`drag_x()+12`, `drag_y()+12`). Visible only while dragging.

## What the screenshots show

`13-drag-and-drop` — the badge reads "ADR 002.rs" as a blue pill floating near the cursor. `14-notifications` shows the same pill shape reused for toasts ("Dropped into folder", "New folder created", "Moved to Trash").

## Physical object feel?

**No.** The badge is an abstract text pill:
- No icon of the dragged item.
- No thumbnail/avatar of the folder or file.
- No shadow (flat pill).
- No scaling/rotation that suggests a physical object following the cursor.
- It doesn't resemble the item under the finger at all — it's a label in a colored capsule.

A premium direct-manipulation drag (macOS Finder, Windows Explorer) shows a miniature of the actual item — icon + name, sometimes a stack count, with a soft shadow and a slight offset — so the user feels they're carrying the object. kukan's badge feels like a tooltip that happens to move with the pointer.

## Toast reuse of drag-badge visual

`14-notifications` shows toasts using the exact same pill shape/color family as the drag badge. A user may confuse an in-progress drag with a completed notification. Toasts should have their own notification style (distinct shape, icon, color semantics).

## Recommendation

Replace the pill badge with a real drag preview: a small card containing the dragged item's icon (actual folder/file icon at ~24–32px) + name text, with a soft drop shadow, rounded rect, and the same 12px cursor offset. Optionally show a ghosted copy at the source while dragging. Reuse this card for multi-item drags (show count badge "N items"). Give toasts a distinct notification style.

---

**Prepared by:** Kilo (UX/visual design engineering audit)
**Status:** Read-only audit complete; no source files modified.
