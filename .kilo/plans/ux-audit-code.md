# UX Audit: Scrollbar (crates/fm)

File audited: `crates/fm/src/ui/scrollbar.rs` (full read, 105 lines)
Usage sites: `crates/fm/src/ui/content.rs` (icon grid + list views)
Mirui framework: `mirui-0.51.0` (read-only dependency)

## Summary

The scrollbar thumb is **drawn but not draggable**. It is a pure read-only overlay that paints the thumb position from the current offset, but it never writes a new offset back. Dragging the thumb does nothing to the content viewport. It respects content size vs visible size only for *painting geometry* (thumb length/travel), not for interaction.

---

## 1. Is the thumb draggable?

**No.** `scrollbar.rs` contains no pointer/press/drag handling at all. The `scroll_bar` view function (lines 31-105) reads `ScrollOffset`, `ScrollBounds`, and `ComputedRect` from the target scroller (line 33), computes thumb geometry (lines 84-89), and issues a single `DrawCommand::Fill` (lines 97-104). There is:

- no `on Press` / `on Drag` / `on Tap` handler on the thumb,
- no `HitTarget` marker inserted on the thumb entity,
- no `ScrollOffset` mutation anywhere in the file.

The thumb is therefore **not an input target**. Mirui's hit-test only considers entities carrying `HitTarget`, `ScrollConfig`, or `ScrollOffset` (mirui hit_test.rs lines 208-210, 344-351). Since the thumb view paints a `DrawCommand::Fill` but registers none of those marker components, a pointer down on the thumb falls through to whatever is underneath (the scroller viewport or background) rather than being captured as a thumb drag.

Citation: `crates/fm/src/ui/scrollbar.rs:97-104` — the only output is a paint command; no input handling.

---

## 2. Does dragging update content offset?

**No.** There is no code path that writes `ScrollOffset.x` or `ScrollOffset.y` in response to thumb movement. The view only *reads* `offset.y` / `offset.x` (lines 49, 56) to compute the thumb's painted shift (line 89: `let shift = position.clamp(...) * travel / max;`). Nothing in `scrollbar.rs` or its callers mutates the scroller's offset.

Mirui *does* support scroll dragging natively: `scroll_system` (mirui system.rs:56-77) hit-tests `PointerDown` events against `ScrollConfig`/`ScrollOffset` entities and drives a `ScrollDragState`. But that only applies to the scroller viewport itself (the `Scroll` widget in content.rs), not to the overlay thumb. The thumb has no `ScrollOffset` of its own and is not wired to the target's offset.

Citation: `crates/fm/src/ui/scrollbar.rs:84-90` — thumb position is derived from offset, never written back.

---

## 3. Does it respect content size vs visible size?

**Partially — for painting geometry only, and with a caveat.**

The thumb length and travel are computed from content vs viewport dimensions:

- Line 84: `let thumb = (viewport * track / content).min(track);` — thumb length scales with `viewport / content`.
- Line 88: `let travel = track - thumb;` — travel is the track minus the thumb.
- Line 89: `let shift = position.clamp(Fixed::ZERO, max) * travel / max;` — thumb position clamps to `[0, max]` where `max = bounds.max_y` / `bounds.max_x` (lines 48, 55).

`ScrollBounds` is published by mirui's `scroll_bounds` (mirui bounds.rs:41-79) with `max_y = (content_height - rect.h).max(Fixed::ZERO)` (line 77), so the clamp correctly reflects the scrollable range when content overflows the viewport.

The early-return guard at lines 60-62 hides the bar when nothing overflows (`content <= viewport || max <= Fixed::ZERO`), which is correct.

**However**, "respecting content vs visible size" for interaction means: drag the thumb by Δ pixels → move content by Δ * (content / track) pixels, clamped to `[0, max]`. That mapping is **absent**. Because there is no drag handling at all, the content-size/vs-visible-size math never influences scrolling.

Citation: `crates/fm/src/ui/scrollbar.rs:60-89`.

---

## 4. Root cause: why it feels non-draggable

The thumb feels non-draggable because it is **purely decorative**:

1. **No input capture.** The view never marks its entity as `HitTarget`, so mirui's hit test (`mirui hit_test.rs:208-210`) never selects the thumb as an event target. A press on the thumb passes through to the scroller viewport (which *is* scrollable via pointer drag on its body) or to content tiles behind it.

