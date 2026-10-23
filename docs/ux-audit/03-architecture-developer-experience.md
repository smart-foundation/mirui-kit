# Architecture, portability and developer-experience audit

Goal: make the correct app cheap to build and the unsafe/inconsistent app hard to build.
Findings separate application-specific fixture defects from capabilities needed by a reusable kit.
Mirui dependency internals are not checked out here; comments about upstream behavior are leads,
not independent proof. Preserve `no_std` + `alloc` and reuse existing rendering capabilities.

## Findings

<a id="a01"></a>

### A01 — The “kit” is a file-manager application crate

**Priority:** P1 · **Evidence:** Confirmed architectural gap

**Source:** [`Cargo.toml:3`](../../Cargo.toml#L3); [`crates/fm/src/lib.rs:27`](../../crates/fm/src/lib.rs#L27); [`crates/fm/src/ui/kit.rs:204`](../../crates/fm/src/ui/kit.rs#L204)

**Problem and consequence.** All reusable UI lives under fm and many controls take FmState. Checkbox reads show_hidden directly; segmented_control switches FmState.view; shell installs demo fonts/theme/state together. Building another app currently means copying internals or importing the file manager.

**Required direction.** Extract a small app-independent kit after contracts stabilize: foundations, controls, overlay/focus/command services and application patterns. Keep filesystem/domain adapters separate; retain fm as an example consumer and integration stress test.

**Acceptance evidence.** Build a settings app and a small document app using public kit APIs without FmState, file-kind data or copied paint/input code.

<a id="a02"></a>

### A02 — Installation is neither configurable nor cleanly scoped

**Priority:** P1 · **Evidence:** Confirmed architectural gap

**Source:** [`crates/fm/src/lib.rs:27`](../../crates/fm/src/lib.rs#L27); [`crates/fm/src/theme.rs:137`](../../crates/fm/src/theme.rs#L137); [`crates/fm/src/font.rs:122`](../../crates/fm/src/font.rs#L122)

**Problem and consequence.** install always registers and activates dark theme, installs static fonts, adds systems/timer, creates a model and spawns a root. There is no install config, teardown handle or typed failure. Installing into an existing host can override host choices or duplicate work.

**Required direction.** Provide KitPlugin/config for services once per world and an AppMount/session for each app root. Make theme/font policy and root ownership explicit; return actionable initialization errors.

**Acceptance evidence.** Mount/unmount two independent app roots, preserve host palette, verify no duplicate timers/resources/IDs, and report missing capabilities without panic or silent fallback.

<a id="a03"></a>

### A03 — Controls are coupled to app state rather than reusable value contracts

**Priority:** P1 · **Evidence:** Confirmed architectural gap

**Source:** [`crates/fm/src/ui/controls.rs:48`](../../crates/fm/src/ui/controls.rs#L48); [`crates/fm/src/ui/kit.rs:204`](../../crates/fm/src/ui/kit.rs#L204); [`crates/fm/src/ui/options.rs:137`](../../crates/fm/src/ui/options.rs#L137)

**Problem and consequence.** A checkbox stores an FmState handle instead of checked/mixed/disabled state plus change intent. View switch and sort helpers hardcode their domain. This prevents reuse and encourages subtly different controls in every OS app.

**Required direction.** Use controlled values and typed callbacks/intents. Provide checked, mixed, disabled, invalid and pending states with labels/focus semantics built in. Domain adapters translate values into app commands.

**Acceptance evidence.** Use the same checkbox/segmented group in Files and Settings; app bindings differ, behavior and accessibility do not.

<a id="a04"></a>

### A04 — There is no single command model across UI entry points

**Priority:** P1 · **Evidence:** Confirmed architectural gap

**Source:** [`crates/fm/src/model.rs:124`](../../crates/fm/src/model.rs#L124); [`crates/fm/src/ui/menu.rs:24`](../../crates/fm/src/ui/menu.rs#L24); [`crates/fm/src/ui/dialog.rs:307`](../../crates/fm/src/ui/dialog.rs#L307); [`targets/desktop/src/main.rs:87`](../../targets/desktop/src/main.rs#L87)

**Problem and consequence.** Command enum, menu u8 IDs, dialog action numbers, literal labels and SDL shortcuts are independently defined. Icon lookup uses labels and new-folder behaviors already diverge. The same user intent has multiple implementations and availability rules.

**Required direction.** Use stable typed CommandId descriptors with label/icon, scope, default bindings, enabled/checked state and typed target context. Route menu, toolbar, palette and keyboard through one dispatcher. Do not force every app action into one global enum.

**Acceptance evidence.** One command registration drives all surfaces, remapping works once, disabled actions cannot execute through an alternate route, and context target is stable.

<a id="a05"></a>

### A05 — Overlay booleans allow contradictory states

**Priority:** P1 · **Evidence:** Confirmed architectural gap

**Source:** [`crates/fm/src/model.rs:259`](../../crates/fm/src/model.rs#L259); [`crates/fm/src/model.rs:272`](../../crates/fm/src/model.rs#L272); [`crates/fm/src/model.rs:647`](../../crates/fm/src/model.rs#L647); [`crates/fm/src/ui/mod.rs:67`](../../crates/fm/src/ui/mod.rs#L67)

**Problem and consequence.** Menu, dialog, palette and options are independent state with fixed composition order. Dismiss Command omits options; palette clears only part of menu state. There is no generic stack, parent relationship, restore-focus owner, keyboard scope or exclusivity policy.

**Required direction.** Create an OverlayHost/controller with modal/transient/nonmodal policies, anchors, focus restoration, outside/Escape behavior, event ownership and explicit transition APIs. Keep passive notices separate.

**Acceptance evidence.** Open/replace/nest overlays from every source; only valid combinations exist, Escape closes the top dismissible layer, focus restores and dismissal never reaches background controls.

<a id="a06"></a>

### A06 — Filesystem mutations are synchronous fixture edits, not a platform contract

**Priority:** P0 · **Evidence:** Confirmed architecture limitation

**Source:** [`crates/fm/src/model.rs:785`](../../crates/fm/src/model.rs#L785); [`crates/fm/src/model.rs:856`](../../crates/fm/src/model.rs#L856); [`crates/fm/src/data.rs:217`](../../crates/fm/src/data.rs#L217)

**Problem and consequence.** All operations mutate synthetic vectors and instantly report success. There are no operation IDs, permission capabilities, request/result messages, cancellation or stale-result handling. Exposing OS I/O directly inside this model would couple UI responsiveness to filesystem latency.

**Required direction.** Define asynchronous domain requests/results outside the core kit, with stable file handles, parent identity, operation IDs and errors. Naledzi adapter performs IPC; UI consumes snapshots and progress. Pure no_std reducers remain testable.

**Acceptance evidence.** Use a fake delayed adapter for success/failure/out-of-order completions and a real Naledzi adapter for IPC. No success before acknowledgement; no stale result overwrites a newer navigation or edit.

<a id="a07"></a>

### A07 — State ownership and invalidation are too broad and duplicated

**Priority:** P1 · **Evidence:** Confirmed architecture limitation

**Source:** [`crates/fm/src/model.rs:242`](../../crates/fm/src/model.rs#L242); [`crates/fm/src/model.rs:1474`](../../crates/fm/src/model.rs#L1474); [`crates/fm/src/model.rs:985`](../../crates/fm/src/model.rs#L985); [`crates/fm/src/sync.rs:31`](../../crates/fm/src/sync.rs#L31)

**Problem and consequence.** One large model mixes filesystem fixtures, navigation, selection, dialogs, documents, theme, notices and screenshot requests. Notification state is duplicated between notices, ages, toast and counters. Broad revisions and manual observation reads increase coupling and stale-view risks.

**Required direction.** Separate domain/session/view/service state with explicit derived selectors. Preserve precise observations and compare-before-write semantics. Avoid a rewrite to an unproven state library; migrate one boundary with tests.

**Acceptance evidence.** Selection invalidates selection-dependent views only; unrelated notices do not rebuild directory rows; state transitions preserve invariants and equal updates are no-ops.

<a id="a08"></a>

### A08 — Global IDs make multi-instance composition unsafe

**Priority:** P1 · **Evidence:** Strong code-derived risk

**Source:** [`crates/fm/src/ui/chrome.rs:261`](../../crates/fm/src/ui/chrome.rs#L261); [`crates/fm/src/ui/content.rs:171`](../../crates/fm/src/ui/content.rs#L171); [`crates/fm/src/ui/status.rs:97`](../../crates/fm/src/ui/status.rs#L97); [`crates/fm/src/ui/scrollbar.rs:26`](../../crates/fm/src/ui/scrollbar.rs#L26)

**Problem and consequence.** Components and shadow links use global string IDs. Two app roots or repeated controls can collide in IdMap, and transient editors share one ID. Whether the engine overwrites or rejects duplicates needs verification; either behavior obstructs independent instances.

**Required direction.** Scope generated identities by mount/component instance. Prefer returned entity handles or scoped references for shadow/scroll targets; reserve public names for test/accessibility hooks.

**Acceptance evidence.** Mount two file-like views and repeated dialogs/fields. Each scrollbar, focus action, shadow and test locator resolves to the correct instance after teardown/remount.

<a id="a09"></a>

### A09 — Text editing policy is duplicated and engine-coupled

**Priority:** P1 · **Evidence:** Confirmed architectural gap

**Source:** [`crates/fm/src/ui/controls.rs:31`](../../crates/fm/src/ui/controls.rs#L31); [`crates/fm/src/ui/dialog.rs:182`](../../crates/fm/src/ui/dialog.rs#L182); [`crates/fm/src/ui/palette.rs:96`](../../crates/fm/src/ui/palette.rs#L96)

**Problem and consequence.** Each form manually hides the engine focus border, installs FieldFocus, duplicates document wiring and chooses fixed metrics. Changed handlers write back into the same document; errors are ignored. Event-loop/redundant update behavior depends on engine equality checks.

**Required direction.** Offer Field/Editor bindings with explicit source-of-truth, validation, commit/cancel, selection and errors. Encapsulate mirui-specific workarounds once and document supported text input/IME capabilities.

**Acceptance evidence.** Type, paste, compose IME, undo and submit in multiple bound editors. One change yields one state update, no feedback loop, and errors cannot disappear silently.

<a id="a10"></a>

### A10 — Performance diagnostics are examples, not enforceable budgets

**Priority:** P1 · **Evidence:** Confirmed test/design gap

**Source:** [`crates/fm/examples/bench.rs:7`](../../crates/fm/examples/bench.rs#L7); [`crates/fm/examples/leak.rs:8`](../../crates/fm/examples/leak.rs#L8); [`crates/fm/examples/perf.rs:89`](../../crates/fm/examples/perf.rs#L89)

**Problem and consequence.** Existing examples print timings/entity counts, but do not fail against budgets or measure true heap growth, input latency and long-session resource retention. A frame cap limits tick rate, not work per frame or a blocking task.

**Required direction.** Retain diagnostics, add reproducible workloads, per-phase timing, memory/damage counters and hardware-qualified budgets. Profile the reported freeze rather than assuming an uncapped loop is the only cause.

**Acceptance evidence.** Record cold/warm startup, idle, hover, selection, scrolling, filtering and overlay/theme/view soak distributions plus memory plateau. Regressions trigger an explicit review gate.

<a id="a11"></a>

### A11 — Bare allocator violates alignment and failure accounting

**Priority:** P0 · **Evidence:** Confirmed code defect

**Source:** [`targets/bare/src/main.rs:55`](../../targets/bare/src/main.rs#L55)

**Problem and consequence.** BumpAllocator returns base+used without aligning to Layout.align(). HEAP is a byte array with no sufficient alignment guarantee. fetch_add consumes space even when the capacity check fails and addition is unchecked. This does not meet GlobalAlloc requirements and can yield undefined behavior.

**Required direction.** Use a proven platform allocator or a correctly aligned checked bump allocator for the smoke target. Keep a non-reclaiming allocator limited to bounded one-shot demos; supply a real allocator for interactive Naledzi apps.

**Acceptance evidence.** Host-test mixed alignments, near-capacity failure, overflow and concurrent allocation; every successful pointer is correctly aligned and disjoint, failure leaves state valid.

<a id="a12"></a>

### A12 — Bare critical section is a non-reentrant spin lock

**Priority:** P0 · **Evidence:** Strong platform safety risk

**Source:** [`targets/bare/src/main.rs:35`](../../targets/bare/src/main.rs#L35)

**Problem and consequence.** acquire spins on AtomicBool and returns no saved interrupt state. Nested entry by the same execution context or an interrupt attempting the lock can deadlock. A single-core comment does not make spinning interrupt-safe.

**Required direction.** Use the board/Naledzi critical-section implementation with documented interrupt masking, nesting and multicore behavior. Do not improvise kernel synchronization in the UI sample.

**Acceptance evidence.** Verify nested sections and interrupt entry on the actual target; establish cross-core safety where relevant and demonstrate bounded scheduling behavior.

<a id="a13"></a>

### A13 — Bare target heap is smaller than a full RGB565 frame

**Priority:** P1 · **Evidence:** Measured budget risk

**Source:** [`targets/bare/src/main.rs:22`](../../targets/bare/src/main.rs#L22); [`targets/bare/src/main.rs:87`](../../targets/bare/src/main.rs#L87)

**Problem and consequence.** 800×480×2 is 768,000 bytes, larger than the 524,288-byte heap, before widget allocations, text layout and caches. The surface allocation strategy is not available locally, so exact startup failure is not asserted. Default embedded font files add 5,631,568 bytes of static atlas assets, not necessarily heap.

**Required direction.** Measure static image, framebuffers, scratch, font decode/cache, UI heap and peak operation budgets separately. Use externally supplied framebuffer storage or an appropriate memory configuration.

**Acceptance evidence.** Publish a target memory map and measured boot/render peak. Exercise low-memory paths without silent spin/panic; prove an interactive session can allocate and reclaim within budget.

<a id="a14"></a>

### A14 — SDL event interception is lossy and not a portable input contract

**Priority:** P1 · **Evidence:** Confirmed platform gap

**Source:** [`targets/desktop/src/main.rs:52`](../../targets/desktop/src/main.rs#L52); [`targets/desktop/src/main.rs:44`](../../targets/desktop/src/main.rs#L44); [`targets/desktop/src/main.rs:136`](../../targets/desktop/src/main.rs#L136)

**Problem and consequence.** One pending flag and last coordinates collapse multiple right-clicks. MouseButtonDown Right is filtered, but the matching right-button release is not explicitly filtered; interpretation depends on the surface backend. Right-click target is previous hovered_item, not a hit test at event coordinates. Global state also prevents per-window ownership.

**Required direction.** Translate host events into a per-window typed queue with button, pointer ID, modifiers, timestamp, coordinates and cancellation. Perform context-menu hit testing at the event position. Provide the same contract on Naledzi.

**Acceptance evidence.** Test right down/up without pointer movement, rapid consecutive clicks, wheel, multiple windows and focus loss; no unmatched pointer-up or stale target reaches the UI.

<a id="a15"></a>

### A15 — Host errors and screenshot requests need explicit results

**Priority:** P1 · **Evidence:** Confirmed code gap

**Source:** [`targets/desktop/src/main.rs:232`](../../targets/desktop/src/main.rs#L232); [`targets/desktop/src/main.rs:329`](../../targets/desktop/src/main.rs#L329); [`targets/snapshot/src/main.rs:138`](../../targets/snapshot/src/main.rs#L138); [`targets/timestamp.rs:64`](../../targets/timestamp.rs#L64)

**Problem and consequence.** Screenshot names have second precision and File::create overwrites, despite uniqueness claims. Desktop failure is only stderr while the request is marked served. Snapshot serve_screenshot reads a texture without explicitly rendering the just-opened sheet first. Duplicate PNG packing logic does not check stride >= packed row bytes.

**Required direction.** Use a shared validated capture adapter, unique create-new names and typed request/completion/failure messages. Render or snapshot the explicitly requested state generation; do not re-enter arbitrary render work from event handling.

**Acceptance evidence.** Capture twice within one second, fail directory writes, use padded/truncated buffers and capture a newly opened sheet. No overwrite/panic; failure is visible and captured pixels match intended generation.

<a id="a16"></a>

### A16 — Font build transforms have incomplete validation

**Priority:** P1 · **Evidence:** Confirmed reproducibility gap

**Source:** [`tools/patch-font-flags.py:38`](../../tools/patch-font-flags.py#L38); [`tools/bake-fonts.sh:42`](../../tools/bake-fonts.sh#L42); [`crates/fm/tests/typeface.rs:43`](../../crates/fm/tests/typeface.rs#L43)

**Problem and consequence.** The patcher clears GPOS/GSUB flags and writes modified bytes without recalculating sfnt checksums or validating semantic effects. The test claims round-up but computes 11×1.5 as floor 16; bake script includes 17. Four-bit coverage for larger rasters is assumed visually harmless without comparison evidence.

**Required direction.** Record source hashes/tool versions and expected output manifest; use font-format validation, shaping comparison and deliberate rounding policy. Preserve the existing renderer path while proving its supported font pipeline.

**Acceptance evidence.** Rebuild deterministic assets, validate modified font structure/checksums, compare kerning/marks/ligatures and raster selection at fractional DPI. Tests compute the engine’s actual rounding policy.

<a id="a17"></a>

### A17 — Documentation and licensing claims exceed checked-in evidence

**Priority:** P2 · **Evidence:** Confirmed packaging gap

**Source:** [`README.md:74`](../../README.md#L74); [`README.md:135`](../../README.md#L135); [`crates/fm/src/icons.rs:12`](../../crates/fm/src/icons.rs#L12)

**Problem and consequence.** README claims a virtualised list and “real” operations despite capped plain columns and in-memory fixtures. Its Medium token name differs from custom inter-medium. The icons refer readers to assets/LICENSES, but only Inter licensing is present there; a complete Solar attribution/license bundle was not found in inspected assets.

**Required direction.** Make a verified capability matrix, remove stale completion claims and supply required third-party notices/source provenance. Track dependency upgrade notes and support tiers.

**Acceptance evidence.** Every documented capability links to a current test/example. All bundled assets have source, version/hash, license and attribution; no claim relies on obsolete screenshots.

<a id="a18"></a>

### A18 — There is no checked-in automated release gate

**Priority:** P1 · **Evidence:** Confirmed delivery gap

**Source:** [`Makefile:5`](../../Makefile#L5); [`rust-toolchain.toml:2`](../../rust-toolchain.toml#L2)

**Problem and consequence.** Local Make targets exist but no .github workflow was found. They do not enforce visual review, feature combinations, no_std library gates or Naledzi runtime invariants. Linux-specific SDL linking also constrains host portability.

**Required direction.** Add reproducible CI for supported targets/configurations, artifact review and toolchain availability; separate core/headless tests from SDL integration. Publish supported host/platform policy instead of promising universal portability.

**Acceptance evidence.** A clean checkout runs documented gates without local hidden files; failures are actionable. Both default/no-default features and separate bare workspace are exercised.
