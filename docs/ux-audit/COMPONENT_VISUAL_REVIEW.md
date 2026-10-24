# Native component visual review — 2026-10-24

Evidence: commit `5928ca1`, successful CI run
[40097469960](https://github.com/smart-foundation/mirui-kit/actions/runs/40097469960).
All 288 isolated state/theme/scale/width captures rendered successfully. Fifteen
exact native representative PNGs were recovered from Checks and inspected,
covering all 13 current isolated families, plus light large-text button/field
samples. **This is not a claim that all 288 states were visually inspected.**

The new suite complements the existing six-route/overlay captures and native
interaction contracts. No baseline was automatically approved.

## Exactly five additional screenshot-visible findings

| Finding | Native evidence | Source diagnosis | Required action |
| --- | --- | --- | --- |
| Numeric field and step buttons have different bottom edges. | `numeric-entry--rest--dark--100--960.png`: field is 44px, adjacent buttons 40px. | `number_step` hardcodes 40px while the field scales from 44px. | Share scaled 44px geometry, including icons; assert native rectangles at both text scales. |
| Listbox entries read like centred action buttons rather than a desktop collection. | `listbox--rest--dark--150--960.png`, versus the correctly left-aligned tree. | Native Button Regular size overrides supplied padding/alignment. | Use native Custom sizing, centre the cross axis, and left-align the main axis; retain selection/focus/disabled behavior. |
| An inactive switch has a square enclosing outline around its pill. | `switch--rest--dark--150--640.png`. | Widget explicitly adds a rectangular border while the native switch paints a capsule. | Match the border/focus radius to its 26px height. Do not replace the native switch. |
| A short badge expands into a full-width grey strip. | `feedback--rest--light--150--640.png`: “Session only” stretches across the complete column. | Default auto-width stretches in a Column. | Request content width; verify that native measured width remains smaller than its parent. |
| Slider paint is disproportionately heavy for compact desktop controls. | `slider--rest--dark--150--640.png`: 36px capsule rail/knob next to 40px step buttons. | Mirui 0.51 native slider renderer uses the whole input rectangle for both track and knob. | Still open: separate native paint geometry from accessible hit geometry upstream or through a supported native extension; do not silently shrink the pointer target or fake a new renderer. |

These findings demonstrate why successful compilation and nonzero geometry are
insufficient for visual acceptance. Text rendering itself is crisp in these
samples; there is no evidence here to justify blindly replacing the font.
Labels/help wrap without clipping in the inspected field/feedback cases. Tree
indent/disclosure and table column edges are coherent in the inspected resting
samples. Calendar reserves a stable six-week body; the empty sixth week in a
five-week month is not automatically wasted-space failure.

## Still not accepted as complete

No full-core parity verdict has changed. Smart Protocol transport, actual host
window controls, all-kit isolated coverage, complete state-by-state visual
review, nested-overlay/drag/scrollbar ownership, semantic accessibility and
suite-scale performance remain separate completion gates. The sidebar-aligned
caption-cap recommendation still stands; these component images do not validate
host window behavior.
