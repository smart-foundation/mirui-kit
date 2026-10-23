# S03 — Collections: a compact sample-asset workspace

**Route:** `/collections` · **Primary task:** find and organize sample assets.  
**Domain:** stable fixture objects with real in-memory hierarchy and operations;
not Files’ capped flat Place vectors and not a real disk browser.

## Default wireframe

```text
+------------------------------------------------------------------------+
| Studio      [Find components…                     ] [Commands] [Help]   |
+---------------+--------------------------------------------------------+
|   Overview    | Sample workspace                            [Inspect]  |
|   Controls    | [Back] Samples / Illustrations                         |
| > Collections | [New folder]  [Grid|List] [Search sample assets…] [More]|
|   Workbench   |                                                        |
|   Surfaces    | [folder] Drafts    [folder] Ready    [image] Aurora      |
|   Foundations | [image] Coast     [image] Notes    [doc] Readme         |
|               |                                                        |
|               |             complete scrollable collection            |
|               |                                                        |
|               | 6 items · 1 selected                       [Selection] |
+---------------+--------------------------------------------------------+
| Fixture workspace · Local session only                      [Status]   |
+------------------------------------------------------------------------+
```

Toolbar relates to current collection only. Commands remains global in the header.
New Folder uses a quiet labelled icon button, not a large colored primary callout.
The collection owns one search scope, view switch and overflow; no duplicate global
search semantics. Item metadata is genuine fixture data, not decorative strings.

## List variant

```text
| [New folder]  [Grid|List] [Search sample assets…] [More]          |
| Name [ascending]            Kind          Size       Modified   |
| [folder] Drafts             Folder        —          Today      |
| [folder] Ready              Folder        —          Yesterday  |
| [image] Aurora              Image         42 KB      22 Oct     |
| [doc] Readme                Document       9 KB      21 Oct     |
|                 scrollable rows with shared selection           |
```

Headers expose real typed sort, direction and keyboard activation. Dates/sizes sort
by metadata, not formatted text. Missing values are explicitly unknown/not applicable.
Grid and list refer to the same object identities, selection and command targets.
Virtualization/recycling never lets a label refer to a different click/drag target.
All 2,048 stress-fixture items are reachable; there is no silent 32/48-item cap.

## Inline creation and rename

```text
| [folder] Drafts     [folder] [New folder____________]            |
|                             Enter to create · Escape to cancel |
|                             [error here, if any]                |
```

Creation from toolbar, context menu and palette dispatches the same command.
Reserve a unique provisional ID, reveal it, then focus/select its draft name.
A list row mounts the same editor controller in its Name column.
Filtering must reveal the provisional item with an explicit temporary “Editing new
item outside this filter” marker, or offer Clear search before creation; never hide it.
The controller validates conflict/invalid names and preserves a recoverable draft.

## Selection and activation

Click selects; double-click/Enter activates. A folder enters its child directory;
a document opens a read-only sample preview or Workbench sample using a shared
Open intent. Info is not the default double-click behavior.
Ctrl selection and Shift range require the complete selection controller; until
implemented, advertise single selection rather than pretending multiselect works.
Right-click preserves/establishes an explicit target according to documented policy.
Keyboard focus is separate from selected items and is always visibly discoverable.

## Context menu O03

Leading semantic icons: Open, Rename, Duplicate, Move, Move to Trash; Restore only
inside Trash. Availability follows fixture capabilities and selection count.
Commands use the target captured at invocation, not later arbitrary selection.
Outside dismissal does not select a different item, scroll or start a drag.

## Direct manipulation

```text
| [source outline]           [lifted image + Aurora label]         |
|                                  pointer grab offset retained   |
| [folder] Ready  [valid-drop outline + "Move into Ready"]         |
```

Preview uses the real source component’s icon/thumbnail and label at a sensible
size, never a generic rectangular filename pill. Source remains identifiable.
Invalid target explains why; release outside cancels. Self/descendant moves are
rejected. Destination is parent ID, not merely a place label.
Moved entries disappear/appear once, counts update together and Undo is offered
only when the inverse operation can actually be performed.

## Empty / filtered / loading / error

```text
| Empty folder: [folder] Nothing here yet. [New folder]            |
| No matches:   [search] No assets match “coast”. [Clear search]    |
| Loading:      Loading samples… [Cancel]                         |
| Error:        [warning] Could not load this sample set. [Retry] |
```

These are mutually meaningful state views, not one blank grid.
Fixture loading/failure is labelled as a scenario when injected; default local
collection must not simulate a slow server simply to display a skeleton.

## Compact / inspector

Column count derives from measured pane width, including an open inspector.
Inspector is a drawer at narrow widths; selection summary remains accessible.
No partial final row creates interactive sentinel cells. Scrollbar has a usable
hit envelope, min thumb and shared paint/input geometry.

## Acceptance

- Every object has one unique ID and one parent; Trash preserves origin.
- All rendered labels, icons, menus and actions agree after sort/filter/rename.
- Grid/list/context/keyboard activation share one command implementation.
- Final stress-fixture item is reachable and correctly actionable.
- Drag, inline editor, scrollbar and overlays never compete for pointer ownership.
- No action claims to read/write the actual host filesystem.
