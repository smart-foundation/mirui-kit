# Visual system, layout and accessibility audit

Read [audit scope](README.md) before interpreting severity.
These are source-level and token-level findings, not a claim of a completed visual regression run.
Use 4.5:1 for normal text and 3:1 for large text as explicit reference benchmarks;
small 11/14px text is not reclassified as large just because its weight increases.
WCAG is useful evaluation guidance here, not automatic native-app certification.

## Findings

<a id="v01"></a>

### V01 — There is no complete semantic typography system

**Priority:** P1 · **Evidence:** Confirmed code gap

**Source:** [`crates/fm/src/text.rs:15`](../../crates/fm/src/text.rs#L15); [`crates/fm/src/font.rs:54`](../../crates/fm/src/font.rs#L54); [`crates/fm/src/ui/dialog.rs:116`](../../crates/fm/src/ui/dialog.rs#L116)

**Problem and consequence.** Three size constants and physical weight tokens are not app-suite typography roles. Small text is reused for menus, filename labels, inputs, inspector values and helper copy. Theme changes cannot currently replace a full role definition including size, line height, weight and tracking.

**Required direction.** Define Body, BodyStrong, Caption, Label, Action, SectionTitle, WindowTitle and DialogTitle as semantic role descriptors. Resolve roles through a host-configurable type policy; provide readable defaults, not mandatory thin styling.

**Acceptance evidence.** Replace the body family/weight and enable a larger text scale without editing component internals. All roles update, text remains aligned and controls resize without clipping.

<a id="v02"></a>

### V02 — Normal-sized action text fails the chosen contrast benchmark

**Priority:** P1 · **Evidence:** Measured token-level defect

**Source:** [`crates/fm/src/theme.rs:59`](../../crates/fm/src/theme.rs#L59); [`crates/fm/src/theme.rs:98`](../../crates/fm/src/theme.rs#L98); [`crates/fm/src/ui/dialog.rs:348`](../../crates/fm/src/ui/dialog.rs#L348)

**Problem and consequence.** Opaque sRGB token calculations give white on dark Primary #0A84FF = 3.647:1 and white on light Primary #007AFF = 4.017:1. At 11px, neither meets a 4.5:1 normal-text benchmark. This is arithmetic on palette tokens, not a full rendered accessibility certification.

**Required direction.** Separate accent, link, focus and filled-action colors. Choose foreground/background pairs by role and state. Do not try to compensate for insufficient contrast only by increasing font weight.

**Acceptance evidence.** Automated role-pair contrast tests cover both palettes and interaction states; fresh rendered buttons confirm actual compositing and text remains readable at the minimum supported size.

<a id="v03"></a>

### V03 — Selected sidebar and count labels have weak contrast

**Priority:** P1 · **Evidence:** Measured token-level defect

**Source:** [`crates/fm/src/theme.rs:70`](../../crates/fm/src/theme.rs#L70); [`crates/fm/src/ui/sidebar.rs:89`](../../crates/fm/src/ui/sidebar.rs#L89); [`crates/fm/src/ui/kit.rs:57`](../../crates/fm/src/ui/kit.rs#L57)

**Problem and consequence.** Primary text on selection is 3.154:1 in dark and 3.218:1 in light. Light badge text #6E6E73 on #C6C6C8 is 2.973:1; light tooltip text on black is 4.141:1. These are small normal-text roles. Selection background alone does not make the text accessible.

**Required direction.** Introduce OnSelection, BadgeForeground and TooltipForeground roles. Validate selected+hovered+pressed combinations after alpha compositing and provide non-color state cues.

**Acceptance evidence.** Contrast tests meet the selected normal-text benchmark and a keyboard/user review can distinguish focus, selection, hover and disabled state without color alone.

<a id="v04"></a>

### V04 — Input geometry is tied to one tiny font and engine-specific inset

**Priority:** P1 · **Evidence:** Confirmed implementation limitation

**Source:** [`crates/fm/src/ui/controls.rs:61`](../../crates/fm/src/ui/controls.rs#L61); [`crates/fm/src/ui/controls.rs:31`](../../crates/fm/src/ui/controls.rs#L31); [`crates/fm/src/ui/chrome.rs:247`](../../crates/fm/src/ui/chrome.rs#L247)

**Problem and consequence.** field_height measures only Default at SMALL plus four pixels. It does not accept role, multiline needs or vertical padding. Search wraps this narrow editor in a separate surface while dialogs use it directly, producing inconsistent control bodies and focus outlines.

**Required direction.** Create a reusable Field composition with label, frame, editable content, prefix/suffix, helper/error, padding, minimum height and one focus treatment. Let engine geometry be an internal adapter, not every app’s problem.

**Acceptance evidence.** Search, dialog, settings and inline editors share alignment rules, work with a larger font and show errors without shifting focus or cutting the caret.

<a id="v05"></a>

### V05 — Focus styling is editor-specific, not system-wide

**Priority:** P1 · **Evidence:** Confirmed code gap

**Source:** [`crates/fm/src/ui/controls.rs:76`](../../crates/fm/src/ui/controls.rs#L76); [`crates/fm/src/ui/kit.rs:204`](../../crates/fm/src/ui/kit.rs#L204); [`crates/fm/src/ui/chrome.rs:289`](../../crates/fm/src/ui/chrome.rs#L289)

**Problem and consequence.** FieldFocus only queries TextEditing. Many interactive Views implement Tap without a kit-level focus treatment or keyboard activation contract. A visible 3px field border does not make navigation, menus, switches and scrollbars keyboard usable.

**Required direction.** Define FocusVisible and focus-ring roles shared by every interactive control, with a high-contrast fallback and predictable keyboard order. Keep focus and selection distinct.

**Acceptance evidence.** Keyboard-only walkthrough reaches and activates every enabled control, focus never disappears on dark/light/selected backgrounds, and inactive windows have a defined presentation.

<a id="v06"></a>

### V06 — The floating app preview is not a whole-app surface

**Priority:** P1 · **Evidence:** Confirmed layout mismatch

**Source:** [`crates/fm/src/ui/mod.rs:37`](../../crates/fm/src/ui/mod.rs#L37); [`targets/desktop/src/main.rs:35`](../../targets/desktop/src/main.rs#L35)

**Problem and consequence.** The root is 100% host size; only titlebar/toolbar sit in padded chrome. Content and status fill the outer window. Increasing SDL to 1400x900 stretches the app rather than displaying an independently sized inner app.

**Required direction.** Introduce PreviewStage outside AppShell. Stage owns centering/backdrop; shell owns a fixed or explicitly constrained app viewport, whole-window corners and elevation. Production hosts mount shell directly.

**Acceptance evidence.** At 1400x900 host size, verify the requested unchanged inner app size, visible margins on all sides, consistent corners and correct popup/input coordinates. Test direct production mounting too.

<a id="v07"></a>

### V07 — Grid width math ignores the inspector and actual content bounds

**Priority:** P1 · **Evidence:** Confirmed layout defect

**Source:** [`crates/fm/src/model.rs:1385`](../../crates/fm/src/model.rs#L1385); [`crates/fm/src/ui/inspector.rs:21`](../../crates/fm/src/ui/inspector.rs#L21); [`crates/fm/src/ui/content.rs:114`](../../crates/fm/src/ui/content.rs#L114)

**Problem and consequence.** grid_columns subtracts a fixed sidebar/padding budget from outer viewport width, ignores inspector width and enforces at least four columns. Narrow windows and an open inspector can overflow or clip content; wider windows are capped at eight columns regardless of space.

**Required direction.** Measure available collection viewport from layout constraints. Derive columns from minimum tile width and supported density, with sensible one-column behavior and a defined max only if justified.

**Acceptance evidence.** Resize with inspector open/closed across minimum to wide widths. No clipped tiles, inaccessible controls or horizontal overflow except intentionally scrollable regions.

<a id="v08"></a>

### V08 — Column-count changes may retain incompatible grid rows

**Priority:** P1 · **Evidence:** Strong code-derived risk

**Source:** [`crates/fm/src/ui/content.rs:115`](../../crates/fm/src/ui/content.rs#L115); [`crates/fm/src/ui/content.rs:69`](../../crates/fm/src/ui/content.rs#L69); [`crates/fm/src/model.rs:1521`](../../crates/fm/src/model.rs#L1521)

**Problem and consequence.** Grid rows capture column count during composition; the walk expression explicitly observes listing_revision and view, not viewport. Row keys depend on grid_columns but dependency tracking through key evaluation is not established. A resize may change model math without rebuilding all affected row contents.

**Required direction.** Make layout inputs explicit reactive dependencies and key identity independently from layout geometry; prefer a layout primitive that handles wrapping from measured bounds.

**Acceptance evidence.** Resize repeatedly without changing files and verify child count, positions, identities, grid height and hit boxes, including partial final rows.

<a id="v09"></a>

### V09 — Popover positioning is a literal coordinate, not an anchor

**Priority:** P1 · **Evidence:** Confirmed layout limitation

**Source:** [`crates/fm/src/ui/options.rs:44`](../../crates/fm/src/ui/options.rs#L44); [`crates/fm/src/ui/options.rs:45`](../../crates/fm/src/ui/options.rs#L45); [`crates/fm/src/model.rs:551`](../../crates/fm/src/model.rs#L551)

**Problem and consequence.** Options always appear at 610,46, disconnected from the actual settings button after toolbar padding/titlebar changes. Menu position clamps only when opened; later viewport resize does not re-anchor an already open panel.

**Required direction.** Provide AnchorPlacement with flip/shift, safe insets, RTL support and an overlay coordinate space. Recompute on layout/scroll/resize while preserving focus and pointer ownership.

**Acceptance evidence.** Open near every edge, resize and move the anchor; popup stays associated with the invoker and within the supported app viewport.

<a id="v10"></a>

### V10 — Elevation is copied as per-instance paint plumbing

**Priority:** P2 · **Evidence:** Confirmed design-system gap

**Source:** [`crates/fm/src/ui/chrome.rs:225`](../../crates/fm/src/ui/chrome.rs#L225); [`crates/fm/src/ui/menu.rs:107`](../../crates/fm/src/ui/menu.rs#L107); [`crates/fm/src/ui/dialog.rs:64`](../../crates/fm/src/ui/dialog.rs#L64); [`crates/fm/src/ui/status.rs:143`](../../crates/fm/src/ui/status.rs#L143)

**Problem and consequence.** Repeated OffscreenRender/DropShadow entities use literal 10px blur, 3px offset and opacity 80 across different roles. A clear toolbar button receives similar treatment to a dialog; shadow ordering and visibility are hand-managed. There is no semantic hierarchy or performance budget.

**Required direction.** Define Surface/Elevation roles: flat chrome, raised control when warranted, popover, dialog and floating window. Centralize border, ambient/key shadow, clipping and shadow hit-test exclusion. Shadow must communicate structure, not decorate every widget.

**Acceptance evidence.** A gallery shows all elevation levels in both themes with consistent stacking, no hidden-surface shadows and measured renderer/memory cost.

<a id="v11"></a>

### V11 — Toolbar grouping should prioritize current-location tasks

**Priority:** P2 · **Evidence:** Design recommendation grounded in code

**Source:** [`crates/fm/src/ui/chrome.rs:19`](../../crates/fm/src/ui/chrome.rs#L19)

**Problem and consequence.** Back/forward, fixed-width location, command discovery, large New Folder text, view switch, search, options, inspector and theme are one fixed-height sequence. Global preferences and occasional command discovery compete with content actions; fixed widths leave little resilience for text expansion.

**Required direction.** Use navigation/location, content actions/view, search, then contextual pane controls as explicit groups. Place global preferences in an app menu/settings surface. Evaluate a labelled icon action or overflow for creation; do not make it prominent merely because it was criticized.

**Acceptance evidence.** Review at supported widths with long location names, keyboard-only use and translated labels. Users identify location, search and available content actions without scanning unrelated global controls.

<a id="v12"></a>

### V12 — Window-control placement needs a host policy, not Apple imitation

**Priority:** P2 · **Evidence:** Design recommendation grounded in code

**Source:** [`crates/fm/src/ui/chrome.rs:63`](../../crates/fm/src/ui/chrome.rs#L63); [`crates/fm/src/ui/mod.rs:57`](../../crates/fm/src/ui/mod.rs#L57)

**Problem and consequence.** The internal titlebar is 22px and the title follows a growing spacer, so it is trailing rather than intentionally centered. Host decorations can coexist with internal traffic lights. Moving lights into the sidebar without window-action/drag-region contracts would only relocate the same false affordance.

**Required direction.** Support native decoration, custom integrated header and preview modes. Let Naledzi’s window manager choose action order, drag regions, maximize behavior and inactive styling. Merge sidebar/header only if navigation alignment and usable drag space improve.

**Acceptance evidence.** No duplicate system controls; titles remain meaningful at narrow width; host move/resize and accessible window actions work in each supported decoration mode.

<a id="v13"></a>

### V13 — Spacing, radius and density are literals rather than coordinated policies

**Priority:** P2 · **Evidence:** Confirmed code gap

**Source:** [`crates/fm/src/ui/kit.rs:20`](../../crates/fm/src/ui/kit.rs#L20); [`crates/fm/src/ui/chrome.rs:105`](../../crates/fm/src/ui/chrome.rs#L105); [`crates/fm/src/ui/dialog.rs:112`](../../crates/fm/src/ui/dialog.rs#L112)

**Problem and consequence.** Padding helpers reduce syntax but do not express semantic spacing. Heights 18,20,22,24,26,28 and radii 4..12 vary by widget without a shared baseline/density contract. A host cannot offer comfortable controls without editing each component.

**Required direction.** Provide a small spacing scale, control-height and icon-size families, focus clearance and compact/comfortable density. Use semantic layout roles rather than a token for every arbitrary number.

**Acceptance evidence.** Switch density once in a gallery and two real apps. Alignment, hit areas, text metrics and keyboard focus remain consistent without per-screen patches.

<a id="v14"></a>

### V14 — Long text and localization are not part of component contracts

**Priority:** P1 · **Evidence:** Confirmed code gap

**Source:** [`crates/fm/src/ui/dialog.rs:19`](../../crates/fm/src/ui/dialog.rs#L19); [`crates/fm/src/ui/palette.rs:57`](../../crates/fm/src/ui/palette.rs#L57); [`crates/fm/src/ui/status.rs:92`](../../crates/fm/src/ui/status.rs#L92); [`crates/fm/src/data.rs:299`](../../crates/fm/src/data.rs#L299)

**Problem and consequence.** Fixed dialog/notification widths and heights, English literals and bytewise ASCII case folding do not accommodate translated labels or Unicode search. RTL direction, formatting locale and long filenames have no explicit policy. The font charset is a finite baked subset, not a fallback chain.

**Required direction.** Make strings app resources, layout direction explicit, metadata locale-aware and text overflow recoverable. Use measured wrapping/min-max layouts and a documented Unicode/fallback strategy without claiming an external renderer is required.

**Acceptance evidence.** Exercise long labels, CJK/combining/RTL names, non-ASCII case pairs and text-scale changes. Names remain selectable/copyable and search behavior is documented and predictable.

<a id="v15"></a>

### V15 — Icon semantics and color policy are split across implementations

**Priority:** P2 · **Evidence:** Confirmed code gap

**Source:** [`crates/fm/src/data.rs:53`](../../crates/fm/src/data.rs#L53); [`crates/fm/src/ui/kindicon.rs:47`](../../crates/fm/src/ui/kindicon.rs#L47); [`crates/fm/src/ui/hover.rs:72`](../../crates/fm/src/ui/hover.rs#L72); [`crates/fm/src/ui/menu.rs:175`](../../crates/fm/src/ui/menu.rs#L175)

**Problem and consequence.** Kinds use raw tints outside theme roles; selected and grid icons use different implementations. Menu icons are chosen by English label string. Selected preview resolves None as file ID 0, showing an unrelated file icon beside “Nothing selected”.

**Required direction.** Introduce semantic IconId and kind-color roles with theme/high-contrast resolution; pass optional item identity explicitly and render an intentional no-selection surface. Map command icons by command identity, not label text.

**Acceptance evidence.** Changing locale cannot change icons. Empty selection shows no unrelated file. Same kind and size have consistent geometry/tint in grid, list, menu, inspector and drag preview.

<a id="v16"></a>

### V16 — Tooltip and skeleton names overstate their available behavior

**Priority:** P2 · **Evidence:** Confirmed code gap

**Source:** [`crates/fm/src/ui/kit.rs:181`](../../crates/fm/src/ui/kit.rs#L181); [`crates/fm/src/ui/kit.rs:139`](../../crates/fm/src/ui/kit.rs#L139); [`crates/fm/src/lib.rs:46`](../../crates/fm/src/lib.rs#L46)

**Problem and consequence.** tooltip is a static visible bubble without trigger association, delay, placement, keyboard reveal or dismissal. Skeleton view and animation systems exist but are not registered by install_views. Calling these helpers does not establish complete reusable components.

**Required direction.** Publish complete Tooltip and LoadingSurface contracts, register views through a coherent kit plugin and honor reduced motion. Avoid shipping nominal components that require undocumented manual wiring.

**Acceptance evidence.** A second app uses each component through the public installation path only; tooltip works by hover/focus and loading can switch to static placeholders without motion.
