# Studio source implementation and verification

This is **an initial source implementation, not release acceptance of the entire
showcase specification or a claim that every Files bug is fixed**. The six route
compositions exist, along with a deterministic workflow core, shared kit controls,
hosts, tests and CI. The design documents remain the acceptance contract; gaps are
listed explicitly below.

## Source map

| Path | Responsibility |
|---|---|
| `crates/kit` (`mirui-kit`) | Shared assets, semantic type sizes, theme, action/field/slider/switch compositions, entity focus ring, bounded action channel, native input gate, focus traversal, draggable scrollbar |
| `crates/studio-core` | Dependency-free `no_std + alloc` session workflows; six routes and 17 surface identifiers |
| `crates/studio` | Mirui `#[model]`, `#[compose]`, callback, timer and system integration; actual route compositions and command/component registries |
| `targets/studio` (`studio-host`) | SDL desktop and headless RGBA/PNG hosts |
| `crates/fm` | Existing Files app, retained; font/theme/icon/scrollbar modules now re-export shared kit implementations |
| `tools/check-mirui-syntax.py` | Reproducible checks using the **actual** pinned Mirui/xrune macro implementations, not a made-up grammar |
| `crates/studio/tests/flows.rs` | Mirui integration/render/input-gate regressions, awaiting a dependency-enabled build |
| `.github/workflows/studio.yml` | Full workspace tests, RISC-V no_std check, SDL compilation and fresh captures; added, **not run here** |

The shared crate has no dependency on Files or Studio. Existing `fm::font`,
`fm::theme`, `fm::icons` and `fm::ui::scrollbar` import paths remain compatible.
All nine static Inter weights retain their feature switches; `weights-all` opts
into all atlases. There is no claim of variable-font support.

## Running

Use the repository's pinned Rust toolchain. Linux desktop builds also need SDL2
and Wayland development libraries (`libsdl2-dev libwayland-dev pkg-config` on
Ubuntu). The existing workspace desktop member enables Mirui's SDL feature during
whole-workspace tests, so those tests need native libraries too.

```sh
# Interactive real Mirui app: 1180×720 floating app in a 1400×900 stage
cargo run --locked -p studio-host --features desktop --bin studio-desktop

# Fresh App per fixture: 6 routes + 17 entry flows + light appearance
cargo run --locked -p studio-host --bin studio-snapshot -- --out artifacts/studio

# Full compilation, old Files regressions and new integration tests
cargo test --locked --workspace
cargo check --locked -p studio --no-default-features --target riscv64gc-unknown-none-elf

# Runs without Cargo registry access; uses the exact production core source
sh tools/test-studio-core.sh

# GitHub sources + local compiler; cache and outputs are ignored
python3 tools/check-mirui-syntax.py
```

Snapshot output is ignored, not committed as generated bulk. Generation asserts
that each requested surface was actually entered. A generated capture still
requires visual review; its existence does not mean the layout passed acceptance.

## What the source does

- Overview navigates into real tasks and the searchable shared-component catalog.
- Controls binds native Mirui documents, Slider and Switch to a validated preset.
  Preview is explicitly a timer-stepped **fixture summary**, not an encoder.
  Editing cancels the old job, stale completion is rejected, and restart is explicit.
  Presets save only to memory; duplicate names preserve the previous preset.
- Collections uses stable identities, hierarchical folders, filtering, folders-first
  name sorting, inline creation/rename, asset inspection and validated folder moves.
  Cycles and conflicts are checked before mutation. Draft cancellation removes only
  the draft, and selecting another asset cancels naming safely.
- Workbench edits a real Mirui TextDocument and saves to the session. Save/Discard/
  Cancel protects dirty departure. It is not a code editor; native single-line
  editing semantics remain, with an explicit Append paragraph action.
- Surfaces provides the 17 identifiers and task-owned entry paths, including
  conflict, confirmation, unsaved-note and appearance recovery states. Cancellation
  does not apply a review, stale reviews fail, and a review cannot commit twice.
- Foundations provides a scoped appearance specimen and global Apply / Keep /
  Revert with ten-second recovery. Typography uses semantic kit sizes and an
  explicitly observed text-scale value, independently of viewport DPI.
- Commands have real typed Solar icon paths, availability/filtering, empty results,
  Up/Down selection and field submission. Tab/Shift Tab operates within the active
  layer; Escape dismisses/reverts. Ctrl P opens commands and Ctrl S saves notes.
