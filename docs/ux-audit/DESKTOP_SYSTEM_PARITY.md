# Desktop system parity and shell decision

2026-10-24 · implementation branch `arena/b695798f-mirui-kit`

## Product decision: compact sidebar-aligned window chrome

**Yes: controls in the sidebar-aligned titlebar are appropriate desktop UX.**
This is a recommendation for this system, not a claim that every platform must
put its caption controls on the left. Reserve a stable leading caption area;
place the sidebar toggle to the right of the real window controls. Navigation
scrolls below that cap. Hiding the sidebar must not hide or move the controls,
remove the toggle, destroy navigation state, or turn button hits into window
movement. A narrow window collapses navigation before it destroys useful content.
Respect system-owned decorations and host-reserved control bounds. Never draw a
second set over system controls. A custom OS may choose its leading arrangement;
Windows/macOS adapters must respect their host's conventions and capabilities.

The content toolbar and status region belong to the content pane, not the
sidebar. Sidebar workspace/account controls belong to the sidebar. Window title,
workspace-level commands and content-level search are different scopes. A command
palette does not replace ordinary menus or discoverable actions. Keep Ctrl
shortcuts for this OS; adopting spatial polish does not imply adopting Cmd.

Caption buttons must represent real host operations and confirmed state. Close
runs the unsaved-work guard before requesting closure; maximize and restore
reflect the compositor/window manager, not an optimistic local boolean. Empty
caption space may be a drag region; inputs, tabs, actions, selected text,
dividers and menu surfaces must be excluded. Preserve double-click/window menu
behavior only where the host contract actually supplies it.

## Authoritative research, inspected again

- GPUI Kit source inventory at commit
  `69a2077c2da588de1c46b89ffd903e1373e45a29`,
  https://github.com/longbridge/gpui-kit/tree/69a2077c2da588de1c46b89ffd903e1373e45a29/crates/component/src
- GPUI desktop design guidance: https://gpui-kit.com/docs/design-guides/
- GPUI caption ownership: https://gpui-kit.com/component/title-bar/
- GPUI navigation structure: https://gpui-kit.com/component/sidebar/
- Current shadcn component index: https://ui.shadcn.com/docs/components
- Material Expressive: https://m3.material.io/blog/building-with-m3-expressive
- Material density: https://m3.material.io/foundations/layout/grids-spacing/density
- Apple sidebar: https://developer.apple.com/design/human-interface-guidelines/sidebars
- Apple toolbar guidance: https://developer.apple.com/design/human-interface-guidelines/toolbars
- Apple AppKit grouping, sidebar/inspector and corner guidance:
  https://developer.apple.com/videos/play/wwdc2025/310/

These are complementary references, not four interchangeable themes. GPUI
contributes native desktop workflows and complex widgets; shadcn contributes
composable stateful families; Material contributes semantic token/state/density
and adaptive principles; HIG contributes hierarchy, direct manipulation and
window/pane behavior. Keep one visual language. Restrained productivity motion
and compact information layout are the default; expressive shape/motion is
selective and must support reduced motion. Material's touch-target guidance must
not be represented as permission to shrink every target to fit a compact mockup.
Density changes spacing, not the user's chosen readable text size.

## Source/visual audit: consequential defects, not just missing names