2. **No write-back.** Even if a drag were captured, `scrollbar.rs` has no handler to translate thumb delta into `ScrollOffset` mutation on `bar.target`. There is no `on Drag` closure, no system writing `offset.y`, and no clamping to `max_y`/`max_x` on interaction.

3. **Placement under an `IgnoreHitTest` parent (content.rs).** In `content.rs:73-82` and `content.rs:95-104`, the thumb's container `View` carries `IgnoreHitTest` (line 80 / line 102) and is absolutely positioned over the scroller's trailing edge. `IgnoreHitTest` excludes the entity *and effectively its subtree* from hit consideration in mirui's geometry collection (the writer checks `IgnoreHitTest` per entity at hit_test.rs:345). So even the `ScrollBar` view's own entity is excluded from hit-test by the ancestor marker, double-guaranteeing the thumb cannot receive pointer events.

Citation: `crates/fm/src/ui/content.rs:73-82`, `crates/fm/src/ui/content.rs:95-104`.

---

## 5. Proposed fix

Make the scrollbar a draggable thumb that mutates the target scroller's `ScrollOffset`, while keeping the existing paint geometry.

### Step A — Expose the thumb as a hit target

Remove `IgnoreHitTest` from the thumb's container `View` in `content.rs` (lines 80 and 102), OR better: keep `IgnoreHitTest` on the *track* (the thin strip) but add `HitTarget` + a drag handler on the thumb entity itself. Since `scroll_bar` is a single view function painting one thumb rect, the cleanest approach is to give the thumb its own hit target and gesture handling inside `scrollbar.rs`, and drop `IgnoreHitTest` from the container in `content.rs` so the thumb is reachable.

### Step B — Add drag handling to `scroll_bar`

Inside `scrollbar.rs`, add an `on Drag`-style handler (mirui gesture/press system) that:

1. On press, records the thumb's start screen position and the target's current `offset`.
2. On drag delta `Δ` (in thumb screen space), computes content delta `Δ_content = Δ * content / track` (inverse of the paint mapping at line 89).
3. Writes `offset.y = clamp(offset.y + Δ_content_y, 0, max_y)` (vertical) or `offset.x` (horizontal) on `bar.target`.
4. Mirui's `settle_offset` (mirui system.rs:95-107) already clamps and invalidates visuals, so reusing that clamp keeps behavior consistent.

The inverse mapping: paint does `shift = position * travel / max`, so a thumb movement of `Δ_shift` corresponds to content movement `Δ_position = Δ_shift * max / travel`, clamped to `[0, max]`.

### Step C — Respect content vs visible size during drag

- If `content <= viewport` or `max <= 0`, the bar already returns early (line 60-62) — no thumb, no drag. Good.
- During drag, clamp to `[0, max]` where `max = bounds.max_y` / `bounds.max_x` (already computed at line 89). This prevents dragging past the content extent.
- Thumb length `viewport * track / content` (line 84) already reflects the visible fraction; keep it.

### Minimal diff sketch (conceptual, not applied)

```rust
// In scrollbar.rs scroll_bar view:
let thumb_entity = ... // the painted thumb rect
thumb_entity [
    HitTarget
] on Drag { delta, .. } => {
    let (content, max, travel) = ... // from bounds/offset as above
    if travel <= Fixed::ZERO { return; }
    let delta_content = delta * content / travel; // inverse of paint
    let new_offset = (position + delta_content).clamp(Fixed::ZERO, max);
    // write new_offset to bar.target's ScrollOffset.y (or .x)
}
```

And in `content.rs`, remove `IgnoreHitTest` from the thumb container `View` (lines 80, 102) so the thumb can be hit-tested, while keeping the rest of the scroll container input-transparent as needed.

---

## Verification checklist

- [ ] Thumb entity carries `HitTarget` (or the container no longer carries `IgnoreHitTest`).
- [ ] `on Drag` / press handler writes `ScrollOffset` on `bar.target`.
- [ ] Offset clamped to `[0, max_y]` / `[0, max_x]` on every drag step.
- [ ] Bar hidden when `content <= viewport` (existing line 60-62 preserved).
- [ ] Dragging thumb by N pixels moves content by `N * content / track`, not `N` pixels 1:1.