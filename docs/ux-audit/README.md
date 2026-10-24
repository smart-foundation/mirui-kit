# Mirui-kit: full code-led UX and app-foundation audit

**Audit date:** 2026-10-23  
**Product goal:** a robust, readable, professional foundation for building Naledzi OS apps quickly.  
**Verdict:** promising rendering/composition demonstration; **not yet a reusable production app kit**.

This audit does not equate “premium” with more blur, thin text, Apple controls or
more components. It evaluates whether people can understand and complete tasks,
whether the UI tells the truth, and whether developers can reuse correct behavior
without copying file-manager internals.

## Current component coverage verdict

The kit is **not OS-suite ready**. The updated [actual component catalog](../COMPONENT_CATALOG.md)
separates shared partial APIs, app-local implementations, native Mirui primitives
and missing components. Eight catalog groups and 17 showcase surface IDs do not
establish completeness. ButtonGroup and TreeView are unimplemented shared controls,
not visual polish items. The visual-shell review does not supersede this verdict.

## Read these reports

| Report | Purpose |
|---|---|
| [01 — Code defects and interaction integrity](01-code-defects.md) | 42 findings about actual workflows, identity, collections, controls and input |
| [02 — Visual system and accessibility](02-visual-accessibility.md) | 16 findings on typography, contrast, layout, focus, elevation and adaptation |
| [03 — Architecture and developer experience](03-architecture-developer-experience.md) | 18 findings on reuse, service boundaries, platform safety and delivery |
| [04 — Timeless kit blueprint](04-timeless-kit-blueprint.md) | Required contracts, component inventory, app patterns and fast developer workflow |
| [05 — Validation and roadmap](05-validation-and-roadmap.md) | Existing test gaps, invariant tests, user journeys, release gates and ordered work |
| [06 — Evidence and coverage](06-evidence-and-coverage.md) | Audit source inventory, measured probes, historical image observations and limits |
| [Finding register](FINDINGS.md) | All 76 finding IDs, priorities and links in one place |
| [2026-10-24 — Visual shell review](VISUAL_SHELL_REVIEW.md) | Audit-first corner/icon/font/shadow and sidebar/content layout decisions; five additional issues and five proposed additions |

The earlier [Naledzi readiness note](../NALEDZI-READINESS.md) remains a historical
baseline. These reports extend it with source-level evidence and detailed contracts.
The current documents do not supersede code with assumptions from previous agent
reports. They inspect the working checkout, including its unverified input patch.

## Executive findings

### 1. Data trust is currently weaker than visual appearance

The same moved built-in file can be appended twice to a destination.
Created folders use a different location/trash path from built-in files.
Created identity can collide after a provisional item is removed.
Duplicate turns non-folder entries into folders.
Date and size sort compare display strings rather than typed metadata.
A successful drag does not recompute the collection.
These faults make a beautiful UI untrustworthy; fix them before extraction.

### 2. The collection views do not expose the complete collection

The current grid renders at most 32 items, or 12 under search.
The list renders at most 48 rows while status can claim all results are shown.
This is capped rendering, not virtualization.
A stable app kit must solve full reachability without making input identity stale.
The list’s index-key/captured-ID split is a particularly serious action-target risk.

### 3. Several visible capabilities are incomplete or misleading

The slider handler discards changes; hidden-files state is not used in filtering.
Forward and internal traffic lights do not perform their apparent actions.
Menu Open does not open a file/folder; grid double-click opens Info instead.
The palette defines 17 commands but constructs only 16 rows.
Inspector tabs and storage data are not wired as real features.
These are not polish-only issues.

### 4. Previous input fixes are a starting patch, not a routing architecture

The working tree contains a backdrop/drag-latch change from the preceding turn.
It has not been compiled or exercised here.
A sampled content guard does not prove correct hover, wheel, focus and gesture
ownership at dispatch time. Overlay combinations are still contradictory.
The kit needs one overlay/focus/capture policy, not more per-widget guards.

### 5. Text readability is not solved by bundling Inter

The code has three sizes and physical weight registrations, not a full semantic
role system. Small regular text is used in many essential controls.
Calculated token pairs show insufficient normal-text contrast for primary buttons,
selected sidebar text, light badges and the light tooltip under the chosen 4.5:1
benchmark. Renderer glyph quality is a separate question requiring fresh images.

### 6. The kit abstraction has not yet been built

