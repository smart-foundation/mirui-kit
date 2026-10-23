//! A modern, Apple-inspired file manager built on `mirui` — as `no_std` + `alloc`.
//!
//! The crate contains no I/O, no clock and no platform code: the whole UI is a
//! pure function of the state in [`model::FmState`] and the static fake
//! filesystem in [`data`]. A target crate supplies a surface, a font and the
//! input events; everything else lives here.

#![no_std]

extern crate alloc;

pub mod data;
pub mod font;
pub mod icons;
pub mod model;
pub mod sync;
pub mod text;
pub mod theme;
pub mod ui;

use mirui::app::{App, RendererFactory};
use mirui::prelude::*;

/// Install the theme, the typeface and the state model, then build the tree.
///
/// Returns the handle of the state model so the host can drive it.
pub fn install<B: Surface, F: RendererFactory<B>>(app: &mut App<B, F>) -> model::FmStateHandle {
    theme::install(app);
    font::install(app);
    install_views(app);
    let state = app.add_model(model::FmState::new()).finish();
    state.recompute();
    sync::install_notifications(app, &state);
    app.add_system(sync::theme_sync::system(state.clone()));
    app.add_system(sync::drag_tracking::system(state.clone()));
    let root = app.spawn_root().id();
    app.compose(root, ui::shell(state.clone()));
    state
}

/// Register the views of the widget kit.
///
/// The built-in widgets come with `App::with_default_widgets`; the views
/// that paint a hover, a selection and a file icon are registered here, so
/// every tile, row and menu entry can carry them.
fn install_views<B: Surface, F: RendererFactory<B>>(app: &mut App<B, F>) {
    app.with_widget(ui::hover::hover_surface::view());
    app.with_widget(ui::hover::kind_icon::view());
    app.with_widget(ui::controls::field_focus::view());
    app.with_widget(ui::controls::check_box::view());
    app.with_widget(ui::scrollbar::scroll_bar::view());
}
