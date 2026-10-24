# Actual Files shell audit — 2026-10-24

The supplied OS screenshot is consistent with the current Files implementation
at merge `348199c`; it is not evidence that the user pulled an old repository.
Previous fixes/captures targeted Studio and shared kit specimens. Files retained
its independent shell. Passing those tests did not establish Files shell parity.

## Source-backed diagnosis before the changes

- `fm/src/ui/mod.rs` placed the sidebar below an 84px floating chrome wrapper,
  with 24px horizontal insets. Thus the sidebar could not occupy the full left
  edge. Its status bar also spanned the whole app rather than the content pane.
- `fm/src/ui/chrome.rs` rendered inert red/yellow/green Views inside an already
  decorated OS window, duplicating the real host controls. No sidebar toggle
  was mounted. Toolbar groups depended on fixed widths and a 240px path column.
- `fm/src/ui/sidebar.rs` gave the label intrinsic flex width; `ui/kit.rs` put
  count text in an unmeasured Auto wrapper. Counts could paint beyond their
  reserved width, right against the divider.
- `FmState::grid_columns` clamped to 4..8 and estimated space from the entire
  window minus a fixed sidebar. It ignored the inspector and could not use
  additional horizontal room. Tiles stayed fixed at 104px.
- The interactive SDL host starts at 1400×900. Mirui 0.51's SDL builder does not
  request a resizable window and its framebuffer dimensions are fixed. This is
  a separate host limitation, not something a layout percentage can fix.

## Corrections in this branch

- Edge-aligned sidebar caption cap and body share their 216px width and colour;
  the body continues to the window bottom. Status belongs to the content column.
- A visible, functioning Sidebar toggle is retained in the non-scrolling cap.
  Inert traffic lights are removed. Real OS window controls remain host-owned;
  moving those into the cap requires a genuine host/Smart Protocol bridge.
- Count pills use native text intrinsic measurement; labels have a zero flex
  basis and counts reserve non-shrinking width with an explicit right inset.
- Grid columns follow the actual measured pane, including inspector and sidebar
  changes, allow fewer than four columns, and are no longer capped at eight.
  Cells share the remaining row width; unused last-row slots are noninteractive.
- Below 1180px, navigation and search/view/Actions occupy two rows. Actions opens
  the existing full palette, preserving New Folder, appearance and inspector
  commands instead of forcing them into a clipped row.
- Desktop initial size accepts `--width` / `--height`. This is not a claim that
  the pinned SDL backend now supports live resize; that needs buffer, texture,
  display-info and event integration together. An OS host must publish its real
  logical viewport rather than an unrelated fixed size.

## Verification and remaining boundaries

New native Files geometry contracts exercise 640×600, 1180×720 and 1400×900,
count insets, search containment, sidebar collapse and measured grid columns.
A native capture binary renders those sizes in both themes and an inspector
case. CI exports the actual 1400×900 Files PNG, not a Studio surrogate.

The existing 32-item icon-grid rendering cap remains deliberately unchanged.
It explains the “32 of 512” status and unused vertical area: responsive layout
alone does not provide virtualised access to all 512 entries. Removing the cap
without bounded rendering would risk the prior performance/freeze regression.
Proper virtualisation or explicit paging remains open, along with true host
controls, live SDL resize, and complete shared-frame migration. This patch must
not be described as full OS-suite completion.
