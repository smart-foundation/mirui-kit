# Product, navigation and compact shell

## Product job

For an app developer: find a component, try its states, understand its contract,
then reuse a minimal real recipe without copying private rendering/input code.
For a design reviewer: complete representative tasks and examine consistency.
For Naledzi integration: exercise the same platform boundaries as a normal app.

Not a benchmark dashboard, icon wallpaper, marketing landing page or collection
of disconnected screenshots. It is an ordinary app whose useful work happens to
teach the kit. A version/support label is available in About, not a persistent hero.

## Information architecture

Six routes only; the labels remain visible in normal width.

| Route | User question | Primary task | Icon identity |
|---|---|---|---|
| Overview | Where do I start or resume? | Resume a recipe / find a component | Home |
| Controls | Do the controls work together? | Configure and validate an export | Sliders/settings |
| Collections | Can I organize many items safely? | Find, select, rename and move samples | Grid |
| Workbench | Can I build a real editing app? | Edit and save a note | Document |
| Surfaces | Do layers and feedback behave correctly? | Review and confirm a safe operation | Layers |
| Foundations | Does one policy adapt the whole app? | Preview and apply appearance | Appearance |

Documentation and diagnostics are secondary drawers/actions, not seven more tabs.
Preserve route-local fixture/filter/scroll state when switching sections.
Unsaved-work policy belongs to the owning task, not every tab switch.

## Default shell: 1180×720 logical app viewport

The desktop preview host is 1400×900, surrounding an unchanged 1180×720 Studio
surface with 110px horizontal and 90px vertical margins. These margins belong to
the preview stage, not the app’s content layout. No automatic UI zoom-to-fit.
Production mounting gives AppShell the actual host constraints and responds normally.

```text
1400×900 PreviewStage (host responsibility)
+--------------------------------------------------------------------+
|                  visible background / window shadow                |
|     +--------------------------------------------------------+     |
|     | Studio       [Find components…]       [Commands] [Help] |     | 48 header
|     +------------+-------------------------------------------+     |
|     | Home       | Current route title              [Inspect]|     |
|     | Controls   | Short purpose + one obvious task           |     |
|     | Collections|                                           |     |
|     | Workbench  |       real component workspace             |     | 648 body
|     | Surfaces   |                                           |     |
|     | Foundations|                                           |     |
|     +------------+-------------------------------------------+     |
|     | Local examples · Changes stay on this device   [Status] |     | 24 status
|     +--------------------------------------------------------+     |
+--------------------------------------------------------------------+
```

Header 48, status 24; body gets remaining height. Navigation width 184.
At default width the main pane is 996, with 24px side insets leaving 948.
A two-pane recipe may use 560 + 24 gap + 364; narrower widths collapse it.
No app route recomputes width using outer host size minus guessed panels.
Measure actual content constraints through the shared layout contract.

## Header and toolbar discipline

Global header contains component discovery, a labelled Commands trigger and Help.
It does not contain New Folder, export, sort or other route-specific operations.
A dedicated showcase is a legitimate place for a command discovery button;
this does not justify adding one beside Files content actions indiscriminately.
Use one command icon plus “Commands”; show Ctrl P in tooltip/help and palette,
not as an unexplained search-like duplicate text button.

The header discovery field searches the component catalog only, not sample files.
It is labelled “Find components”. Collection search says “Search sample assets”.
Do not put two indistinguishable Search fields in the same toolbar.
Discovery results use a compact anchored result list that navigates to a specimen;
commands perform actions. Both are keyboard-accessible, but their purpose differs.

Route toolbars contain only the task’s actions; low-frequency options use overflow.
Do not scatter search between unrelated icons or pad a sparse header with fake controls.
Use clear enabled/disabled state for unavailable actions and contextual reasons.
Host window actions are delegated; custom chrome uses the shared host contract.
No decorative traffic lights, duplicate close controls or unreliable drag regions.

## Responsive policy

| Width in logical pixels | Composition |
|---|---|
| >=1100 | Persistent labelled navigation; optional side-by-side recipe result |
| 960–1099 | Labelled navigation; recipe sections stack; inspector becomes overlay |
| 800–959 | Navigation becomes labelled Menu button/drawer; single-column tasks |
| <800 | Explicit unsupported-size message or host minimum; no silent clipping |

Height minimum is provisionally 480 for compact layouts; header/status stay small,
main content scrolls and dialogs use bounded internal scrolling.
The 1180×720 app preview remains fixed if host space permits; a smaller preview host
must request a supported responsive app size explicitly, never resize only chrome.
Larger text can trigger stacked layout earlier. Breakpoints are starting design
constraints, subject to actual text/host measurements rather than magic final values.

## Navigation and history

Route selection uses a shared navigation item, current-state cue and focus behavior.
Selecting a route moves focus to the route heading only for explicit navigation,
not every model update. Back returns through explicit navigation/deep links,
not every hover/slider change. Forward exists only when there is forward history.
Component links include route + specimen ID; app IDs are scoped by mount.
After inspection, closing a drawer restores focus to the invoked specimen/action.

## Progressive disclosure

The first view demonstrates one complete recipe, not thirty independent knobs.
“Inspect component” reveals public API, states, support and code recipe.
“Advanced states” reveals fault injection and unusual variations; normal controls
remain approachable. Every visual specimen is labelled Live, Fixture-backed or
Unavailable, with an explanation when it cannot run on the current host.
Unsupported is not the same as disabled due to a task precondition.

## Status and trust

Status reports local example mode, current operation and meaningful capability
status. Do not display static free-space percentages or “all tests passed”.
A Diagnostics link opens actual last-run evidence, labelled with timestamp/build.
A screen that has not run its tests says Not verified, even if its visuals look polished.
