//! Naledzi Studio: real Mirui compositions over a no_std, session-only core.
#![no_std]
extern crate alloc;
pub mod catalog;
pub mod specimens;
pub mod commands;
pub mod model;
pub mod ui;
use crate::commands::Command;
use crate::model::{Studio, StudioHandle};
use mirui::prelude::*;

pub fn install<B: Surface, F: RendererFactory<B>>(app: &mut App<B, F>) -> StudioHandle {
    mirui_kit::install(app);
    let state = app.add_model(Studio::default()).finish();
    state.initialize();
    // Dispatch first, then close the native input gate in the same frame.
    app.add_system(synchronize::system(state.clone()));
    mirui_kit::bind_actions(app, state.actions());
    app.on_event(Key::EVENT, shortcuts::callback(state.clone()));
    app.add_timer(clock::timer(state.clone()));
    let root = app.spawn_root().id();
    app.compose(root, ui::shell(state.clone()));
    mirui_kit::selection::install(app, root);
    state
}
#[system]
fn synchronize(state: model!(Studio), commands: commands!()) {
    state.sync();
    let theme = state.wanted_theme();
    if state.theme_applied() != Some(theme) {
        state.set_theme_applied(theme);
        commands.set_theme(theme.name());
    }
}
#[timer(every = 250)]
fn clock(state: model!(Studio)) {
    state.tick();
}
#[event]
fn shortcuts(
    event: event!(Key),
    state: model!(Studio),
    selection: res!(optional mirui_kit::selection::SelectionToolsResource),
) -> bool {
    if !event.pressed || event.repeat == Some(true) {
        return false;
    }
    if state.surface() == Some(studio_core::Surface::Palette) {
        if event.physical == Some(PhysicalKey::ArrowDown) {
            state.move_palette(1);
            return true;
        }
        if event.physical == Some(PhysicalKey::ArrowUp) {
            state.move_palette(-1);
            return true;
        }
    }
    if event.code == 27 {
        if let Some(selection) = selection {
            if selection.0.visible() || !selection.0.feedback().is_empty() {
                selection.0.dismiss();
                return true;
            }
        }
        let key = if state.surface().is_none() && state.editing().is_some() {
            Command::CancelName
        } else {
            Command::Close
        };
        state
            .actions()
            .activate(key as u16, state.surface().is_some());
        return true;
    }
    if !event
        .modifiers
        .is_some_and(|m| m.control && !m.meta && !m.alt && !m.shift)
    {
        return false;
    }
    if event.physical == Some(PhysicalKey::KeyP) && state.surface().is_none() {
        state.actions().activate(Command::Palette as u16, false);
        return true;
    }
    if event.physical == Some(PhysicalKey::KeyS)
        && state.surface().is_none()
        && state.route() == studio_core::Route::Workbench
    {
        state.actions().activate(Command::SaveNote as u16, false);
        return true;
    }
    false
}