- Background input uses native Mirui **UserState::Disabled propagation**, not just
  ignored app clicks. The backdrop is a hit target; stale queued background actions
  are rejected after a layer transition. Focus rings use the actual entity, so two
  buttons sharing a command ID do not both acquire keyboard focus feedback.
- Work is bounded: 32 queued actions, 128 fixture assets, 32 saved presets, three
  notices, 64-character names and an 8192-byte note document. Idle timer ticks do
  not publish UI updates. SDL installs the native clock and tick pacing plugin.
- Shared primary/on-primary, error, badge and tooltip color pairs were adjusted
  for readable text. This is not a blanket WCAG-conformance claim.

## Verification actually performed in this environment

| Check | Result |
|---|---|
| Dependency-free Rust tests | **33 passed**, zero failed |
| Production core compiled as no_std library | **Passed**, native x86_64 target |
| Model/component/compose/view/event/system/timer macro expansion plus UI parsing/lowering | **Passed**, upstream Mirui `v0.51.0`, xrune `v1.10.1`, exact tagged macro dependencies |
| New Rust source parsing/formatting | Passed using Rust parser/rustfmt and xrune formatter; layout-binding DSL retained where its formatter cannot handle it |
| Cargo workspace manifest/lock metadata | Passed (`cargo metadata --locked --no-deps`) |
| Full Cargo compilation / integration tests | **Blocked**, crates.io index TLS access denied; offline cache lacks Mirui/mirx |
| SDL execution / new screenshots / visual review | **Not performed** |
| RISC-V target compilation | **Not performed**, CI configured |
| All audit findings / all acceptance scenarios | **Not completed** |

The local compiler is Rust `1.92.0-nightly` from integrity-checked npm-packaged
Rust components. It was sufficient for the core and upstream macro checks; it is
not represented as execution of the repository's pinned `nightly-2026-07-24`.
The full build attempt failed fetching `https://index.crates.io/config.json`
with an SSL unexpected-EOF error. No dependency was stubbed to produce a green
build, and historical PNGs were not relabelled as current evidence.

Macro validation catches actual DSL/model errors and has already caught and
prevented invalid event/child ordering, namespaced positional argument parsing,
unsupported namespaced widget heads, borrowed model returns and reactive children
without a widget parent. **It does not type-check generated application code or
prove that any rendered UI is correct.**

## Remaining acceptance gaps — do not hide these behind “complete”

1. Full native and no_std target builds must pass, with all integration regressions
   executed and any compilation/runtime errors fixed. The new CI has not been run.
2. All 17 states currently share a blocking sheet presentation. The specified
   anchored menus/options, true hover tooltip, modeless inspector and compact
   navigation drawer need differentiated placement, lifecycle and interaction.
3. Collections does not yet implement direct visual dragging, trash/undo,
   multi-selection, list/grid switching or the full component catalog coverage.
   Its Move operation is an explicit validated destination picker.
4. The 1180×720 stage is intentionally fixed. Responsive 800–1099 layouts, smaller
   host handling and every 150%-text reflow/clipping scenario need implementation
   and visual acceptance. Appearance preview is a scoped specimen, not an entire
   nested themed copy of the running app.
5. Accessibility/platform semantics export, arrow/roving patterns beyond the
   palette, scroll-into-view for keyboard focus, complete labels/announcements,
   every disabled/error state and full contrast coverage remain open.
6. No OS persistence, filesystem bridge, export encoder or verified bare-metal
   allocator/font-memory configuration is included. Session behavior is labelled.
7. The historical Files audit remains open beyond the shared-control, editing and
   input-ownership changes documented in [the text-editing audit](../ux-audit/TEXT_EDITING.md). This branch does not establish a fix for
   the user's eventual freeze: bounded work and tick pacing reduce known risks,
   but a sustained interaction/render soak and profiling are still required.

These are implementation/release blockers, not optional polish. Use the six
screen documents and acceptance matrix to continue the work without treating
this source milestone as the finished framework.

## Text-editing follow-up

The latest correction is implemented in shared source, **not runtime-accepted**.
See [TEXT_EDITING.md](../ux-audit/TEXT_EDITING.md) for the screenshot/root-cause
audit, changed controls, exact verification results, and remaining editing gaps.
`sh tools/test-editing-policy.sh` passes **11 additional tests** and compiles the
production policy in a `no_std` wrapper. Six new real Files integration tests are
added but cannot be executed with the missing dependency cache.

Mirui's `render()` / `snapshot()` do not advance systems. Integration fixtures and
the Studio capture host now tick before checking/capturing system-driven field
metrics, focus and input gates. This fixes a verification-path error; it is not
evidence that the captures or integration tests have run.
