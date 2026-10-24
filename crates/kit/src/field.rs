//! Field chrome is separate from Mirui 0.51's two-pixel-inset native editor.
//! Never pad the editor itself: caret hit testing ignores Style padding upstream.
use mirui::input::event::focus::FocusState;
use mirui::prelude::*;
use mirui::render::{command::DrawCommand, font::FontManager};
use mirui::ui::{Children, Style, dirty::Dirty, widgets::TextEditing};

/// A visible editing hit surface, for consumers with their own drag recognizer.
#[component]
pub struct EditingSurface;
#[component]
pub struct FieldContent;
#[component]
#[derive(Default)]
pub struct FieldFrame {
    pub focused: bool,
}

/// Re-measure when typography changes, including the consumer's text-scale setting.
/// Only changed geometry dirties layout; idle frames do not enqueue work.
#[system]
pub fn measure(
    rows: query!(Entity, FieldContent, mut Style),
    fonts: res!(FontManager),
    commands: commands!(),
) {
    for (entity, _, style) in rows {
        let line = fonts
            .resolve(style.font_stack.primary().cache_key())
            .metrics(style.font_size.unwrap_or(14))
            .line_height;
        let height = Dimension::Px(line + Fixed::from_int(4));
        if style.layout.height != height {
            style.layout.height = height;
            commands.insert(entity, Dirty);
        }
    }
}

/// Padding is part of the input's hit target, without replacing native text hit testing.
#[event]
pub fn focus(
    node: entity!(),
    children: query!(Children),
    editors: query!(TextEditing),
    focus: res!(mut FocusState),
    commands: commands!(),
) -> bool {
    if let Some(children) = children.get(node) {
        for &child in &children.0 {
            if editors.get(child).is_some() {
                if let Some(previous) = focus.focused {
                    commands.insert(previous, Dirty);
                }
                focus.focused = Some(child);
                commands.insert(child, Dirty);
                return true;
            }
        }
    }
    false
}

#[view(component = FieldFrame, name = "FieldFrame", priority = 80)]
pub fn frame(frame: component!(), theme: res!(Theme), paint: paint!()) {
    if !frame.focused || paint.state() == mirui::ui::theme::WidgetState::Disabled {
        return;
    }
    paint.draw(&DrawCommand::Border {
        area: paint.rect(),
        transform: paint.transform(),
        quad: paint.quad(),
        color: theme.resolve(crate::theme::FOCUS),
        width: Fixed::from_int(3),
        radius: Fixed::from_int(8),
        opa: 255,
    });
}

/// Install the field services without replacing the consumer's theme or fonts.
pub fn install<B: Surface, F: RendererFactory<B>>(app: &mut App<B, F>) {
    app.with_widget(frame::view());
    app.add_system(measure::system());
    app.add_system(focus_sync::system());
}

/// A focus transition repaints both old/new outer rings even inside an offscreen cache.
#[system]
pub fn focus_sync(
    frames: query!(Entity, mut FieldFrame),
    children: query!(Children),
    focus: res!(FocusState),
    commands: commands!(),
) {
    for (entity, frame) in frames {
        let focused = children
            .get(entity)
            .is_some_and(|children| children.0.iter().any(|child| Some(*child) == focus.focused));
        if frame.focused != focused {
            frame.focused = focused;
            commands.insert(entity, Dirty);
        }
    }
}
