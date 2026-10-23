# Evidence, source coverage and limitations

## Audit snapshot

Date: 2026-10-23. Branch: `arena/794cefdd-mirui-kit`.
Baseline commit: `7db20f35b59c9e7c4db2306b0c3fd2db413cc95e`.
Audit includes uncommitted changes from the preceding input-isolation turn.
Source line references describe this working tree and can move after later edits.
Hashes below are a reproducibility snapshot, not a claim of immutable line numbers.

## Coverage depth

- Detailed behavioral review: all `crates/fm/src` modules, including the entire
  model implementation, layout/controls/overlays, registration and synchronisation.
- Detailed host/runtime review: desktop event filtering/run loop/capture,
  snapshot state sequencing and bare allocator/critical-section setup.
- Verification review: test structure, assertions and the major feature journeys;
  tests were read, not executed. See report 05 for misleading coverage claims.
- Supporting-tool review: font bake/patch pipeline, icon converter transformation
  logic, timestamp naming, SDL linking and timing/entity-count diagnostics.
- Inventory only for generated vector coordinates and binary assets: their presence,
  sizes and call sites were checked; every path segment and MIRX payload was not
  independently decoded or visually certified.
- Upstream dependency internals and Naledzi kernel/IPC code are outside the checkout.
  No dependency or target behavior is certified from comments alone.

No subagent audit is claimed. These reports were produced by direct code review.

## Structural/arithmetic probes actually executed

Small Python probes read the repository and calculate these facts:

| Probe | Observed result | Interpretation |
|---|---|---|
| Palette command count vs constructed indices | Declared 17; constructed 0..15 | Final command has no row (F20) |
| Lexical size labels | `100 KB`, `12 KB`, `2 MB`, `9 KB` | String order is not byte order (F06) |
| Lexical date labels | `Oct 10`, `Oct 2`, `Oct 3` | String order is not chronology (F06) |
| Length-derived ID example | Remove base from base/base+1/base+2; next is base+2 | Collision path in F04 |
| Fractional output size | floor(11×1.5)=16; ceil=17 | Test comment/calculation disagree (A16) |
| Bare framebuffer arithmetic | 800×480×2 = 768,000 bytes | Exceeds 524,288-byte heap if fully heap-backed (A13) |
| Default font asset bytes | Regular+Medium+SemiBold = 5,631,568 | Static file budget, not a measurement of live heap |

These probes did not execute the Rust methods. The method-level findings are based
on the corresponding source paths and should receive regression tests.

### Contrast calculation

Calculated from checked-in opaque sRGB tokens using linearized relative luminance:

`channel = c/12.92` if `c <= 0.04045`, otherwise `((c+0.055)/1.055)^2.4`.

`L = 0.2126 R + 0.7152 G + 0.0722 B`.

`contrast = (L_lighter+0.05)/(L_darker+0.05)`.

| Role pair | Foreground | Background | Ratio |
|---|---|---|---|
| Dark filled-action text | #FFFFFF | #0A84FF | 3.647:1 |
| Light filled-action text | #FFFFFF | #007AFF | 4.017:1 |
| Dark selected sidebar text | #0A84FF | #1E3A5F | 3.154:1 |
| Light selected sidebar text | #007AFF | #D9E7FF | 3.218:1 |
| Light badge text | #6E6E73 | #C6C6C8 | 2.973:1 |
| Light tooltip text | #6E6E73 | #000000 | 4.141:1 |

The reference target is 4.5:1 for normal text; the relevant styles are 11/14px.
This is not a screenshot sampling result. Hover/press alpha layers, disabled state,
actual glyph rendering and backdrop compositing need separate rendered checks.
Source: `crates/fm/src/theme.rs` and the corresponding control text bindings.

## Saved-image observations

Images are historical artifacts with no verified build correspondence to current
code. In particular, saved menu shortcuts/icons and drag preview differ from
current source. Do not treat these images as the result of this audit's edits.

### `artifacts/desktop-window.png`

Host titlebar and internal traffic-light controls coexist.
The inner content area looks square/full-height rather than a separate rounded
floating app. Toolbar contains global command discovery beside content actions.
This supports reviewing host-decoration policy and whole-app preview composition;
it does not establish the exact latest executable geometry.

### `artifacts/02-light-icons.png`

Clean basic color grouping is a useful starting point. Filename and secondary
chrome text remain small; sidebar selected text relies heavily on blue color.
The status reads 32 of 512 items while the content shows four rows of eight.
The screenshot alone does not say whether additional rows can be reached; code
inspection establishes the rendering cap.

### `artifacts/04-list.png`

Rows are denser and use different text sizing from grid labels.
There are no visible column headers, so Kind and date values lack header context.
Status reports 400 of 400 even though code caps constructed rows at 48.
A visible scrollbar does not prove full reachability or draggable thumb behavior.

### `artifacts/07-context-menu.png`

The historical menu shows Cmd hints and lacks the icon column now in code.
Its surface boundary is subtle against content. Current input routing cannot be
inferred from this static picture. The prior IgnoreHitTest behavior is established
by earlier source and test changes, not by visual interpretation.

