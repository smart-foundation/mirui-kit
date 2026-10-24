//! A scroll bar for a scrolling container.
//!
//! mirui has no scroll bar widget at all: a `Scroll` or a `LazyList` keeps
//! its offset and its bounds, but nothing draws them. This view reads both
//! and paints a rounded thumb that follows the offset.
//!
//! The bar is an overlay: place it as a direct child of the scrolling
//! container, and the view draws the thumb at the container's trailing
//! edge — right for a vertical bar, bottom for a horizontal one.
//!
//! The thumb is a hit target: dragging it translates pointer movement into a
//! content offset through the inverse of the paint mapping, clamped to the
//! scrollable range.

use mirui::input::event::scroll::{ScrollBounds, ScrollOffset};
use mirui::prelude::*;
use mirui::render::DrawCommand;
use mirui::ui::{ComputedRect, HitTarget};

/// The gap between the thumb and the edge of the container.
/// The scroll container this bar reports on.
#[component]
#[derive(Clone, Copy)]
pub struct ScrollBar {
    /// The scrolling container the bar reports on.
    pub target: Entity,
    /// True for a vertical bar, false for a horizontal one.
    pub vertical: bool,
    /// The thickness of the thumb, in logical pixels.
    pub thickness: i32,
}

/// State of an in-progress thumb drag: the content offset and pointer
/// position where the drag began, used to translate thumb movement into a
/// content offset without drifting frame to frame.
#[component]
#[derive(Clone, Copy)]
pub struct ScrollBarThumbDrag {
    start_offset: Fixed,
    start_pointer: Fixed,
}

/// The thumb of a scroll bar.
#[view(
    component = ScrollBar,
    name = "ScrollBar",
    priority = 70,
    attach = scrollbar_attach::callback(),
    input = (
        scrollbar_drag_start::callback(),
        scrollbar_drag_move::callback(),
        scrollbar_drag_end::callback(),
        scrollbar_drag_cancel::callback(),
    ),
)]
pub fn scroll_bar(
    bar: component!(),
    rows: query!(ScrollOffset, optional ScrollBounds, optional ComputedRect),
    theme: res!(Theme),
    paint: paint!(),
) {
    let Some((offset, bounds, _container)) = rows.get(bar.target) else {
        return;
    };
    let Some(bounds) = bounds else {
        return;
    };

    let (viewport, content, max, position) = if bar.vertical {
        (
            bounds.viewport_height,
            bounds.content_height,
            bounds.max_y,
            offset.y,
        )
    } else {
        (
            bounds.viewport_width,
            bounds.content_width,
            bounds.max_x,
            offset.x,
        )
    };
    // Nothing overflows: the bar would be an empty rail.
    if content <= viewport || max <= Fixed::ZERO {
        return;
    }

    // The bar is the track: the thumb is drawn inside the bar's own box, so
    // the bar is a thin strip on the trailing edge of the container.
    // The bar is the track. `paint.rect()` is expressed in the space of the
    // bar's parent, so the thumb is placed at the bar's own origin.
    let widget = paint.rect();
    let (track, cross) = if bar.vertical {
        (widget.h, widget.w)
    } else {
        (widget.w, widget.h)
    };
    if track <= Fixed::ZERO {
        return;
    }
    let thickness = Fixed::from_int(bar.thickness).min(cross);
    if thickness <= Fixed::ZERO {
        return;
    }

    // The thumb is as long as the visible part of the content, and it
    // travels over the rest of the track.
    let thumb = (viewport * track / content).min(track);
    if thumb <= Fixed::ZERO {
        return;
    }
    let travel = track - thumb;
    let shift = position.clamp(Fixed::ZERO, max) * travel / max;

    let (offset_x, offset_y, w, h) = if bar.vertical {
        (Fixed::ZERO, shift, thickness, thumb)
    } else {
        (shift, Fixed::ZERO, thumb, thickness)
    };

    paint.draw(&DrawCommand::Fill {
        area: Rect::new(widget.x + offset_x, widget.y + offset_y, w, h),
        transform: paint.transform(),
        quad: paint.quad(),
        color: theme.resolve(crate::theme::SCROLLBAR),
        radius: thickness.min(thumb) / Fixed::from_int(2),
        opa: 255,
    });
}

