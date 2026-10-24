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
