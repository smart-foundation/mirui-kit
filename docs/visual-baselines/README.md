# Native visual baseline approval

No images in this directory have been approved yet. Do not turn a generated
capture into a baseline just to make a build green.

`component-snapshot` now captures 13 shared component families and meaningful
states in isolation, without relying on their position below a route's scroll
fold. CI's component-review matrix covers dark/light × 100/150% text ×
640/960 logical-pixel viewport widths. Each shard writes native RGBA PNGs and an
`index.json` containing component, state, viewport, theme, scale and measured
bounds. Button hover/pressed/focused captures use native input events; disabled,
pending, validation, range, expanded, selected and empty fixtures prepare actual
models. Prepared states do not prove that a user interaction can reach them.

## Review and acceptance

1. Run `cargo run --locked -p studio-host --bin component-snapshot -- --out
   artifacts/studio/components --theme dark --scale 150 --width 640`.
2. Inspect every image at actual size. Check text/glyphs, clipping, alignment,
   whitespace, targets, state distinction, borders, corner/shadow silhouettes,
   theme contrast and long content. Check the app route/overlay captures too.
3. Exercise keyboard and pointer workflows: Tab order, arrows, selection versus
   focus, validation recovery, drag ownership, modal blocking and focus return.
   Static images cannot establish these properties.
4. Only after review, copy approved exact PNGs here under their generated names.
   Record source commit, renderer/toolchain, reviewer and approved cases in an
   approval record. No tool here performs automatic approval.
5. Run `python3 tools/compare-native-captures.py artifacts/studio/components
   --baseline docs/visual-baselines --out artifacts/studio/component-differences`.

The comparator validates PNG CRCs/filter reconstruction, checks dimensions,
compares RGBA pixels, and emits difference images, changed-pixel counts and a
bounding box. Defaults are exact: no hidden tolerances or automatic baseline
rewrites. Missing baselines fail by default. CI bootstrapping explicitly uses
`--allow-missing`: those cases report **needs-review**, not visual acceptance.
Existing baseline regressions still fail. Any tolerance change needs deliberate
review because it can hide precisely the one-pixel bugs we care about.

## Coverage limitations, deliberately not hidden

This first isolated suite is not all-kit coverage. It covers button, labelled
field, integer entry, checkbox, switch, slider, choice group, listbox, tree,
table, calendar, split and grouped feedback specimens. Icon-button variants,
standalone chrome/layout, editing-selection surfaces, all nested overlays,
scrollbars, typography, semantic accessibility, reduced motion, localization,
RTL, pointer drags and specialized editor/chart/docking families still need
explicit specimens and/or event-driven contracts. Add each new component's
states here as part of implementing it; a catalog name is not evidence.

Do not compare screenshots rendered by different fonts/toolchains/viewport
scales without explaining the environment change. Do not use wide tolerances
as a substitute for deterministic rendering. Pixel comparison catches changes;
human review establishes whether a change is an improvement or a regression.

## Restricted-network review

A successful `[capture-review]` run exports resting representatives as lossless
Checks annotations in addition to uploading all 288 matrix PNGs as artifacts.
Recover and view the representative evidence without an artifact-host redirect:

```sh
python3 tools/recover-component-evidence.py RUN_NUMBER
python3 tools/component-review-gallery.py artifacts/studio/component-review
```

`evidence.json` records the source commit, run/check IDs, exact PNG SHA-256 and
viewport. `review.html` embeds those unchanged PNGs with provenance and an actual
pixel-size toggle. The recovery command rejects failed runs, incomplete chunks,
corrupt PNGs and duplicate chunks. This representative export is not a transport
for every state; the complete matrix remains in the run artifacts.