/// Marks the scroll bar entity as a pointer hit target so the thumb can
/// receive drag input.
#[event]
fn scrollbar_attach(node: entity!(), commands: commands!()) {
    commands.insert(node, HitTarget);
}

/// Begins a thumb drag: remembers the content offset and pointer position at
/// the press point.
#[event]
fn scrollbar_drag_start(
    node: entity!(),
    input: event!(DragStart),
    bars: query!(ScrollBar),
    targets: query!(ScrollOffset, optional ScrollBounds),
    commands: commands!(),
) -> bool {
    let Some(bar) = bars.get(node) else {
        return false;
    };
    let Some((offset, bounds)) = targets.get(bar.target) else {
        return false;
    };
    let Some(bounds) = bounds else {
        return false;
    };

    let (position, max, viewport, content, pointer) = if bar.vertical {
        (
            offset.y,
            bounds.max_y,
            bounds.viewport_height,
            bounds.content_height,
            input.y,
        )
    } else {
        (
            offset.x,
            bounds.max_x,
            bounds.viewport_width,
            bounds.content_width,
            input.x,
        )
    };

    // Nothing overflows: there is nothing to drag.
    if content <= viewport || max <= Fixed::ZERO {
        return false;
    }

    commands.insert(
        node,
        ScrollBarThumbDrag {
            start_offset: position,
            start_pointer: pointer,
        },
    );
    true
}

/// Drags the thumb: translates the pointer delta into a content offset via
/// the inverse of the paint mapping (`shift = position * travel / max`) and
/// clamps to the scrollable range.
#[event]
fn scrollbar_drag_move(
    node: entity!(),
    input: event!(DragMove),
    bars: query!(ScrollBar, ComputedRect),
    targets: query!(ScrollOffset, optional ScrollBounds),
    drags: query!(ScrollBarThumbDrag),
    commands: commands!(),
) -> bool {
    let Some((bar, rect)) = bars.get(node) else {
        return false;
    };
    let Some((offset, bounds)) = targets.get(bar.target) else {
        return false;
    };
    let Some(bounds) = bounds else {
        return false;
    };
    let Some(drag) = drags.get(node) else {
        return false;
    };

    let (max, viewport, content, track, start_offset, start_pointer, pointer) = if bar.vertical {
        (
            bounds.max_y,
            bounds.viewport_height,
            bounds.content_height,
            rect.0.h,
            drag.start_offset,
            drag.start_pointer,
            input.y,
        )
    } else {
        (
            bounds.max_x,
            bounds.viewport_width,
            bounds.content_width,
            rect.0.w,
            drag.start_offset,
            drag.start_pointer,
            input.x,
        )
    };

    // Nothing overflows: there is nothing to drag.
    if content <= viewport || max <= Fixed::ZERO {
        return false;
    }

    let thumb = (viewport * track / content).min(track);
    if thumb <= Fixed::ZERO {
        return false;
    }
    let travel = track - thumb;
    if travel <= Fixed::ZERO {
        return false;
    }

    // Inverse of the paint mapping: `shift = position * travel / max`, so a
    // thumb delta of `delta` moves the content by `delta * max / travel`.
    let delta = pointer - start_pointer;
    let new_position = (start_offset + delta * max / travel).clamp(Fixed::ZERO, max);

    let new_offset = if bar.vertical {
        ScrollOffset {
            x: offset.x,
            y: new_position,
        }
    } else {
        ScrollOffset {
            x: new_position,
            y: offset.y,
        }
    };
    commands.insert(bar.target, new_offset);
    true
}

/// Ends a thumb drag: clears the drag state.
#[event]
fn scrollbar_drag_end(node: entity!(), _input: event!(DragEnd), commands: commands!()) {
    commands.remove::<ScrollBarThumbDrag>(node);
}

/// Cancels a thumb drag: clears the drag state without applying a final move.
#[event]
fn scrollbar_drag_cancel(node: entity!(), _input: event!(DragCancel), commands: commands!()) {
    commands.remove::<ScrollBarThumbDrag>(node);
}
