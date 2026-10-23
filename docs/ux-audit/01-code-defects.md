# Code-level UX defects and interaction integrity

Audit date: 2026-10-23. Read alongside [scope and severity](README.md).
These findings concern this checkout, including the prior unverified input patch.
“Confirmed” means established by inspected code, not a successful runtime reproduction.
P0 denotes identity/operation safety or memory-safety release blockers; P1 blocks core use;
P2 is important consistency, efficiency or trust debt. Strong risks name their missing runtime proof.

## Findings

<a id="f01"></a>

### F01 — Moved built-in entries are inserted twice

**Priority:** P0 · **Evidence:** Confirmed code defect

**Source:** [`crates/fm/src/model.rs:1532`](../../crates/fm/src/model.rs#L1532)

**Problem and consequence.** The initial range scan already includes a relocated built-in entry when its effective place matches. The subsequent relocations loop appends the same ID again. One successful move can produce duplicate rows, counts and row keys.

**Required direction.** Build one canonical collection by stable identity; apply location and trash predicates once. Do not merely deduplicate the presentation and leave domain state contradictory.

**Acceptance evidence.** Move one built-in entry to another place. Its ID appears exactly once there, zero times at the origin, and total increases by one, not two.

<a id="f02"></a>

### F02 — Trash does not provide a coherent recoverable collection

**Priority:** P0 · **Evidence:** Confirmed code defect

**Source:** [`crates/fm/src/model.rs:735`](../../crates/fm/src/model.rs#L735); [`crates/fm/src/model.rs:1532`](../../crates/fm/src/model.rs#L1532); [`crates/fm/src/model.rs:749`](../../crates/fm/src/model.rs#L749)

**Problem and consequence.** Trashed built-in IDs are skipped for every place, including Trash; the Trash place instead enumerates a separate synthetic range. Created folders are appended without checking trashed membership. A newly created folder can remain visible after “Moved to Trash”. Restore only removes a flag and cannot make a synthetic Trash-range item leave Trash.

**Required direction.** Represent trash state and original parent explicitly for every entry kind; derive the Trash collection from that state. Expose Restore only where meaningful.

**Acceptance evidence.** Trash and restore both a built-in file and a created folder. They disappear from the source, appear in Trash once, retain origin metadata, then return once.

<a id="f03"></a>

### F03 — Moved created folders can remain in their original place

**Priority:** P0 · **Evidence:** Confirmed code defect

**Source:** [`crates/fm/src/model.rs:856`](../../crates/fm/src/model.rs#L856); [`crates/fm/src/model.rs:1561`](../../crates/fm/src/model.rs#L1561)

**Problem and consequence.** CreatedFolder.place is not changed by move_to. Collection appends created folders based on that old place while the relocations loop appends them at the destination. The same object can be listed in two places with metadata claiming only one.

**Required direction.** Use one authoritative parent/location record; never let collection building use a different location rule from metadata.

**Acceptance evidence.** Create, move, enumerate both places, then move again and trash. Assert single ownership and consistent inspector location after every step.

<a id="f04"></a>

### F04 — Created IDs are derived from a shrinking vector length

**Priority:** P0 · **Evidence:** Confirmed code defect

**Source:** [`crates/fm/src/model.rs:804`](../../crates/fm/src/model.rs#L804); [`crates/fm/src/model.rs:825`](../../crates/fm/src/model.rs#L825); [`crates/fm/src/model.rs:841`](../../crates/fm/src/model.rs#L841)

**Problem and consequence.** IDs use CREATED_BASE + created.len(). Cancelling an older placeholder removes an element. With IDs base, base+1, base+2, removing base makes the next allocated ID base+2, colliding with an existing item. Renames, selection and relocation can then target the wrong object.

**Required direction.** Allocate monotonic checked IDs or generational handles; never derive identity from collection size. Clean associated state on removal.

**Acceptance evidence.** Create three objects, remove an earlier placeholder through the public editing methods, create another and assert uniqueness plus no stale metadata inheritance.

<a id="f05"></a>

### F05 — Duplicate changes every file into a folder

**Priority:** P1 · **Evidence:** Confirmed code defect

**Source:** [`crates/fm/src/model.rs:756`](../../crates/fm/src/model.rs#L756); [`crates/fm/src/model.rs:1316`](../../crates/fm/src/model.rs#L1316)

**Problem and consequence.** duplicate stores every result as CreatedFolder; entry_kind treats all IDs above CREATED_BASE as folders. Duplicating a document or image loses its kind, size and meaningful content semantics while reporting success.

**Required direction.** Use a typed entry record and a real copy operation contract; preserve kind and metadata, then resolve naming conflicts.

**Acceptance evidence.** Duplicate each supported kind and assert kind, parent, source independence, distinct identity and truthful progress/result handling.

<a id="f06"></a>

### F06 — Date and size sorting compare formatted labels

**Priority:** P1 · **Evidence:** Confirmed code defect

**Source:** [`crates/fm/src/model.rs:1582`](../../crates/fm/src/model.rs#L1582); [`crates/fm/src/model.rs:1589`](../../crates/fm/src/model.rs#L1589)

**Problem and consequence.** String ordering puts “100 KB” before “9 KB” and “Oct 10” before “Oct 2”. Date labels omit year; integer MB formatting also discards precision. Visible order contradicts the requested criterion.

**Required direction.** Sort typed timestamps and byte counts, with stable identity/name tie-breakers and a documented folder policy. Format only at the display boundary.

**Acceptance evidence.** Use mixed units, equal displayed sizes, multiple months/years and missing metadata; assert numeric/chronological order and stable ties.

<a id="f07"></a>

### F07 — Show hidden files has no filtering effect

**Priority:** P1 · **Evidence:** Confirmed code gap

**Source:** [`crates/fm/src/model.rs:468`](../../crates/fm/src/model.rs#L468); [`crates/fm/src/model.rs:1532`](../../crates/fm/src/model.rs#L1532)

**Problem and consequence.** The toggle changes show_hidden and recomputes, but collect_visible never reads it and entries have no hidden flag. The control promises an operation it cannot perform.

**Required direction.** Add domain visibility metadata and one filter predicate, or hide/disable the setting in a clearly labelled demo mode.

**Acceptance evidence.** A hidden entry is absent by default, appears when toggled, and is counted consistently; ordinary entries remain unchanged.

<a id="f08"></a>

### F08 — Both views silently cap reachable content

**Priority:** P1 · **Evidence:** Confirmed code defect

**Source:** [`crates/fm/src/model.rs:168`](../../crates/fm/src/model.rs#L168); [`crates/fm/src/model.rs:187`](../../crates/fm/src/model.rs#L187); [`crates/fm/src/model.rs:1494`](../../crates/fm/src/model.rs#L1494)

**Problem and consequence.** The grid renders at most 32 entries (12 for a search); the list renders at most 48. These are not virtual windows with scroll-driven replacement. Remaining entries have no rendered rows to scroll to. This is a functional loss, not an acceptable performance optimization.

**Required direction.** Implement verified virtualization or explicit pagination with reachable totals. Never truncate a directory invisibly.

**Acceptance evidence.** Reach and activate first, middle and final items in 2,048 and 10,000 entry datasets in both views, without unbounded retained widgets.

<a id="f09"></a>

### F09 — Status count reports more displayed rows than exist

**Priority:** P1 · **Evidence:** Confirmed code defect

**Source:** [`crates/fm/src/model.rs:1474`](../../crates/fm/src/model.rs#L1474); [`crates/fm/src/model.rs:1371`](../../crates/fm/src/model.rs#L1371); [`crates/fm/src/model.rs:1494`](../../crates/fm/src/model.rs#L1494)

**Problem and consequence.** List shown equals total despite the 48-row cap. Grid shown uses the 32-item cap even when grid_count uses the 12-result search cap. Users cannot tell whether results are omitted, filtered or scrollable.

**Required direction.** Derive matched, total, loaded and visible counts from one collection view model; show only distinctions users can act on.

**Acceptance evidence.** For empty, search-limited and large directories, compare status count to reachable results and document the precise count meaning.

<a id="f10"></a>

### F10 — List labels and actions can refer to different objects

**Priority:** P0 · **Evidence:** Strong code-derived risk

**Source:** [`crates/fm/src/ui/content.rs:90`](../../crates/fm/src/ui/content.rs#L90); [`crates/fm/src/ui/content.rs:223`](../../crates/fm/src/ui/content.rs#L223)

**Problem and consequence.** Rows are walked by index; row_cell captures id and kind once. Labels later resolve entry_label(index) on listing_revision, while selection, DropZone, icon and Tap retain captured id. If keyed rows are retained after sorting/filtering/navigation, the label changes but clicking or dragging can act on the old file. Dependency walk retention semantics must be verified.

**Required direction.** Key rows by stable entry identity, or update all presentation and interaction fields together during rebinding. No index-to-identity split.

**Acceptance evidence.** In list mode, sort/filter/switch places, then click, inspect and drag each first row. Assert visible name, selection, menu target, kind and moved ID are the same object.

<a id="f11"></a>

### F11 — Last grid row builds interactive sentinel entries

**Priority:** P1 · **Evidence:** Strong code-derived risk

**Source:** [`crates/fm/src/ui/content.rs:114`](../../crates/fm/src/ui/content.rs#L114); [`crates/fm/src/model.rs:1513`](../../crates/fm/src/model.rs#L1513); [`crates/fm/src/model.rs:1303`](../../crates/fm/src/model.rs#L1303)

**Problem and consequence.** Every row builds all columns, even beyond grid_count. Missing positions return u32::MAX; entry_kind labels the sentinel Folder and entry_title eventually falls through to the clamped final synthetic entry. These cells still carry HitTarget and DropZone.

**Required direction.** Represent absent entries as Option and do not construct an interactive cell for them. Avoid sentinels that fall through normal file APIs.

**Acceptance evidence.** Render result counts 1, 5 and columns+1. Count exactly that many actionable tiles; clicking blank final-row space cannot select or open a sentinel.

<a id="f12"></a>

### F12 — Open is not open, and grid/list activation differs

**Priority:** P1 · **Evidence:** Confirmed code gap

**Source:** [`crates/fm/src/ui/menu.rs:255`](../../crates/fm/src/ui/menu.rs#L255); [`crates/fm/src/ui/content.rs:208`](../../crates/fm/src/ui/content.rs#L208); [`crates/fm/src/ui/content.rs:223`](../../crates/fm/src/ui/content.rs#L223)

**Problem and consequence.** Menu Open selects an item and closes the menu; grid double-tap opens Get Info instead of opening a folder/file. List rows have no corresponding double-tap activation. There is no directory-enter or application-launch contract.

**Required direction.** Define one Activate command: navigate a directory or ask the host to open a file. Map all activation surfaces consistently. Info remains a distinct action.

**Acceptance evidence.** Open via menu, double-click, Enter and accessibility action in both views; assert identical domain intent and preserved history.

<a id="f13"></a>

### F13 — Folder drop does not establish a parent-child relationship

**Priority:** P1 · **Evidence:** Confirmed code gap

**Source:** [`crates/fm/src/model.rs:917`](../../crates/fm/src/model.rs#L917)

**Problem and consequence.** A tile destination resolves to place_of(folder). Dropping onto a folder in the same place leaves the source in the same place; self/descendant relationships are not modeled. The toast can suggest physical organization that did not occur.

**Required direction.** Model parent directory IDs and validate destinations, cycles, permissions and cross-volume move/copy policy before showing allowed-drop feedback.

**Acceptance evidence.** Drop into sibling folder, self, descendant, read-only folder and another volume. Verify parent relationship or precise rejection with no false success.

<a id="f14"></a>

### F14 — Icon-size slider ignores its emitted value

**Priority:** P1 · **Evidence:** Confirmed code defect

**Source:** [`crates/fm/src/ui/options.rs:82`](../../crates/fm/src/ui/options.rs#L82)

**Problem and consequence.** The handler reads the existing icon_size and discards it. Moving the control does not commit the new value to the model. Model-only tests calling set_icon_size cannot prove slider functionality.

**Required direction.** Bind the emitted numeric value through a shared controlled Slider contract, clamp/quantize once and render from canonical state.

**Acceptance evidence.** Drag and keyboard-adjust the actual slider; assert model value, thumb, numeric label and displayed icon size agree, including after reopening.

<a id="f15"></a>

### F15 — Inline editor visibility is frozen at composition

**Priority:** P1 · **Evidence:** Strong code-derived risk

**Source:** [`crates/fm/src/ui/content.rs:131`](../../crates/fm/src/ui/content.rs#L131); [`crates/fm/src/model.rs:818`](../../crates/fm/src/model.rs#L818); [`crates/fm/src/model.rs:804`](../../crates/fm/src/model.rs#L804)

**Problem and consequence.** cell reads editing_folder_id into a local boolean and uses non-reactive if editing. Row keys incorporate entries and icon size, not editing state. Existing tiles do not necessarily rebuild when editing starts. New-folder creation also recomputes before setting editing_folder_id; no focus/select-all request is made.

**Required direction.** Use a reactive edit-state branch keyed by stable identity and request focus/select-all after editor mount. Never depend on incidental tree rebuild timing.

**Acceptance evidence.** Start editing an existing folder and a newly created folder. Exactly one mounted editor is visible, focused, selected and scrolled into view without an unrelated rebuild.

<a id="f16"></a>

### F16 — New-folder behavior differs across entry points and views

**Priority:** P1 · **Evidence:** Confirmed code gap

**Source:** [`crates/fm/src/ui/chrome.rs:222`](../../crates/fm/src/ui/chrome.rs#L222); [`crates/fm/src/ui/menu.rs:281`](../../crates/fm/src/ui/menu.rs#L281); [`crates/fm/src/model.rs:702`](../../crates/fm/src/model.rs#L702); [`crates/fm/src/ui/content.rs:223`](../../crates/fm/src/ui/content.rs#L223)

**Problem and consequence.** Toolbar starts an inline editor; menu and palette open a dialog. List rows have no inline edit branch. A newly created item can sort below the caps or be filtered out, leaving the user with no visible editor.

**Required direction.** Dispatch one NewFolder intent with a consistent editor controller supporting grid/list, reveal, provisional name, commit and cancel. Provide a deliberate fallback only when necessary.

**Acceptance evidence.** Create from all entry points in both views with a search active and a large sorted directory; the provisional item is reachable and editing rules match.

<a id="f17"></a>

### F17 — Name editing lacks errors, conflict handling and safe cancellation

**Priority:** P1 · **Evidence:** Confirmed code gap

**Source:** [`crates/fm/src/model.rs:771`](../../crates/fm/src/model.rs#L771); [`crates/fm/src/model.rs:785`](../../crates/fm/src/model.rs#L785); [`crates/fm/src/model.rs:825`](../../crates/fm/src/model.rs#L825); [`crates/fm/src/ui/content.rs:191`](../../crates/fm/src/ui/content.rs#L191)

**Problem and consequence.** Only empty trimmed names are rejected; duplicate/reserved/invalid names have no feedback. Blur always commits inline editing, while no Escape wiring invokes cancel_inline_rename. Mutating text and then clicking another action may commit unexpectedly.

**Required direction.** Define editing state with validation errors, pending request, explicit commit/cancel and documented blur behavior. Host rules determine valid names; error text stays next to the field.

**Acceptance evidence.** Test empty, whitespace, conflicts, separators, Unicode, permission failure, Escape, Enter and clicking another item; no lost name or silent success.

<a id="f18"></a>

### F18 — Forward and native-looking window controls are decorative

**Priority:** P1 · **Evidence:** Confirmed code gap

**Source:** [`crates/fm/src/ui/chrome.rs:129`](../../crates/fm/src/ui/chrome.rs#L129); [`crates/fm/src/ui/chrome.rs:88`](../../crates/fm/src/ui/chrome.rs#L88)

**Problem and consequence.** Forward is a HitTarget without an action or forward stack. Red/yellow/green Views have no window callbacks. These affordances imply available actions that cannot occur.

**Required direction.** Remove fake controls from production or connect host window actions and enabled state. Implement bidirectional navigation with explicit button semantics.

**Acceptance evidence.** Back/forward work across visits; unavailable controls are visibly and semantically disabled. Close/minimize/maximize either work or are absent.

<a id="f19"></a>

### F19 — Location display is fixed, not navigation state

**Priority:** P1 · **Evidence:** Confirmed code gap

**Source:** [`crates/fm/src/ui/chrome.rs:41`](../../crates/fm/src/ui/chrome.rs#L41)

**Problem and consequence.** Every place shows the same Apple-like path regardless of actual location. It is not an editable address or breadcrumb and has no relationship to Naledzi namespace rules.

**Required direction.** Use typed location/breadcrumb data supplied by the app; label demo paths explicitly. Support useful copy/navigation actions only when implemented.

**Acceptance evidence.** Visit every place and nested directory; visible location and copied address match current domain state.

<a id="f20"></a>

### F20 — Palette omits the final declared command

**Priority:** P1 · **Evidence:** Confirmed code defect

**Source:** [`crates/fm/src/model.rs:145`](../../crates/fm/src/model.rs#L145); [`crates/fm/src/ui/palette.rs:113`](../../crates/fm/src/ui/palette.rs#L113)

**Problem and consequence.** COMMANDS has 17 entries indexed 0..16, but command_rows manually constructs only 0..15. Open Command Palette is counted by command_count but never rendered. Static duplicated row lists will drift whenever commands are added.

**Required direction.** Render descriptors from the registry itself and test registry-to-view completeness. Avoid hardcoded command indices.

**Acceptance evidence.** Every registered available command has exactly one result row; filtering a final-entry-only label cannot produce a positive count with no row.

<a id="f21"></a>

### F21 — Palette is click-filterable but not a complete command interface

**Priority:** P1 · **Evidence:** Confirmed code gap

**Source:** [`crates/fm/src/ui/palette.rs:20`](../../crates/fm/src/ui/palette.rs#L20); [`crates/fm/src/model.rs:677`](../../crates/fm/src/model.rs#L677)

**Problem and consequence.** No selected-result model, Arrow/Enter execution, Escape dismissal, auto-focus, command enabled predicate, context or no-results surface is implemented in this layer. All commands appear equally available even in inappropriate states.

**Required direction.** Add a reusable palette controller with command registry, keyboard navigation, enabled explanations and a proper empty result state.

**Acceptance evidence.** Open by Ctrl P, type, navigate results, execute Enter and dismiss Escape without mouse; focus returns to the invoker and disabled actions cannot execute.

<a id="f22"></a>

### F22 — Opening the palette leaves a context menu logically open

**Priority:** P1 · **Evidence:** Confirmed code defect

**Source:** [`crates/fm/src/model.rs:647`](../../crates/fm/src/model.rs#L647); [`crates/fm/src/model.rs:587`](../../crates/fm/src/model.rs#L587); [`crates/fm/src/ui/menu.rs:57`](../../crates/fm/src/ui/menu.rs#L57)

**Problem and consequence.** open_palette clears menu but not menu_target. Menu visibility and menu_open depend on menu_target, so a prior context menu survives under the palette and can reappear after close. Overlay state is represented redundantly.

**Required direction.** Use one authoritative menu state and explicit overlay transition policy; opening a modal palette closes transient menus atomically.

**Acceptance evidence.** Open a context menu, invoke Ctrl P, close palette and verify no menu remains visible, logically open or input-blocking.

<a id="f23"></a>

### F23 — Background-menu actions do not consistently close the menu

**Priority:** P1 · **Evidence:** Confirmed code gap

**Source:** [`crates/fm/src/ui/menu.rs:279`](../../crates/fm/src/ui/menu.rs#L279); [`crates/fm/src/model.rs:474`](../../crates/fm/src/model.rs#L474); [`crates/fm/src/model.rs:1203`](../../crates/fm/src/model.rs#L1203)

**Problem and consequence.** New-folder opens a dialog that closes the menu, but sort changes and toggle_options do not call close_menu. Menu and options can coexist with competing backdrops; sort can leave a transient menu lingering after activation.

**Required direction.** Command execution owns menu dismissal and transitions to the next overlay atomically. Preserve menu only for explicitly checkable stay-open settings.

**Acceptance evidence.** Activate every menu row and check resulting focus, menu state and at most the intended overlay. Repeat via keyboard and pointer.

<a id="f24"></a>

### F24 — Previous input patch is not a complete overlay manager

**Priority:** P1 · **Evidence:** Strong code-derived risk

**Source:** [`crates/fm/src/model.rs:1143`](../../crates/fm/src/model.rs#L1143); [`crates/fm/src/sync.rs:72`](../../crates/fm/src/sync.rs#L72)

**Problem and consequence.** The current guard is sampled once per system invocation and only governs custom content drag tracking. A down/up pair between samples, overlay dismissal before the system, wheel events and framework WidgetState hover are not owned by this latch. Initial fixes must not be treated as verified framework isolation.

**Required direction.** Centralize event-time ownership, capture, cancellation, focus and z-order. Retain regression coverage for the latch, but prove routing at the actual dispatch boundary.

**Acceptance evidence.** Replay batched and separate-frame events across all overlays; assert no background hover, activation, scrolling or file movement and no lost next legitimate click.

<a id="f25"></a>

### F25 — Drag targets ignore visibility, clipping and input ownership

**Priority:** P1 · **Evidence:** Strong code-derived risk

**Source:** [`crates/fm/src/sync.rs:85`](../../crates/fm/src/sync.rs#L85)

**Problem and consequence.** The scan checks axis-aligned ComputedRect membership only, stops at the first query match and does not explicitly check clipping, transforms, scroll viewport containment, topmost ordering, cursor.present or pointer identity. Correctness depends on unverified engine behavior.

**Required direction.** Ask a shared hit-test/capture service for the valid target or pass a correctly clipped interactive-region graph. Do not duplicate a partial hit tester per app.

**Acceptance evidence.** Drag over clipped/offscreen rows, overlapping panels, scrollbars and transformed/nested panes; only the top eligible target highlights.

<a id="f26"></a>

### F26 — Drag icon captures the initial item instead of the current one

**Priority:** P1 · **Evidence:** Strong code-derived risk

**Source:** [`crates/fm/src/ui/menu.rs:129`](../../crates/fm/src/ui/menu.rs#L129); [`crates/fm/src/ui/hover.rs:42`](../../crates/fm/src/ui/hover.rs#L42)

**Problem and consequence.** drag_badge reads dragging once at composition and stores it in KindIcon.id. The label reads dragging reactively. Since the badge is initially hidden with no dragged item, its icon can resolve index 0 rather than the actual dragged item, and later drags can mismatch label and icon.

**Required direction.** Pass stable reactive drag-session data into a single item preview component. Keep kind/name/thumbnail/size in the same session snapshot.

**Acceptance evidence.** Drag a folder, image and document successively; every preview matches the source and never intercepts the drop target.

<a id="f27"></a>

### F27 — Drag preview is still a detached label card

**Priority:** P2 · **Evidence:** Confirmed implementation mismatch

**Source:** [`crates/fm/src/ui/menu.rs:128`](../../crates/fm/src/ui/menu.rs#L128)

**Problem and consequence.** The current visual uses a 20px icon and horizontal bordered card offset 12px from the pointer. It is not a lifted version of the grid/list item and does not preserve grab position or selection multiplicity.

**Required direction.** Use a source-derived item preview with grab offset, subtle elevation, allowed/denied operation indication and multi-selection count. Keep direct manipulation distinct from click-through fixes.

**Acceptance evidence.** Compare pointer-relative preview against the source geometry in grid/list and at scaled DPI; source and destination feedback remain legible.

<a id="f28"></a>

### F28 — Scrollbar starts a “thumb” drag anywhere on the rail

**Priority:** P1 · **Evidence:** Confirmed code gap

**Source:** [`crates/fm/src/ui/scrollbar.rs:142`](../../crates/fm/src/ui/scrollbar.rs#L142)

**Problem and consequence.** The entire six-pixel-wide bar is a HitTarget. DragStart checks overflow but never checks whether the pointer lies within the thumb. Track clicks have no separate page/jump policy and no keyboard range interface.

**Required direction.** Separate rail and thumb semantics, increase invisible hit width, provide min thumb size and keyboard actions, and use one geometry function for paint and input.

**Acceptance evidence.** Press thumb center/edges and empty rail; verify documented drag/page behavior, stable grab offset, endpoints, cancellation and disabled overflow state.

<a id="f29"></a>

### F29 — Scrollbar thumb has no minimum usable size

**Priority:** P2 · **Evidence:** Confirmed code gap

**Source:** [`crates/fm/src/ui/scrollbar.rs:109`](../../crates/fm/src/ui/scrollbar.rs#L109); [`crates/fm/src/ui/content.rs:78`](../../crates/fm/src/ui/content.rs#L78)

**Problem and consequence.** Proportional size can approach zero as content grows; hit width is only six logical pixels. The control becomes harder to acquire precisely when the collection becomes larger.

**Required direction.** Set an accessible target envelope independent from painted width, enforce a minimum thumb and share adjusted travel mapping with drag inversion.

**Acceptance evidence.** At very large content extent and several DPI values, thumb remains operable and maps exactly to top/bottom without jumps.

<a id="f30"></a>

### F30 — Inspector tabs and storage are not data-backed features

**Priority:** P1 · **Evidence:** Confirmed code gap

**Source:** [`crates/fm/src/ui/inspector.rs:55`](../../crates/fm/src/ui/inspector.rs#L55); [`crates/fm/src/ui/inspector.rs:105`](../../crates/fm/src/ui/inspector.rs#L105); [`crates/fm/src/ui/status.rs:49`](../../crates/fm/src/ui/status.rs#L49)

**Problem and consequence.** The inspector declares a three-count TabBar with no app tab selection/content handlers. Both inspector and status use a fixed storage figure and 0.62 progress independent of any volume. These look like live facts and controls.

**Required direction.** Remove or clearly label fixtures, then introduce actual tab descriptors/content and typed volume usage. Unknown data is unknown, not a decorative percentage.

**Acceptance evidence.** Switch every tab; it has a label and distinct content. Mount two volumes and verify storage/counts follow the selected volume and report failure/unknown.

<a id="f31"></a>

### F31 — Sidebar counts are captured from the static table

**Priority:** P2 · **Evidence:** Confirmed code defect

**Source:** [`crates/fm/src/ui/sidebar.rs:56`](../../crates/fm/src/ui/sidebar.rs#L56); [`crates/fm/src/ui/kit.rs:41`](../../crates/fm/src/ui/kit.rs#L41)

**Problem and consequence.** Counts are computed once and badge receives a plain u32. Create, trash, move and restore do not update them. The sidebar can disagree with content and status immediately.

**Required direction.** Expose derived reactive per-place counts with a stated meaning and avoid misleading counts for smart collections.

**Acceptance evidence.** Perform every supported mutation; sidebar, collection and status agree, including zero and unknown counts.

<a id="f32"></a>

### F32 — Empty-state and loading components are not used by collection flows

**Priority:** P2 · **Evidence:** Confirmed code gap

**Source:** [`crates/fm/src/ui/kit.rs:65`](../../crates/fm/src/ui/kit.rs#L65); [`crates/fm/src/ui/content.rs:47`](../../crates/fm/src/ui/content.rs#L47)

**Problem and consequence.** The kit defines a static empty_state helper, but area renders only grid/list. No-results, empty directory, loading, permission denied, offline and failed refresh are not distinct UI states. Blank content conveys no recovery path.

**Required direction.** Use a collection state enum and contextual state views with useful actions; preserve prior results during refresh where safe.

**Acceptance evidence.** Render and recover from each state using one app-independent collection controller, including a search-clear action and retry.

<a id="f33"></a>

### F33 — Read-only inspector rows and badges pretend to be interactive

**Priority:** P2 · **Evidence:** Confirmed code gap

**Source:** [`crates/fm/src/ui/inspector.rs:120`](../../crates/fm/src/ui/inspector.rs#L120); [`crates/fm/src/ui/kit.rs:41`](../../crates/fm/src/ui/kit.rs#L41); [`crates/fm/src/ui/kit.rs:181`](../../crates/fm/src/ui/kit.rs#L181)

**Problem and consequence.** Metadata rows, passive count badges and the static tooltip are HitTargets despite having no useful activation. Metadata also paints press feedback. This creates false affordances and extra input targets.

**Required direction.** Keep passive information passive; if copy/details are supported, expose an intentional action with proper semantics. Tooltip content should not become an accidental blocker.

**Acceptance evidence.** Pointer and keyboard target maps contain only real actions; passive metadata never displays a pressed state or steals a drag.

<a id="f34"></a>

### F34 — Dialogs use generic action labels and weak validation feedback

**Priority:** P1 · **Evidence:** Confirmed code gap

**Source:** [`crates/fm/src/ui/dialog.rs:307`](../../crates/fm/src/ui/dialog.rs#L307); [`crates/fm/src/ui/dialog.rs:357`](../../crates/fm/src/ui/dialog.rs#L357)

**Problem and consequence.** All dialogs use Cancel and OK, including About/Info and trash confirmation. Empty names silently do nothing. Rename target is read from current selection at confirmation rather than a durable dialog target. Text fields have no submitted action wiring here.

**Required direction.** Dialog descriptors carry target identity, title, intent-specific primary label, validation, initial focus, pending state and result. Use Close for informational surfaces.

**Acceptance evidence.** Verify Create/Rename/Move to Trash/Close labels, Enter/Escape policy, visible errors and target stability even if app state changes while open.

<a id="f35"></a>

### F35 — Critical and long notification content is short-lived and clipped by design

**Priority:** P2 · **Evidence:** Confirmed code gap

**Source:** [`crates/fm/src/model.rs:1069`](../../crates/fm/src/model.rs#L1069); [`crates/fm/src/ui/status.rs:66`](../../crates/fm/src/ui/status.rs#L66)

**Problem and consequence.** All kinds share the same timeout and the queue drops its oldest at capacity. Fixed 320px cards with one short body line do not scale for paths/errors. Tapping the whole card discards the message; no action/history or pause policy exists.

**Required direction.** Separate ephemeral success from persistent actionable errors; allow wrapping, explicit dismissal, pause on interaction and a host notification/history bridge.

**Acceptance evidence.** Long translated error remains readable, retry/undo is reachable, critical failure survives queue pressure and assistive announcement does not steal focus.


<a id="f36"></a>

### F36 — Successful drag does not recompute the listing

**Priority:** P1 · **Evidence:** Confirmed code defect

**Source:** [`crates/fm/src/model.rs:917`](../../crates/fm/src/model.rs#L917); [`crates/fm/src/model.rs:856`](../../crates/fm/src/model.rs#L856)

**Problem and consequence.** end_drag updates relocations and shows a notice, but unlike move_to it never calls recompute. The source listing, shown/total and listing_revision can remain unchanged after a successful drop. A later refresh then reveals a different state.

**Required direction.** Route a drag completion through the same validated domain move result used by other entry points and update all derived collections once.

**Acceptance evidence.** Drop across places and assert immediate source disappearance, correct count and destination insertion without toggling view or forcing recompute.

<a id="f37"></a>

### F37 — Invalid identities silently alias real entries

**Priority:** P1 · **Evidence:** Confirmed code gap

**Source:** [`crates/fm/src/model.rs:1425`](../../crates/fm/src/model.rs#L1425); [`crates/fm/src/data.rs:294`](../../crates/fm/src/data.rs#L294); [`crates/fm/src/model.rs:1303`](../../crates/fm/src/model.rs#L1303)

**Problem and consequence.** Out-of-range list positions become ID 0; data::entry clamps invalid indices to the last fixture. Deleted or sentinel IDs can acquire real-looking names instead of returning an error. This masks stale-target defects and can misdirect user actions.

**Required direction.** Use Option/Result for lookup and typed IDs validated against a canonical store. Make removed/stale targets unavailable and cancel pending UI actions.

**Acceptance evidence.** Look up invalid, sentinel and deleted IDs through every public read/action path; no valid object is substituted and no operation mutates another file.

<a id="f38"></a>

### F38 — Renames can leave list and inspector metadata stale

**Priority:** P1 · **Evidence:** Strong code-derived risk

**Source:** [`crates/fm/src/ui/content.rs:253`](../../crates/fm/src/ui/content.rs#L253); [`crates/fm/src/ui/inspector.rs:141`](../../crates/fm/src/ui/inspector.rs#L141); [`crates/fm/src/ui/dialog.rs:296`](../../crates/fm/src/ui/dialog.rs#L296); [`crates/fm/src/model.rs:771`](../../crates/fm/src/model.rs#L771)

**Problem and consequence.** Rename increments names_revision, but list labels and most info fields explicitly observe listing_revision, not names_revision. If the renamed entry remains in the same sorted ID order, recompute does not increment listing_revision. Inspector name observes names_revision, while Info Name and list rows do not, creating divergent update contracts.

**Required direction.** Define derived entry selectors with exact name/metadata revisions shared by every presentation. Do not rely on sorting to invalidate content.

**Acceptance evidence.** Rename an item without changing its position, with inspector and Info visible in turn; all text updates immediately while untouched rows remain clean.

<a id="f39"></a>

### F39 — Selection can outlive the visible or valid target

**Priority:** P1 · **Evidence:** Confirmed state-model gap

**Source:** [`crates/fm/src/model.rs:434`](../../crates/fm/src/model.rs#L434); [`crates/fm/src/model.rs:493`](../../crates/fm/src/model.rs#L493); [`crates/fm/src/model.rs:1474`](../../crates/fm/src/model.rs#L1474); [`crates/fm/src/model.rs:825`](../../crates/fm/src/model.rs#L825)

**Problem and consequence.** select accepts any u32. Filter/recompute do not reconcile selection, and cancelling an empty provisional item does not clear all references to its ID. Retaining off-filter selection may be intentional, but no visible policy or command target explanation exists.

**Required direction.** Maintain active item, selected IDs and visible focus anchor under explicit policies. Validate membership/existence and clear or explain hidden selections; prune references when removing an object.

**Acceptance evidence.** Filter out, remove or navigate away from selected/edited targets, then invoke Info/Rename/Delete. The intended target remains explicit and invalid actions cannot proceed.

<a id="f40"></a>

### F40 — Checkbox label row is not a toggle target

**Priority:** P2 · **Evidence:** Confirmed code defect

**Source:** [`crates/fm/src/ui/options.rs:117`](../../crates/fm/src/ui/options.rs#L117); [`crates/fm/src/ui/options.rs:110`](../../crates/fm/src/ui/options.rs#L110)

**Problem and consequence.** Only the nested 16×16 View handles Tap. The surrounding labelled row is a HitTarget with press/hover feedback but no Tap action. Clicking the visible label can look interactive without changing anything.

**Required direction.** Make the labelled row one accessible checkbox target with a visual indicator child; prevent nested double toggles.

**Acceptance evidence.** Click label, indicator and row padding and press Space when focused; each toggles exactly once. Disabled state blocks all routes.

<a id="f41"></a>

### F41 — Synthetic date formatter has an off-by-one day

**Priority:** P2 · **Evidence:** Confirmed code defect

**Source:** [`crates/fm/src/data.rs:117`](../../crates/fm/src/data.rs#L117); [`crates/fm/src/data.rs:212`](../../crates/fm/src/data.rs#L212)

**Problem and consequence.** make_entry generates day 1..28, but month_day indexes MONTH_DAYS with day rather than day-1. Day 1 displays “2”. Even fixture metadata should be truthful when used to verify sort and inspector behavior.

**Required direction.** Use typed validated dates and one formatter; if fixtures remain, test known edge values explicitly.

**Acceptance evidence.** Day 1, 28 and 31 format correctly; invalid days are rejected or intentionally represented, not silently shifted/clamped.

<a id="f42"></a>

### F42 — Menu availability does not reflect location or action preconditions

**Priority:** P2 · **Evidence:** Confirmed code gap

**Source:** [`crates/fm/src/ui/menu.rs:85`](../../crates/fm/src/ui/menu.rs#L85); [`crates/fm/src/ui/menu.rs:271`](../../crates/fm/src/ui/menu.rs#L271); [`crates/fm/src/ui/menu.rs:270`](../../crates/fm/src/ui/menu.rs#L270)

**Problem and consequence.** Every item menu shows Open, Trash and Put Back regardless of whether the target is already trashed, unavailable, read-only or a folder. Restore can report success for an ordinary file. The menu has no semantic disabled/reason state.

**Required direction.** Derive action availability and checked/destructive roles from a command context with target capabilities and location. Use consistent availability across menu, palette and keyboard.

**Acceptance evidence.** Compare ordinary, trashed, missing and read-only items; only valid actions execute, unavailable states have a clear explanation and no misleading success toast.