### `artifacts/08-dialog-new-folder.png`

The name editor is a shallow text strip with little visible internal clearance.
Generic OK/Cancel labels and small helper text weaken the action hierarchy.
A dark backdrop separates the modal, but static dimming does not prove a focus trap
or background input isolation. Current field-border code requires fresh rendering.

### `artifacts/10-command-palette.png`

One result sits beneath a narrow editor inside a much taller fixed panel.
The layout does not adapt its perceived density to a single result.
The historical Cmd N hint is not evidence of a working shortcut.
Keyboard selection/execution cannot be judged from this image.

### `artifacts/11-inspector.png`

A blank-looking three-section tab strip has a colored underline but no readable
labels. Grid tiles at the inspector edge appear clipped rather than cleanly
reflowed. The metadata and close target are small; storage repeats a fixed figure.
Code separately confirms missing app tab behavior and ignored inspector width.

### `artifacts/12-view-options.png`

Options overlap a highlighted file; controls and labels are small and dense.
The dark slider thumb has weak visual distinction from its track in this saved
state. The static picture cannot prove the reported slider/file drag conflict;
the event and model paths must be tested. Code confirms its ValueChanged no-op.

### `artifacts/13-drag-and-drop.png`

Historical preview is a blue filename pill, not the lifted source object.
Current code has moved to an icon card, but that still does not satisfy physical
item direct manipulation and has a captured-identity risk (F26/F27).

### `artifacts/14-notifications.png`

Cards form a coherent bottom-right stack, but text is tiny and body space is
limited. Long paths/errors and queue timing need actual stress fixtures.
Static raised surfaces do not establish appropriate error persistence or actions.

## Source inventory

The table records paths and SHA-256 prefixes of the inspected/inventoried source
snapshot. It is not a coverage percentage or proof that every branch executed.
Code findings provide symbol-level links; supporting files below enable follow-up.

