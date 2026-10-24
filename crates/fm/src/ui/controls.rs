//! The controls the application draws itself.
//!
//! Two of mirui's widgets are not what this application wants. The text
//! input draws a focus ring of its own, and the ring is always square; the
//! check box is a filled square without a tick. Both are chrome, so the
//! application draws them: a rounded ring around the field that has the
//! focus, and a box with a tick in it for the view options.

use mirui::prelude::*;
use mirui::render::FillRule;
use mirui::render::command::DrawCommand;
use mirui::render::font::FontManager;
use mirui::render::scene::Paint;
use mirui::types::Transform;
use mirui::ui::theme::WidgetState;
use mirui::ui::widgets::TextEditing;

use crate::icons;
use crate::model::FmState;
use crate::text;
use crate::theme::{HOVER, PRESSED};

/// The view box the icons are drawn on.
const ICON_VIEWBOX: Fixed = Fixed::from_ratio(24 * 256, 256);

/// The inset mirui lays the text of a field out from.
///
/// The platform hardcodes it: `mirui::ui::widgets::text_editing::geometry`
/// insets the content of a field by two pixels on every side and never
/// centres the paragraph, so the field has to be sized to the text.
const FIELD_INSET: i32 = 2;

/// The ring around the text field that has the focus.
#[component]
#[derive(Clone, Copy)]
pub struct FieldFocus {
    /// The colour of the ring.
    pub color: ThemedColor,
    /// The corner radius of the ring.
    pub radius: Fixed,
    /// The width of the ring.
    pub width: Fixed,
}

/// A check box with a tick in it.
#[component]
#[derive(Clone)]
pub struct CheckBox {
    /// The state the box is read from.
    pub model: model!(FmState),
    /// The size of the box, in logical pixels.
    pub size: i32,
}

/// The native editor's line box, NOT its outer field chrome. Shared FieldContent
/// remeasures the actual font; FieldFrame provides a 40px padded hit target.
pub fn field_height(fonts: &FontManager) -> Fixed {
    let line = fonts
        .resolve(FontToken::Default.cache_key())
        .metrics(text::BODY)
        .line_height;
    line + Fixed::from_int(FIELD_INSET * 2)
}

/// Paints the focus ring of a text field.
///
/// mirui draws a square ring of its own, in the colour of the input. The
/// field hides that ring by giving the input its own background as the ring
/// colour, and this view draws the ring the application wants: rounded, and
/// in the colour of the theme.
#[view(component = FieldFocus, name = "FieldFocus", priority = 80)]
pub fn field_focus(
    focus: component!(),
    entity: entity!(),
    editors: query!(TextEditing),
    theme: res!(Theme),
    paint: paint!(),
) {
    let Some(editing) = editors.get(entity) else {
        return;
    };
    if !editing.focused() {
        return;
    }
    paint.draw(&DrawCommand::Border {
        area: paint.rect(),
        transform: paint.transform(),
        quad: paint.quad(),
        color: focus.color.resolve(&theme),
        width: focus.width,
        radius: focus.radius,
        opa: 255,
    });
}

/// Paints the check box of the view options.
///
/// An empty box is an outline; a ticked one is filled with the accent and
/// carries the tick. The hover and the press are the translucent overlays
/// of the hover surface: they strengthen the outline of an empty box and
/// deepen the fill of a ticked one.
#[view(component = CheckBox, name = "CheckBox", priority = 70)]
pub fn check_box(
    check: component!(),
    state: model!(component.model, watch(show_hidden)),
    theme: res!(Theme),
    paint: paint!(),
) {
    let rect = paint.rect();
    let radius = Fixed::from_int(4);
    let checked = state.show_hidden;

    if checked {
        paint.draw(&DrawCommand::Fill {
            area: rect,
            transform: paint.transform(),
            quad: paint.quad(),
            color: theme.resolve(ColorToken::Primary),
            radius,
            opa: 255,
        });
    } else {
        paint.draw(&DrawCommand::Border {
            area: rect,
            transform: paint.transform(),
            quad: paint.quad(),
            color: theme.resolve(ColorToken::Outline),
            width: Fixed::ONE,
            radius,
            opa: 255,
        });
    }

    let overlay = match paint.state() {
        WidgetState::Pressed => Some(theme.resolve(PRESSED)),
        WidgetState::Hovered => Some(theme.resolve(HOVER)),
        _ => None,
    };
    if let Some(overlay) = overlay {
        paint.draw(&DrawCommand::Fill {
            area: rect,
            transform: paint.transform(),
            quad: paint.quad(),
            color: overlay,
            radius,
            opa: 255,
        });
    }

    if !checked {
        return;
    }

    // The tick, scaled down from Solar's 24x24 grid into the box. The glyph
    // sits in the middle of the grid, so the box only has to be scaled.
    let size = Fixed::from_int(check.size);
    if size <= Fixed::ZERO {
        return;
    }
    let scale = size / ICON_VIEWBOX;
    let transform = paint
        .transform()
        .compose(&Transform::translate(rect.x, rect.y))
        .compose(&Transform::scale(scale, scale));
    let brush = Paint::Color(theme.resolve(ColorToken::OnPrimary).into());
    paint.draw(&DrawCommand::FillPath {
        path: &icons::CHECK,
        transform,
        paint: &brush,
        opa: 255,
        fill_rule: FillRule::EvenOdd,
    });
}
