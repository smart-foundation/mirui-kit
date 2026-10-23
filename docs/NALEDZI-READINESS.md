# Naledzi readiness audit and release gates

> Follow-up: the [full code-led UX audit](ux-audit/README.md) expands this baseline
> into 76 findings, a reusable-kit blueprint and an evidence-led delivery plan.


Status: **development prototype; not production-ready**.
This is a baseline audit and an initial input-isolation patch, not a completed
redesign or an OS integration certification.

## Evidence and limitations

Reviewed the supplied conversation, current model/synchronisation/shell code,
interaction tests, and saved desktop, options, context-menu and new-folder images.
The saved images are historical evidence, not freshly rendered output from this
checkout. They visibly differ from current code (for example menu icons).
Do not use a historical image to certify a current implementation.

The execution environment has neither `cargo` nor `rustc`. Compilation, linting,
regression execution, new snapshots and interactive soak testing remain pending.
No claim of corrected rendering or eliminated freezes is justified yet.

## Immediate findings and initial patch

### Input ownership — release blocker

Evidence: `ui/menu.rs` used `IgnoreHitTest` on the menu backdrop. The existing
interaction test explicitly expected dismissal to select the file behind it.
`sync::drag_tracking` independently scanned underlying file rectangles and only
blocked one frame of a subset of menu presses. Options, dialogs and the palette
were not considered. A held pointer could acquire a file after leaving a control.

Initial changes:
- Menu backdrop is a hit target; outside tap dismisses only.
- Menu and options panels have hit targets for otherwise empty panel space.
- Model exposes one overlay predicate for menu, options, dialog and palette.
- Content pointer blocking persists until release, including after dismissal.
- A file can only be acquired on the sampled down transition.
- Overlay entry cancels a drag without committing its pending drop.
- Drop highlights are only tracked during a real drag.
- Replaced the test that required click-through with a no-activation assertion.
- Added model and interaction regressions. These have not been executed here.

Remaining: verify framework input/system ordering, hover blocking, wheel routing,
release outside the host window, focus loss, pointer capture, multi-pointer input,
and pressed/dragged controls under each layer. A model-side guard is not a
replacement for framework-level hit testing and capture. In particular, test
popup dismissal that occurs before the synchronisation system samples input.

### Shortcut honesty

Evidence: the desktop filter handles Ctrl P and Ctrl Shift S. Menu and palette
labels advertised additional Cmd shortcuts; no corresponding handlers were found.
Removed unimplemented shortcut hints, including Escape, rather than relabelling
non-functional shortcuts as Ctrl. Keep the two implemented Ctrl hints.

Remaining: central command bindings, editor-aware routing, Escape dismissal,
rename/new-folder bindings, and host-independent keyboard tests. Naledzi must
provide equivalent command input, not inherit an SDL-only implementation.

## Visual findings (saved images; require current reproduction)

- Toolbar mixes global command discovery, file creation, view mode, search and
  app preferences without clear grouping. Search sits between unrelated actions.
- Back and forward do not have equivalent behavior: forward is decorative.
- Traffic-light decorations are not functional window controls. Do not ship
  fake controls; delegate to the host or implement a window-action contract.
- Text at 11 px is overused for filenames, menus, inputs and status. Secondary
  controls are visually weak. Test legibility and contrast before choosing weight.
- New-folder input has insufficient visible breathing room. Its thin strip does
  not read as a comfortable editing target.
- Popover appears over a still-highlighted underlying item. Static screenshots
  cannot establish event ownership; reproduce with real pointer movement.
- Menu has little visible separation from content. Elevation needs deliberate
  surface roles and consistent layer ordering, not per-widget shadow duplication.
- The saved desktop shows host decorations plus internal traffic lights, square
  content edges and no separately floating whole-app surface.
- Current `shell` insets only the toolbar; its content/status fill the host.
  1400x900 SDL dimensions alone do not satisfy a fixed inner-app preview.
- Options use fixed coordinates instead of anchoring to their triggering control.
- Drag preview captures `fm.dragging()` once at composition time for the icon,
  while the label is reactive. Reproduce subsequent drags before calling it fixed.