| Source | Lines | SHA-256 prefix |
|---|---:|---|
| [`.gitignore`](../../.gitignore) | 6 | `e8dc79ddd4de2482` |
| [`Cargo.lock`](../../Cargo.lock) | 1415 | `e71218b07d4fd10a` |
| [`Cargo.toml`](../../Cargo.toml) | 15 | `27be2d542d992657` |
| [`Makefile`](../../Makefile) | 38 | `9fe7e3ca39beb20c` |
| [`README.md`](../../README.md) | 196 | `dffb5d55f58df0ae` |
| [`crates/fm/Cargo.toml`](../../crates/fm/Cargo.toml) | 42 | `addd6e36a2f91356` |
| [`crates/fm/examples/bench.rs`](../../crates/fm/examples/bench.rs) | 85 | `5ecfa9bf8164d598` |
| [`crates/fm/examples/fontinfo.rs`](../../crates/fm/examples/fontinfo.rs) | 37 | `856aaf88f813982e` |
| [`crates/fm/examples/leak.rs`](../../crates/fm/examples/leak.rs) | 54 | `2e4e3d3ca3c0060b` |
| [`crates/fm/examples/perf.rs`](../../crates/fm/examples/perf.rs) | 175 | `357fd69668d0b128` |
| [`crates/fm/examples/sbcheck.rs`](../../crates/fm/examples/sbcheck.rs) | 33 | `fa384a38bfc0186b` |
| [`crates/fm/src/data.rs`](../../crates/fm/src/data.rs) | 352 | `13d2f279e30ca482` |
| [`crates/fm/src/font.rs`](../../crates/fm/src/font.rs) | 151 | `32b0cfeb4299448e` |
| [`crates/fm/src/icons.rs`](../../crates/fm/src/icons.rs) | 113 | `fab37ff53e6bb9a9` |
| [`crates/fm/src/lib.rs`](../../crates/fm/src/lib.rs) | 52 | `5cdb17badcd6355b` |
| [`crates/fm/src/model.rs`](../../crates/fm/src/model.rs) | 1607 | `d1b69e6cb30f050a` |
| [`crates/fm/src/sync.rs`](../../crates/fm/src/sync.rs) | 220 | `e62c40bbaf9da9c9` |
| [`crates/fm/src/text.rs`](../../crates/fm/src/text.rs) | 21 | `45eab9dcf12dde92` |
| [`crates/fm/src/theme.rs`](../../crates/fm/src/theme.rs) | 141 | `6ef8d8aacdc974b2` |
| [`crates/fm/src/ui/chrome.rs`](../../crates/fm/src/ui/chrome.rs) | 349 | `104578bb6315aa08` |
| [`crates/fm/src/ui/content.rs`](../../crates/fm/src/ui/content.rs) | 283 | `9d3d2e90ddfffa1e` |
| [`crates/fm/src/ui/controls.rs`](../../crates/fm/src/ui/controls.rs) | 177 | `41bfc7cd02fb15eb` |
| [`crates/fm/src/ui/dialog.rs`](../../crates/fm/src/ui/dialog.rs) | 377 | `2b3a6c5b5a59e067` |
| [`crates/fm/src/ui/hover.rs`](../../crates/fm/src/ui/hover.rs) | 106 | `5d7e8b4a657d0be0` |
| [`crates/fm/src/ui/inspector.rs`](../../crates/fm/src/ui/inspector.rs) | 148 | `5824cb9808fb0a77` |
| [`crates/fm/src/ui/kindicon.rs`](../../crates/fm/src/ui/kindicon.rs) | 58 | `2e3989ab8de64bb2` |
| [`crates/fm/src/ui/kit.rs`](../../crates/fm/src/ui/kit.rs) | 246 | `6efc045eb293ebf5` |
| [`crates/fm/src/ui/menu.rs`](../../crates/fm/src/ui/menu.rs) | 291 | `05581b9d5af8f19d` |
| [`crates/fm/src/ui/mod.rs`](../../crates/fm/src/ui/mod.rs) | 77 | `60717efdec2e403e` |
| [`crates/fm/src/ui/options.rs`](../../crates/fm/src/ui/options.rs) | 172 | `e00dbe9c81f4efc2` |
| [`crates/fm/src/ui/palette.rs`](../../crates/fm/src/ui/palette.rs) | 173 | `63d5c50bc19fdfce` |
| [`crates/fm/src/ui/scrollbar.rs`](../../crates/fm/src/ui/scrollbar.rs) | 278 | `3d583bac54d00adc` |
| [`crates/fm/src/ui/sidebar.rs`](../../crates/fm/src/ui/sidebar.rs) | 101 | `7252035305db7f02` |
| [`crates/fm/src/ui/status.rs`](../../crates/fm/src/ui/status.rs) | 397 | `7dfb32ae7d651c4b` |
| [`crates/fm/tests/appearance.rs`](../../crates/fm/tests/appearance.rs) | 118 | `373ba0e0239f8471` |
| [`crates/fm/tests/data.rs`](../../crates/fm/tests/data.rs) | 215 | `dcaf6f334a6c1b8b` |
| [`crates/fm/tests/features.rs`](../../crates/fm/tests/features.rs) | 607 | `b38df50a8e1d45ed` |
| [`crates/fm/tests/interaction.rs`](../../crates/fm/tests/interaction.rs) | 326 | `1261ab79fc5f3dfb` |
| [`crates/fm/tests/typeface.rs`](../../crates/fm/tests/typeface.rs) | 334 | `ca3f5ffdba23d00f` |
| [`crates/sdl2-link/Cargo.toml`](../../crates/sdl2-link/Cargo.toml) | 10 | `19515673af89bb92` |
| [`crates/sdl2-link/build.rs`](../../crates/sdl2-link/build.rs) | 37 | `fc1f16b75a7e130a` |
| [`crates/sdl2-link/src/lib.rs`](../../crates/sdl2-link/src/lib.rs) | 12 | `30173118d60487b5` |
| [`rust-toolchain.toml`](../../rust-toolchain.toml) | 8 | `9874251acaf92ec1` |
| [`targets/bare/Cargo.toml`](../../targets/bare/Cargo.toml) | 14 | `5ee1b8b409804c6f` |
| [`targets/bare/src/main.rs`](../../targets/bare/src/main.rs) | 102 | `8bfcf8cd0de1f3f9` |
| [`targets/desktop/Cargo.toml`](../../targets/desktop/Cargo.toml) | 14 | `33526cd2d9c40366` |
| [`targets/desktop/src/main.rs`](../../targets/desktop/src/main.rs) | 338 | `de0de5814283bcaf` |
| [`targets/snapshot/Cargo.toml`](../../targets/snapshot/Cargo.toml) | 13 | `d9445df8e1092066` |
| [`targets/snapshot/src/main.rs`](../../targets/snapshot/src/main.rs) | 231 | `40d1a05f9d1f39a3` |
| [`targets/timestamp.rs`](../../targets/timestamp.rs) | 123 | `9bbcee3426e29e22` |
| [`tools/bake-fonts.sh`](../../tools/bake-fonts.sh) | 104 | `0c01cdb7c50c9cbd` |
| [`tools/patch-font-flags.py`](../../tools/patch-font-flags.py) | 74 | `db1b300ae5ee19fc` |
| [`tools/solar_to_mirui.py`](../../tools/solar_to_mirui.py) | 260 | `2e0c4cac3c93e983` |

## Assets and provenance limits

Nine MIRX weight assets and corresponding TTF sources exist under `assets/fonts`.
Inter license files exist at `assets/LICENSES/inter.txt` and
`assets/fonts/src/LICENSE.txt`. No separate Solar license/attribution bundle was
found under `assets/LICENSES` beyond the source comments/README references.
This is a packaging gap to review, not legal certification.

## What remains to verify

- Compilation, lint, all test suites and no_std target builds.
- Mirui walk retention/reactive dependency/event dispatch semantics.
- Live desktop focus, input cancellation, DPI and screenshot behavior.
- Font shaping correctness and actual output quality after source flag patching.
- Memory allocation strategy of the FramebufSurface dependency.
- Reported long-running freeze, memory growth and compositor responsiveness.
- Accessibility bridge behavior and actual Naledzi OS integration.

A future fix should append an execution record with finding IDs and artifacts,
not silently change “unverified” to “done” because a handler or report exists.
