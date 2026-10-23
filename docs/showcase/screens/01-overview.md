# S01 — Overview: useful work before component browsing

**Route:** `/overview` · **Primary task:** resume or start a safe recipe.  
**Components:** AppShell, Navigation, SearchField, Button, Progress, Badge,
Section, ScrollArea, Inspector and shared command surfaces.

## Default wireframe

```text
+------------------------------------------------------------------------+
| Studio      [Find components…                     ] [Commands] [Help]   |
+---------------+--------------------------------------------------------+
| > Overview    | Overview                                     [Inspect]  |
|   Controls    | Try a real task. Inspect the components as you go.       |
|   Collections |                                                        |
|   Workbench   | Continue your note                                     |
|   Surfaces    | [doc] Interface notes                 Edited locally    |
|   Foundations | Last saved in this session.                  [Continue] |
|               |                                                        |
|               | Start a recipe                                         |
|               | [settings] Export preset    [folder] Sample workspace   |
|               | Fields + validation         Search + select + organize  |
|               | [Configure preset]          [Open samples]              |
|               |                                                        |
|               | Explore by need                                        |
|               | [Input] [Navigation] [Collections] [Feedback] [Layout]   |
|               |                                                        |
|               | Recent: Field · Menu · Dialog              [View all]  |
+---------------+--------------------------------------------------------+
| Local examples · Session state only                         [Status]   |
+------------------------------------------------------------------------+
```

No decorative dashboard counts or huge welcome hero. A progress element appears
only when a real recipe task is pending, not as an arbitrary “kit completeness” bar.
Overview itself demonstrates clear sections, navigation and stateful content.

## Content and actions

| Element | Real source | Action/result |
|---|---|---|
| Continue note | Current Workbench draft | Navigate to the same draft and caret/reveal policy |
| Configure preset | Controls recipe session | Navigate without resetting values |
| Open samples | Collection fixture session | Navigate to last location/filter |
| Need chips | Catalog category descriptors | Open filtered component discovery |
| Recent component | Inspector history by stable component ID | Open O02 on that component |
| Inspect | Page descriptor | Explain AppShell/sections and their shared sources |

Recent is a convenience, not hidden required navigation. The full catalog remains
available through discovery and the component coverage drawer.
Labels announce destination and do not say vague “Learn more” on every card.

## First launch / no history

```text
| Overview                                                        |
| Start with a two-minute recipe. All examples are local.          |
| [Configure an export preset]  [Organize sample assets]           |
| No recent components yet. Inspect any component to add one here.|
```

No empty Continue card; no fake “last opened” timestamp.
Do not open a tutorial modal automatically or force a tour before using the app.
Help offers a short optional walkthrough and keyboard guide.

## Discovery results state

```text
| Find components: [menu____________________________]              |
| +------------------------------------------------------------+ |
| | [layers] Menu              Contextual actions       Ready    | |
| | [panel]  Popover           Anchored options          Pending | |
| | No task commands here. Use Commands for actions.             | |
| +------------------------------------------------------------+ |
```

Ready/Pending reflect actual implementation status. Selecting a component routes
to its specimen and reveals it before opening the inspector. It does not claim a
planned control can run. Up/Down/Enter/Escape and focus restoration are specified.
No results offers category browse; it does not change the active route silently.

## Loading and failure

Overview fixture state is normally immediate. If session restoration is provided,
show non-blocking “Restoring local session…” while navigation remains usable.
Failure shows “Could not restore this session” with Start fresh and Details;
existing in-memory edits are never overwritten by a late restore result.
No animated skeleton is needed for a synchronous local object.

## Compact layout

At 800–959 width, persistent nav becomes O11 Menu. Recipe cards stack vertically;
Continue remains first. Discovery occupies available header width; Help moves
into app overflow with a visible labelled Menu/Commands path.
Cards and page content scroll inside the main pane, not under host chrome.
At larger text scale category chips wrap and cards grow; no fixed one-line heights.

## Keyboard and assistive flow

Header discovery, Commands, Help; navigation group; page task actions; recent list.
Within navigation, arrows choose route; Enter activates according to shared policy.
After explicit navigation, announce “Overview” heading once.
A card has one primary action; avoid a whole-card action plus nested button that
activates twice. Passive status uses text semantics, not a badge focus target.

## Exit and state

Navigation preserves drafts/fixture values. Overview Reset is not a permanent
prominent primary action; “Reset examples” lives in app menu and invokes O12.
If resetting would discard a draft, its summary lists affected work explicitly.

## Acceptance

- First launch has no fictional history or operation progress.
- Continue restores the correct draft, not a new blank document.
- Catalog filtering never executes a command.
- Navigating to an unavailable component explains the gap without dead controls.
- All cards use the shared Button/Section contracts rather than custom HTML-like skins.
- Tab/focus order matches visual grouping in default and compact layouts.
