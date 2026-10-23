# S02 — Controls: configure a real export recipe

**Route:** `/controls` · **Primary task:** validate and preview an export preset.  
**Fixture:** a local export configuration; no upload, disk write or remote export.

## Default wireframe

```text
+------------------------------------------------------------------------+
| Studio      [Find components…                     ] [Commands] [Help]   |
+---------------+--------------------------------------------------------+
|   Overview    | Export preset                               [Inspect]  |
| > Controls    | Configure a local preview; no files will be written.   |
|   Collections |                                                        |
|   Workbench   | Preset                                  Preview         |
|   Surfaces    | Name                                    +-------------+ |
|   Foundations | [Review images________________]         | sample art  | |
|               | A descriptive name for this preset.     | real image  | |
|               |                                         +-------------+ |
|               | Format  (PNG) (JPEG) (WebP*)             3 sample items  |
|               | Quality  [------o------------] 80        Estimated only  |
|               | [x] Include metadata                    No real export  |
|               | [ ] Resize images                                       |
|               | Width [1280____] px  [disabled until Resize]            |
|               |                                                        |
|               | [Reset preset]                    [Preview export]     |
+---------------+--------------------------------------------------------+
| Local recipe · Unsaved preset changes                       [Status]   |
+------------------------------------------------------------------------+
```

`WebP*` is an example of capability-driven availability, not a commitment that the
current kit ships an encoder. Unsupported format is disabled with a readable
reason or omitted. All estimates are clearly labelled deterministic fixture estimates.
Sample art is a static asset loaded through the host/asset contract, not AI-generated
control imagery pretending to be a rendered result.

## Real components and controlled values

| Control | Value/intent | Contract |
|---|---|---|
| Name Field | `preset.name` | Label, helper/error, maximum policy, submit validation |
| Format SegmentedGroup | Format enum | Single selection, arrow keys, available options |
| Quality Slider | Integer 1..100 | Step 1, keyboard range, numeric value description |
| Metadata Checkbox | Boolean | Whole label toggles exactly once |
| Resize Checkbox | Boolean | Enables width; preserves draft when unchecked |
| Width NumberField | Optional positive integer | Invalid intermediate text allowed until validation |
| Preview Button | Submit preset | Disabled only when pending/unavailable; validates visibly |
| Reset Button | Restore recipe defaults | Confirm only if meaningful changes would be lost |

PNG does not expose a lossy quality slider. When PNG is selected, replace quality
with a short “PNG is lossless” explanation; do not leave a meaningless knob active.
Do not replace a proper boolean with a fake decorative switch.
NumberField is a planned adapter until implemented; no Live label before it works.

## Validation state

```text
| Name                                                           |
| [____________________________________________________________] |
| [error] Enter a preset name.                                    |
| Resize: [x]   Width [0_______] px                               |
| [error] Width must be greater than zero.                        |
|                                    [Preview export]            |
```

Submit focuses the first invalid control, preserves all input and shows local
errors. Error does not rely on red alone. Live validation begins after a field has
been visited or a submit attempt; no aggressive error on untouched first launch.
Enter submits only when the current editor/control contract permits it.

## Pending / cancelled / failed preview

```text
| Previewing 3 local samples…       [====      ]       [Cancel]     |
| Changing settings cancels this preview. Preview again to update. |
```

Preview executes a deterministic fixture job with operation ID and cancellation.
Value changes cancel the old job and require a fresh Preview; do not auto-restart
indefinitely while dragging the quality slider.
Cancel restores the last successful preview and announces cancellation, not success.
Injected failure offers Retry and Details while retaining the preset.
Late results for old presets cannot overwrite a newer preview.

## Empty/result presentation

No preview initially: sample illustration + “Preview this preset to see the result”.
Success: show fixture result summary with exact preset/version and “Local simulation”.
Do not display a fabricated byte size as measured encoder output.
If codec execution is later real, surface its capability and measured result explicitly.

## Inspector

Inspect any field through a dedicated “Inspect component” mode or context action,
not by hijacking ordinary clicks. The app-level Inspect action opens the form recipe;
component-specific selection highlights a specimen without consuming its control input.
O02 explains controlled values, validation, keyboard behavior and the actual public API.

## Compact and text-scale layout

Preview stacks below the form under 1100px. Label, control and error remain together.
Footer actions wrap or stack; they never overlap scroll content.
Do not fix form height to the number of controls at default text size.

## Acceptance

- The same shared Field/Checkbox/Slider styles are used in Studio’s own preferences.
- Slider updates thumb, model, value label and preview version consistently.
- Tapping checkbox label toggles once; disabled width has an accessible reason.
- Format-dependent controls are meaningful, not inert demonstration props.
- Reset, cancellation and failed preview preserve a clear recovery path.
- No spinner/toast claims a filesystem operation occurred.
