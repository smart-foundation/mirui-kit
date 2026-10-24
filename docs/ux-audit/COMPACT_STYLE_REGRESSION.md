# Compact desktop style regression

The user rejected the post-parity visual direction: controls became taller,
rounding was insufficiently refined, persistent blue outlines added noise, and
Studio looked like a component inventory rather than a polished desktop app.
Those are design regressions, not failures that passing compiler/geometry tests
can dismiss. Files-shell work is paused while this shared styling is corrected.

## Concrete causes

- `ButtonSpec::labelled` explicitly chose Comfortable/40px despite exposing a
  Compact density. Inputs independently used 44px. The showcase's legacy action
  helper also used 40px, while text-scale multiplied those large bases.
- List, tree, table and grouped-option templates painted blue borders whenever
  their models remembered an active row. They did not require keyboard focus.
  Thus several unrelated collections could look focused at once.
- The focus layer assigned an initial focus target and the painter always drew
  a heavy 3px ring; pointer and keyboard presentation were not distinguished.
- Most control outlines were 2px at rest and the shared radius stayed at 6px.

## Focused correction

Desktop default buttons/collection rows use a shared 32px base; fields use 36px.
Body font size and weight are unchanged. Comfortable/40px and Touch/48px remain
explicit density choices; enlarged text continues to scale controls rather than
being clipped into a smaller box. Normal control outlines are 1px and shared
corners are 8px, not universal pills. Keyboard focus remains clearly visible
at 2px, but is not painted merely because a model remembers its active item.
Pointer press hides keyboard rings without destroying focus or selection.

This is a bounded correction, not a declaration that Studio has been redesigned.
A useful showcase still needs coherent task flows, proper composition hierarchy,
restrained elevation and selected/default/hover/pressed/keyboard states reviewed
together. The native slider's coupled paint/hit rectangle remains an upstream
constraint. Screenshots must be inspected before accepting new visual baselines;
none are automatically approved by this change.


## Native verification — 2026-10-24

Source revision `c30c56d`, Actions run
[40104490897](https://github.com/smart-foundation/mirui-kit/actions/runs/40104490897)
is successful: full workspace tests, native shared-kit contracts, Files pointer
and overlay tests, shell geometry, no_std RISC-V, SDL compilation, syntax checks,
six route captures and eight component capture shards. The final interaction
helper now identifies actual `DropZone` file targets rather than same-height
row containers; selection and overlay assertions were retained.

Exact PNGs were recovered to `artifacts/studio/compact-review/` (ignored,
regenerable), with `review.html` as an index. Reviewed all six Studio routes,
the 1400×900 Files capture, and dark 100% button, field and numeric-entry
resting specimens. Fifteen representative component PNGs were recovered in
all; the remaining specimens and complete state matrix are not claimed reviewed.
The 640/1180/1400 shell geometry is tested, but only the 1400 Files image was
visually inspected in this pass.

Observed: compact buttons and fields retain readable body type and restrained
resting outlines; Files grid fills the pane and counts are inset. Studio is
**not visually accepted**: the slider remains excessively thick, selected
navigation wraps a button in an oversized blue backing, full-width commands
flatten action hierarchy, preview panes waste space, and Collections/Surfaces
still prioritize inventories over composed tasks. No visual baseline is approved
and this run does not establish core framework parity or suite readiness.
