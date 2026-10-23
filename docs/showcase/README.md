# Naledzi Studio — showcase application specification

**Status:** implementation-ready design specification, not an implemented showcase.
The existing executable is still Files. Markdown wireframes below are screen/state
specifications, not screenshots of software that has been built.

Studio is a compact, useful app for trying, inspecting and composing the **real
Mirui-kit components**. Its shell, navigation, search, forms, overlays, commands,
feedback and inspector use those same components. It is not a custom gallery skin
with decorative replicas that behave differently from the app suite.

A small creative workspace makes controls meaningful: adjust an export preset,
organize sample assets, edit a note and inspect a component’s state. Safe local
fixtures make mistakes recoverable and tests reproducible. Simulation is labelled;
there is no pretend disk, upload or OS operation.

## Read the collection

| Document | Deliverable |
|---|---|
| [Product and information architecture](01-product-and-navigation.md) | Scope, compact shell, routes, navigation and progressive disclosure |
| [Visual and component contract](02-design-and-dogfooding.md) | Tokens, type, spacing, component ownership, accessibility and state rules |
| [Overview screen](screens/01-overview.md) | Useful starting workspace and component discovery |
| [Controls screen](screens/02-controls.md) | Real form controls in a working export recipe |
| [Collections screen](screens/03-collections.md) | Complete, identity-safe list/grid interactions |
| [Workbench screen](screens/04-workbench.md) | Document-like editing and unsaved-work flow |
| [Surfaces screen](screens/05-surfaces.md) | Menus, dialogs, notifications and deliberate layering |
| [Foundations screen](screens/06-foundations.md) | Appearance/type/density with readable live examples |
| [All overlays and transient states](03-overlays-and-transient-states.md) | Palette, inspector, menus, dialogs, tooltips, diagnostics and failure wireframes |
| [User flows and state machines](04-flows-and-state.md) | Mouse/keyboard/assistive flows, invariants and persistence |
| [Component coverage and real implementation](05-components-and-api.md) | Source-backed inventory, gaps, build boundaries and developer workflow |
| [Robustness and acceptance](06-acceptance-and-delivery.md) | Scenario fixtures, release gates, screenshots and implementation sequence |
| [Deeper UX review](07-critical-review.md) | Failure-oriented critique and explicit defenses against an impressive but unusable demo |

Every top-level route has a dedicated wireframe. Every overlay family has a
wireframe in the overlay document. Loading/empty/error/disabled/pending/compact
variants have explicit contracts, not an assumption that the happy-path image is enough.

## Screen map

```text
Studio (real AppShell)
  Overview       S01  resume, discover and reset safe experiments
  Controls       S02  build a validated export recipe
  Collections    S03  organize complete sample-asset collections
  Workbench      S04  edit a local note; test save/failure/cancel
  Surfaces       S05  exercise overlays in controlled tasks
  Foundations    S06  inspect/apply global appearance policy

Global surfaces (not extra navigation pages)
  O01 command palette      O02 component inspector / recipe drawer
  O03 context menu         O04 anchored options popover
  O05 confirmation dialog  O06 validation/conflict dialog
  O07 notification history O08 diagnostics drawer
  O09 tooltip              O10 unsaved-changes dialog
  O11 compact navigation   O12 reset confirmation
  O13 component catalog    O14 help / keyboard / About
  O15 appearance recovery  O16 sample preview
  O17 move chooser
```

## Non-negotiable implementation rules

1. Studio imports shared kit components; it does not duplicate them locally.
2. A component is not advertised as ready until its behavior is real and tested.
3. Icons accompany command labels; icons never replace an accessible name.
4. No control silently fails, fakes success or changes a different object.
5. Compact means focused composition, not tiny text or hidden essential actions.
6. One overlay host and command registry serve both app chrome and specimens.
7. A lab changes only its fixture state unless the UI explicitly says “Apply to Studio”.
8. Changes to global appearance use a rollback-safe preview; reset never discards work silently.
9. Keyboard, focus, cancellation, long content and failure states are design inputs.
10. “Flawless” is an acceptance goal, not a claim made by this document.

## Relationship to the current code

The [full UX audit](../ux-audit/README.md) documents the current Files prototype.
Studio is the second consumer that should prove extraction of the reusable kit.
Do not copy FmState, its flat fake filesystem or its fixed global IDs into Studio.
The [component inventory](05-components-and-api.md) distinguishes existing source,
required adapters, incomplete helpers and proposed capabilities.

This turn also changes the existing Files palette to use semantic leading icons
and registry-driven rows. That patch is uncompiled/unrendered in this environment.
It is not evidence that the future Studio palette or its keyboard behavior exists.