| Area | Current finding | Required resolution |
| --- | --- | --- |
| Window controls | Files `traffic_lights()` draws three non-interactive 12px Views. They are decoration, not controls. Its 22px titlebar is not a complete caption/drag contract. | Real host-backed control family, close guard, system/client-decoration negotiation, hit regions and confirmed state. Do not advertise these dots as implementation. |
| Shell containment | Files has a whole-window chrome block; Studio's notice spans beneath the sidebar. | Adopt pane-aligned toolbar/status, permanent caption cap and sidebar toggle without deleting header features. |
| Slot geometry | Native named slots have Auto hosts. Percentage-filled split content measured zero width despite visible overflowing text. | Shared native `layout::FillSlot` supplies definite bounds. A visible screenshot alone does not establish correct containment. |
| Shared collections | Fresh native Collections now shows aligned tree labels/table columns and wrapped guidance. | Finish header keyboard paths, modifier/typeahead input, reveal, virtual rendering, column gestures and editing. |
| Focus/selection | Some specimen rows retain focus-style outlines independently of current native composite focus. | Separate active descendant, selection, keyboard focus and inactive-window state across all collections. |
| Native typography | Static Inter faces; explicit medium/semibold literals remain in many compositions. | Themeable semantic roles across chrome, menus, fields, dialogs and collections. Do not invent variable-font support. |
| Overlays | Tested policy stack is not wired to binary InputGate/focus ownership. | One actual nested overlay service: modal, nonmodal, nested menus, restoration, dismissal, pointer/drag ownership and portal anchoring. |
| State presentation | Several disabled collection items reject model changes without full native input-state treatment. | Unified pointer/keyboard/native-disabled/semantics behavior; no deceptive active hover. |
| Feedback | Basic shared alert/progress exists; no full shared notification surface/executor bridge. | Bounded lifecycle, cancellation, stale completion, retry, announcements, determinate/indeterminate distinction. |
| Elevation/shape | Native shadow source exists; six base captures do not establish all popup/dialog shadow quality. | Review actual native silhouette, clip, border, shadow and focus together; purposeful elevation roles, no blanket rounding. |
| Editing | Single-line native editing exists; it is not a multiline/code editor or complete shared inspector. | Selection actions, IME, clipboard, undo transactions, inline naming, error recovery and complex editor contracts. |
| Runtime lifecycle | Green small regressions do not explain earlier freeze/spin reports. | Audit observer ownership, timers, idle invalidations, retained branches, unmount/cancel and collection allocation behavior. |

## Required parity surface (all are in scope)

A family is not complete merely because a primitive, policy or specimen exists.
Parity means shared API, coherent state/behavior, actual composition, theme and
host contracts, not identical renderer/framework APIs or copied visual styling.

1. **Foundation:** semantic color/type/shape/spacing/elevation/motion/density;
   theme scopes; reduced motion; DPI versus text scale; RTL and localization;
   focus scopes/restoration; command routing/keymaps; semantics/action bridge;
   lifecycle, cancellation, resource ownership and safe bounded scheduling.
2. **Actions:** Button/IconButton/Link/SplitButton; ButtonGroup/ToggleGroup;
   radio/check/switch; shortcut labels; tooltip/hover card; disabled, read-only,
   pending, destructive, repeat and default/cancel-action contracts.
3. **Forms:** label/help/error/required associations; input/input group/search;
   multiline/password/OTP/masked entry; integer/decimal/unit spinbox; slider,
   range slider; select/multiselect/combobox/autocomplete/token entry; date,
   range, time and color pickers; form validation/submission/error summary.
4. **Navigation/chrome:** sidebar sections/nesting/counts/rail/collapse;
   titlebar/real caption controls/drag exclusions; toolbar/overflow/customization;
   status regions; breadcrumb; tabs/document tabs/overflow/reorder/close/dirty;
   menubar, application/menu/context commands, pagination, navigation history.
5. **Layout:** stack/grid/wrap/insets; cards/panels/group box/description list;
   disclosure/accordion; divider; scrolling, scrollbars, reveal and restoration;
   horizontal/vertical split panes; master-detail; inspectors; docking and
   document workspace; minimum-size/overflow/responsive rules.
6. **Collections:** list/item/listbox/searchable list/grid/tree/treegrid/table;
   stable identity, selection versus focus, modifier ranges, disabled items,
   keyboard/typeahead, sorting/filtering/grouping; variable-height virtualization;
   row/cell editing, resize/reorder/pin columns; direct manipulation/drop targets
   with autoscroll and cancellation; empty/loading/error/partial states.
7. **Surfaces:** popover/dropdown/context/submenu; tooltip/hover card; dialog,
   alert/confirmation, sheet/drawer, inspector and command palette; nested
   ownership, anchor flip/clamp, Escape/outside press, return focus and guards.
