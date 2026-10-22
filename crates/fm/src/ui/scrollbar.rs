//! A scroll bar for a scrolling container.
//!
//! mirui has no scroll bar widget at all: a `Scroll` or a `LazyList` keeps
//! its offset and its bounds, but nothing draws them. This view reads both
//! and paints a rounded thumb that follows the offset.
//!
//! The bar is an overlay: place it as a direct child of the scrolling
//! container, and the view draws the thumb at the container's trailing
//! edge — right for a vertical bar, bottom for a horizontal one.

use mirui::prelude::*;
use mirui::input::event::scroll::{ScrollBounds, ScrollOffset};
use mirui::render::DrawCommand;
use mirui::ui::ComputedRect;

/// The gap between the thumb and the edge of the container.
/// The scroll container this bar belongs to.
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

/// The thumb of a scroll bar.
#[view(component = ScrollBar, name = "ScrollBar", priority = 70)]
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
