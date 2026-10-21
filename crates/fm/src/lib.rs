//! A modern, Apple-inspired file manager built on `mirui` — as `no_std` + `alloc`.
//!
//! The crate contains no I/O, no clock and no platform code: the whole UI is a
//! pure function of the state in [`model::FmState`] and the static fake
//! filesystem in [`data`]. A target crate supplies a surface, a font and the
//! input events; everything else lives here.

#![no_std]

extern crate alloc;

pub mod data;
pub mod icons;
pub mod model;
pub mod sync;
pub mod theme;
pub mod ui;

use mirui::app::{App, RendererFactory};
use mirui::prelude::*;
use mirui::render::font::{Font, FontManager, FontToken};

/// The UI typeface, in mirui's MIRX asset container.
pub const FONT_BYTES: &[u8] = include_bytes!("../../../assets/misans_ui.mirx");

/// Install the theme, the typeface and the state model, then build the tree.
///
/// Returns the handle of the state model so the host can drive it.
pub fn install<B: Surface, F: RendererFactory<B>>(app: &mut App<B, F>) -> model::FmStateHandle {
    theme::install(app);
    install_font(app);
    let state = app.add_model(model::FmState::new()).finish();
    state.recompute();
    app.add_system(sync::theme_sync::system(state.clone()));
    app.add_system(sync::drop_target::system(state.clone()));
    let root = app.spawn_root().id();
    app.compose(root, ui::shell(state.clone()));
    state
}

/// Register the typeface for every built-in font role.
pub fn install_font<B: Surface, F: RendererFactory<B>>(app: &mut App<B, F>) {
    let font = Font::from_mirx(
        "MiSans UI",
        14,
        FONT_BYTES,
        &mirx::reader::PayloadLimits::HOST,
    )
    .expect("MiSans UI MIRX asset decodes");
    let Some(manager) = app.world.resource::<FontManager>() else {
        return;
    };
    manager.add_static(FontToken::Default.cache_key(), font.clone());
    manager.add_static(FontToken::Heading.cache_key(), font);
}
