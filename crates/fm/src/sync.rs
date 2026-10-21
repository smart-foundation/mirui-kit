//! Cross-widget synchronisation.
//!
//! The theme lives in the state model, but the palette itself is an
//! application resource. A small system bridges the two: whenever the model
//! says the app should be light or dark, the matching theme is installed.
//!
//! A second system watches the pointer during a drag and works out which
//! folder is underneath it, so a file can be dropped into it.

use mirui::prelude::*;

use crate::ui::content::DropZone;

/// Applies the palette the state model asks for.
#[system]
pub fn theme_sync(state: model!(crate::model::FmState), theme: res!(Theme), commands: commands!()) {
    let wanted = if state.dark() { "dark" } else { "light" };
    if theme.id().as_str() != wanted {
        commands.set_theme(wanted);
    }
}

/// Finds the folder under the pointer while a file is being dragged.
#[system]
pub fn drop_target(
    zones: query!(Entity, DropZone),
    rects: query!(Entity, mirui::ui::ComputedRect),
    state: model!(crate::model::FmState),
) {
    if state.dragging().is_none() {
        state.set_drop_target(None);
        return;
    }
    let (x, y) = state.drag_position();
    let pointer_x = Fixed::from_int(x);
    let pointer_y = Fixed::from_int(y);
    let mut found = None;
    for (entity, rect) in rects {
        let Some((_, zone)) = zones.get(entity) else {
            continue;
        };
        let rect = rect.0;
        if pointer_x >= rect.x
            && pointer_x < rect.x + rect.w
            && pointer_y >= rect.y
            && pointer_y < rect.y + rect.h
        {
            found = Some(match zone.place {
                Some(place) => crate::model::DropTarget::Place(place),
                None => crate::model::DropTarget::Tile(zone.position),
            });
            break;
        }
    }
    state.set_drop_target(found);
}
