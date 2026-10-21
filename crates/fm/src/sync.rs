//! Cross-widget synchronisation.
//!
//! The theme lives in the state model, but the palette itself is an
//! application resource. A small system bridges the two: whenever the model
//! says the app should be light or dark, the matching theme is installed.

use mirui::prelude::*;

/// Applies the palette the state model asks for.
#[system]
pub fn theme_sync(state: model!(crate::model::FmState), theme: res!(Theme), commands: commands!()) {
    let wanted = if state.dark() { "dark" } else { "light" };
    if theme.id().as_str() != wanted {
        commands.set_theme(wanted);
    }
}
