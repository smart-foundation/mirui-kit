//! Shared no_std Mirui components. No file-manager or showcase state lives here.
#![no_std]
extern crate alloc;
pub mod availability;
pub mod behavior;
pub mod controls;
pub mod editing_policy;
pub mod feedback;
pub mod field;
pub mod font;
pub mod forms;
pub mod icons;
pub mod keyboard;
pub mod scrollbar;
pub mod selection;
pub mod style;
pub mod table;
pub mod theme;
pub mod widgets;
pub use widgets::{
    Actions, ActionsHandle, Editor, FocusItem, InputGate, Number, NumberHandle, Toggle,
    ToggleHandle,
};

use mirui::prelude::*;
/// Install assets and the shared component views once per App, after default widgets.
pub fn install<B: Surface, F: RendererFactory<B>>(app: &mut App<B, F>) {
    field::install(app);
    theme::install(app);
    font::install(app);
    app.with_widget(widgets::editor_view::view());
    app.with_widget(widgets::focus_ring::view());
    app.with_widget(scrollbar::scroll_bar::view());
    app.with_widget(widgets::gate_view::view());
}

/// Bind the input/focus services after the consumer's action-dispatch system.
pub fn bind_actions<B: Surface, F: RendererFactory<B>>(
    app: &mut App<B, F>,
    actions: ActionsHandle,
) {
    app.add_system(widgets::gate_sync::system());
    keyboard::install(app, actions);
}