8. **Feedback/content:** alerts, badge/tag/chip/avatar, skeleton/shimmer/spinner,
   progress/stepper, toast/history/banner; image/aspect ratio/attachment/carousel;
   rich text/link/code/diff/markdown; charts/plots/rating and accessible summaries.
9. **Complex apps:** settings/form/questionnaire composition, message/scroller,
   streamed task results, terminal/editor hooks, attachment transfer, undo/history,
   inspector/debugging and discoverable command/search infrastructure. Expensive
   specialty renderers may be optional modules, not app-local reinventions.
10. **System workflows:** capability-denied/revoked/offline states; open/save,
    conflict/overwrite, permission, unsaved, destructive and recovery dialogs;
    file/provider selection, clipboard/drag offers, notifications, session/window
    restoration, accessibility and automation. Host operations are not simulated
    by widgets silently changing local labels.

Each family needs default/hover/pressed/focused/selected/disabled/read-only/
loading/error states where meaningful, light/dark and user scale, narrow/wide
windows, keyboard and pointer paths, localized long content and teardown rules.
This requirement applies to everything above, not only the named controls.

## Smart Protocol boundary: inspect before inventing

Inspected `smart-foundation/smart-protocol-official` at commit
`9175b06bb7b2ddee87af4dd12aa02f07a4d3402f`. The actual contract is Cap'n Proto
schemas, not an imagined HTTP/JSON endpoint. The separate `smart-protocol-rs`
repository was empty when inspected; do not claim a usable generated Rust SDK.

- `schema/windowmanager.capnp`: `WindowManagerSession.handshake`; opaque typed
  `WindowId`; `closeWindow`, `minimizeWindow`, `restoreWindow`, `maximizeWindow`,
  `moveWindow`, `resizeWindow`, `moveResizeWindow`, fullscreen and event streams.
- `schema/display.capnp`, `input.capnp`, `clipboard.capnp`,
  `accessibility.capnp` are distinct existing contract surfaces. Surface/text
  input references in audit documents must be checked against actual schemas;
  an audit filename is not itself a shipped interface.
- The adapter must acquire permitted sessions, retain opaque identities,
  preserve structured result/error/unknown data and handle revocation. It must
  not invent a parallel permission/token scheme inside every component.
- Protocol transport/service calls stay out of layout and render callbacks.
  Widgets issue typed semantic intents; adapters reconcile confirmed results
  and events on the UI owner. Cancellation, bounded queues and late-result
  rejection belong to the lifecycle contract, not ad hoc timers in each app.
- Mirui remains the native UI/render/input foundation and the core remains
  no_std. Protocol encoding/runtime/transport is an integration module, not a
  reason to substitute a web UI or require std throughout the kit.

## Implemented in this continuation versus still outstanding

Native table/tree alignment and wrapping fixes are implemented and visually
confirmed. Native conditional/scaled content sizing and retained table updates
passed on `887dac3`. A shared native SplitPane, Workbench consumer and native
input/layout checks were added; those exposed the named-slot containment defect.
The correction is now generalized as `layout::FillSlot`. The new
`layout::{DesktopChrome,DesktopFrame}` supplies persistent sidebar-cap slots,
model-backed collapse, content-local toolbar/status and a separate overlay slot.
It is structural source, not a claim that Files/Studio already have complete
protocol-backed window chrome or that the entire parity surface is finished.

**The core is still incomplete.** Research, green CI and a larger catalog must
never be substituted for completion of the families and interactions above.


### Live compact-shell implementation

Studio now uses the shared `layout::sidebar_toggle` and a per-window
`DesktopChrome` model. Hiding the sidebar preserves its mounted navigation;
its toggle stays in the non-scrolling header. Header height is 48 logical pixels
at default text scale, scales with text, and retains the title, palette and Help.
The status/notice region is now inside the content column. The separate
DesktopFrame caption slots are available for real host controls; this does not
turn the older Files dots into functioning window controls. The shared toggle
uses a visible label until tooltip/host-semantics integration can support an
icon-only replacement without losing discoverability.