The workspace contains a Files application with useful helpers.
Checkbox, view switching, notifications and installation depend on FmState.
Fonts, theme, model and root creation are installed together with global IDs.
A Settings or Editor developer would need to copy or untangle application code.
The right extraction is shared contracts proven by a second app—not renaming fm
and exporting every current helper as a stable API.

### 7. Naledzi runtime safety still needs explicit engineering

The smoke-target allocator does not satisfy alignment requirements.
Its spin-lock critical section is not a demonstrated interrupt-safe contract.
The framebuffer-size arithmetic already exceeds the configured heap if that
surface needs a full heap-backed RGB565 frame; allocation strategy needs proof.
Filesystem operations and host input/actions are fixtures/SDL-specific, not a
production Naledzi adapter. `no_std` compilation alone cannot certify an OS app.

## What is worth preserving

- `no_std` + `alloc` separation and a distinct bare workspace.
- An existing Mirui renderer/composition path rather than a custom replacement.
- Inter assets with explicit weight features and an existing bake pipeline.
- Semantic color tokens as a starting point for role-based appearance.
- A pure model direction that can support deterministic fixture tests.
- Stable-ID intent in file selection and some keyed grid construction.
- Headless rendering, pointer-scripted tests and performance diagnostic examples.
- The aspiration to exercise real app flows before freezing an OS-wide kit.

Preserve these strengths while correcting contracts. Do not discard the project
because it is incomplete, and do not promote demo shortcuts into OS-wide standards.

## Scope and evidence rules

Inspected application code includes model/data, fonts/themes, all UI modules,
synchronisation, public installation and host entry points.
Inspected verification includes data/features/interaction/appearance/typeface
suites, snapshot generation, performance examples and build/font/icon tooling.
Evidence inventory links and source hashes are in report 06.

Evidence classifications:

- **Confirmed code defect/gap:** directly present or absent in the inspected code.
  This does not claim successful runtime reproduction.
- **Strong code-derived risk:** a concrete failure path whose manifestation also
  depends on Mirui lifecycle, retention or event-order semantics not verified here.
- **Measured token/budget fact:** reproducible arithmetic from checked-in values,
  with assumptions stated. Not a full rendered/target certification.
- **Design recommendation:** a proposed product contract, not an alleged engine bug.
- **Historical visual observation:** visible in a saved artifact; not proof of the
  current checkout after later edits.

The Mirui dependency source is not vendored or installed in this environment.
Upstream behavior described in comments/README is not independently verified.
Do not attribute an application bug to Mirui without a reduced dependency-level test.

## Severity

| Priority | Meaning | Release treatment |
|---|---|---|
| P0 | Identity/action integrity, unsafe runtime foundation or equivalent severe risk | Resolve or disable affected capability before production exposure |
| P1 | Broken core flow, access/readability barrier, false capability or foundational reuse gap | Required for a usable supported release of that capability |
| P2 | Important consistency, efficiency, discoverability, maintainability or polish debt | Prioritize by journey impact; not “optional forever” |

Priorities are engineering triage, not a security exploit assessment.
A conceptual architecture gap can be P1 without implying a crashing program.
Do not add priorities together as an invented numerical quality score.

## Recommended decision order

1. Restore object identity, collection and operation truth.
2. Prove input routing and basic control behavior.
3. Establish semantic accessibility/readability and measured layout.
4. Finish coherent workflows, including complete content reachability.
5. Extract a kit with two unrelated app consumers.
6. Integrate and harden actual Naledzi adapters on target hardware.

Visual work should follow a shared layout/type/surface contract, not wait until all
platform work is done. But do not independently restyle every widget while its
behavioral contract and data target remain wrong.

## Explicit non-claims

This audit does not claim the reported freeze is resolved.
It does not claim fresh screenshots, compiled tests or a working Naledzi port.
It does not certify accessibility or font smoothness from source inspection.
It does not prescribe replacing Mirui, adopting a heavyweight runtime or copying
macOS window conventions. It does not treat earlier agent completion notes as proof.

The audit is comprehensive across the checked-in app-foundation surface, not a
promise that all possible runtime defects or upstream bugs have been discovered.
Acceptance evidence in each finding defines what is needed to close it.

## Showcase follow-up

The [Naledzi Studio specification](../showcase/README.md) translates the kit goal
into six task-based screens and a shared-component implementation contract. Its
[critical review](../showcase/07-critical-review.md) adds deeper discoverability,
inspection, scope, feedback and lifecycle checks. The Files palette now has a
semantic-icon/registry-row patch; F20 remains patched but unverified, not closed.
The audit evidence inventory is the earlier snapshot, not a hash of this later patch.