- The drag visual remains a small bordered horizontal card, not the physical
  grid/list item representation requested.

Do not fix these independently without checking the resulting whole layout.

## Functional and architectural release blockers

1. **Filesystem:** static fake entries and in-memory operations are not a real
   filesystem. Define stable file identity, directory enumeration, metadata,
   asynchronous operation requests/results, permissions and recoverable errors.
   Keep filesystem I/O out of the UI crate. Test with a fake adapter first, then
   implement Naledzi IPC outside `fm`. Never report success before acknowledgement.
2. **Hierarchy:** dropping on a folder currently maps to its place, not a genuine
   parent directory. There is no production directory tree to navigate.
3. **Capacity:** icon grid uses a drawing cap. Verify every entry can be reached,
   not merely that the list view can display thousands of rows.
4. **Options:** slider ValueChanged handler is a no-op; icon size cannot reliably
   round-trip to model state. Connect the actual event value and test reflow.
5. **Responsiveness:** 60 Hz rate cap is not proof of a freeze fix. Capture stacks,
   event rates, frame times, memory and model writes during a desktop soak test.
6. **Typography:** regular remains the default face. Introduce semantic body,
   caption, action and heading roles registered through FontManager; allow host
   theme configuration. Do not globally hardcode semibold or silently map every
   distinct role to one weight. Validate exact atlas sizes and DPI fallbacks.
7. **Access:** keyboard navigation, focus containment/restoration, screen-reader
   semantics, contrast and reduced motion have not been certified. A thick input
   border alone is not accessibility compliance.
8. **Bare target allocator:** current bump allocator ignores Layout alignment,
   consumes space even on failed allocations and never frees. Fix before treating
   execution as safe. Compilation of a no_std binary is not a runtime memory test.
9. **Critical sections:** the bare spin lock is not an interrupt-safe, reentrant
   platform implementation. Supply the real Naledzi synchronization contract.
10. **Memory:** 512 KiB heap is not a measured framebuffer/font/UI budget. Publish
    startup, peak, steady-state and failure-mode measurements per target.
11. **Lifecycle:** focus loss, suspend/resume, resize, DPI changes, low memory,
    cancellation and stale asynchronous results need explicit contracts.

## Ordered delivery plan

1. Execute and extend input regression tests; fix framework capture as needed.
2. Reproduce freeze with instrumentation and a repeatable workload.
3. Establish semantic typography, spacing, focus, surface and elevation roles.
4. Recompose the entire inner app within a larger preview host, preserving app
   coordinates, clipping and popup anchoring. Test at supported sizes and DPI.
5. Repair slider/scrollbar behavior and direct manipulation; verify real flows.
6. Introduce filesystem/command/platform adapters and test failure semantics.
7. Implement Naledzi adapters, memory budget and lifecycle handling.
8. Capture current light/dark states, perform accessibility and soak gates.

## Required validation before release

On a host with the pinned Rust toolchain and SDL dependencies:

```sh
cargo fmt --all -- --check
cargo test --workspace
cargo test -p fm --no-default-features
cargo test -p fm --all-features
cargo clippy --workspace --all-targets -- -D warnings
(cd targets/bare && cargo fmt -- --check)
(cd targets/bare && cargo build --target riscv64gc-unknown-none-elf)
cargo run --release -p snapshot
cargo run --release -p desktop
```

Review fresh snapshots, not just a successful exit. Exercise each overlay with
hover, tap, held drag, wheel and outside dismissal; assert the background stays
unchanged. Test scrollbar thumb end-to-end and slider-to-model updates. Create,
rename, cancel and drag multiple different items in succession. Run light/dark,
keyboard-only, resized viewport and 1x/1.5x/2x DPI cases. Soak the desktop under
repeated overlay/theme/view transitions and record CPU, memory and latency.

For Naledzi, add real IPC/filesystem permission, missing-file, disconnect,
operation cancellation and out-of-memory tests. No production-ready designation
until these gates have reproducible evidence.
