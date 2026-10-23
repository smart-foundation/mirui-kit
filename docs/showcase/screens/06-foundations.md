# S06 — Foundations: one policy, every component

**Route:** `/foundations` · **Primary task:** preview and apply readable appearance.  
**Demonstration:** typography, color, density, icons, elevation and motion in one
real mini recipe. The app itself consumes the same tokens and components.

## Default wireframe

```text
+------------------------------------------------------------------------+
| Studio      [Find components…                     ] [Commands] [Help]   |
+---------------+--------------------------------------------------------+
|   Overview    | Appearance                                  [Inspect]  |
|   Controls    | Preview changes before applying them to Studio.        |
|   Collections |                                                        |
|   Workbench   | Theme       [System | Light | Dark]                     |
|   Surfaces    | Text scale  [100%____________]                          |
| > Foundations | Density     [Compact | Comfortable]                    |
|               | [x] Follow host reduced-motion preference              |
|               |                                                        |
|               | Preview only                                           |
|               | +----------------------------------------------------+ |
|               | | Interface settings                                | |
|               | | Display name [Studio________________]              | |
|               | | [x] Keep examples local       [Secondary] [Primary] | |
|               | | Body · helper text · selected row · focus example   | |
|               | +----------------------------------------------------+ |
|               | [Restore defaults]               [Apply to Studio]     |
+---------------+--------------------------------------------------------+
| Preview differs from Studio · Not yet applied                [Status]   |
+------------------------------------------------------------------------+
```

Theme System is available only if host preference events exist; otherwise explain
that host following is unavailable and allow explicit light/dark selection.
Text scale is validated against supported constraints; not an arbitrary slider
that can make all controls disappear. Start with 100/125/150/200% choices.
Density changes hit targets and layout coherently, not just font size.

## Preview versus applied state

All controls edit an appearance draft. Preview components resolve that draft in a
scoped policy context. The surrounding app stays unchanged until Apply.
Applied settings form a versioned snapshot that can be rolled back.

```text
| Appearance applied. Keep these settings?                       |
| Reverting in 15 seconds unless confirmed. [Revert] [Keep]       |
```

This recovery banner is a high-priority app surface with clearly visible keyboard
path. Timer uses host monotonic clock; suspend/resume cannot leave a broken policy
permanently applied. Losing timer capability prevents timed preview and uses an
explicit non-timed confirmation with a guaranteed reset shortcut instead.
Do not allow an appearance test to destroy app accessibility.

## Role inspection, not a swatch wall

Expanded details show role name, resolved value, intended use and measured contrast
pairs. Pair status is computed, with threshold/assumptions visible; it is not a green
“accessible” badge based only on one foreground color.

```text
| Role             Use                   Pair contrast          |
| TextPrimary      Body on Surface       measured  … : 1        |
| OnAccent         Primary action        measured  … : 1        |
| OnSelection      Selected item text    measured  … : 1        |
| Focus            Focus boundary        rendered review needed |
```

No fabricated ratios in the implementation. Actual palette values must first be
corrected; the current Files audit found several failing small-text pairs.
Links open the role’s actual definition/recipe in O02, not an unrelated CSS example.

## Type and icon detail

Type specimens use meaningful short sentences, numerals, punctuation and filename
samples, not only “Aa”. Include long/combining/RTL examples under Advanced.
Glyph coverage/support is explicit. Optional weights are shown only if available.
Choosing a weight for Body updates the semantic role, not every widget one by one.
Icons have searchable names, semantic uses and consistent optical-size previews;
copying an icon recipe includes its accessible-label requirements.

## Elevation and motion detail

Show real control, popover, dialog and window surfaces through shared components.
Do not fake the entire hierarchy by drawing four rectangles with baked shadows.
Motion preview can pause/stop. Reduced motion applies to both specimen animation
and app-level transitions. A loading specimen explains its pending state and stops
when hidden. Surface effects have a measured cost note only when actually measured.

## Unsupported/error state

Font or asset unavailable: keep last valid policy and name the missing capability.
Invalid text-scale input: local error, no global mutation.
Appearance apply failure: revert to the last usable policy and show persistent
feedback. Restore defaults affects appearance only, not documents or collections.
A full example reset remains O12 with a distinct scope.

## Compact layout

Controls and preview stack. The preview shares supported text scale and responsive
layout; it may scroll internally only when that is the specimen’s real behavior.
Do not scale a screenshot down and call it responsive.
The recovery banner remains visible without covering confirmation actions.

## Acceptance

- Studio and specimens use the same semantic policy resolver.
- Draft changes stay scoped until Apply; Revert restores the exact prior snapshot.
- All tested themes/text scales preserve keyboard access and readable controls.
- Contrast/support evidence is computed and labelled, not invented.
- Reduced motion stops ongoing animations and avoids hidden background repaint loops.
