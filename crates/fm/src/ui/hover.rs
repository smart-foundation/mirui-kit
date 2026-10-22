//! The widget kit: surfaces that react to the pointer, and the file icon.
//!
//! mirui's own hover blend is eight percent toward the on-surface colour,
//! which is invisible on the dark palette, and a widget that binds its own
//! background colour reactively dirties every widget that binds it. So the
//! hover and the press are painted by a view that reads the widget state and
//! paints nothing at rest, and the selection stays a reactive background
//! colour, which mirui compares before it writes.

use mirui::prelude::*;
use mirui::render::command::DrawCommand;
use mirui::render::scene::Paint;
use mirui::render::FillRule;
use mirui::types::Transform;
use mirui::ui::theme::WidgetState;


/// The view box the icons are drawn on.
const ICON_VIEWBOX: Fixed = Fixed::from_ratio(24 * 256, 256);

/// A surface that reacts to the pointer.
#[component]
#[derive(Clone, Copy)]
pub struct HoverSurface {
    /// The colour under the pointer.
    pub hover: ThemedColor,
    /// The colour while the pointer is down.
    pub pressed: ThemedColor,
    /// The corner radius.
    pub radius: Fixed,
}

/// The file icon of the entry the model points at.
#[component]
#[derive(Clone)]
pub struct KindIcon {
    /// The state the icon is read from.
    pub model: model!(crate::model::FmState),
    /// The position in the current listing.
    pub index: u32,
    /// When set, this entry is drawn instead of the one at `index`.
    pub id: Option<u32>,
    /// The size of the icon in logical pixels.
    pub size: i32,
}

/// Paints the hover and the press of a surface.
///
/// Nothing is painted at rest, so a resting row costs no repaint and the
/// surface underneath - a selection, a panel - shows through.
#[view(component = HoverSurface, name = "HoverSurface", priority = 70)]
pub fn hover_surface(surface: component!(), theme: res!(Theme), paint: paint!()) {
    let state = paint.state();
    let color = match state {
        WidgetState::Pressed => surface.pressed,
        WidgetState::Hovered => surface.hover,
        _ => return,
    };
    let rect = paint.rect();
    paint.draw(&DrawCommand::Fill {
        area: rect,
        transform: paint.transform(),
        quad: paint.quad(),
        color: color.resolve_in(&theme, state),
        radius: surface.radius,
        opa: 255,
    });
}

/// The vector icon of the file the model points at.
#[view(component = KindIcon, name = "KindIcon", priority = 70)]
pub fn kind_icon(
    icon: component!(),
    model: model!(component.model, watch(listing_revision)),
    theme: res!(Theme),
    paint: paint!(),
) {
    let id = match icon.id {
        Some(id) => id,
        None => model.entry_id(icon.index),
    };
    let kind = model.entry_kind(id);

    let size = Fixed::from_int(icon.size);
    if size <= Fixed::ZERO {
        return;
    }
    let rect = paint.rect();
    let x = rect.x + (rect.w - size) / Fixed::from_int(2);
    let y = rect.y + (rect.h - size) / Fixed::from_int(2);
    let scale = size / ICON_VIEWBOX;
    let transform = paint
        .transform()
        .compose(&Transform::translate(x, y))
        .compose(&Transform::scale(scale, scale));

    let color = ThemedColor::Raw(kind.tint()).resolve_in(&theme, paint.state());
    let brush = Paint::Color(color.into());
    paint.draw(&DrawCommand::FillPath {
        path: kind.icon(),
        transform,
        paint: &brush,
        opa: 255,
        fill_rule: FillRule::EvenOdd,
    });
}
