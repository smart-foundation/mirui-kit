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

/// How far the pointer has to travel before a press becomes a drag.
const DRAG_THRESHOLD: i32 = 6;

/// Follows the pointer while a file is held down, and finds the folder under it.
///
/// The drag is tracked from the pointer cursor rather than from the drag
/// gesture: a mirui text widget consumes drag gestures for its own caret
/// selection, and a press event is delivered to the text rather than to the
/// tile around it, so neither route reaches a labelled tile.
#[system]
pub fn drag_tracking(
    cursor: res!(mirui::input::event::PointerCursor),
    zones: query!(Entity, DropZone),
    rects: query!(Entity, mirui::ui::ComputedRect),
    state: model!(crate::model::FmState),
) {
    let x = cursor.x.to_int();
    let y = cursor.y.to_int();
    let pointer_x = Fixed::from_int(x);
    let pointer_y = Fixed::from_int(y);

    // One pass over the zones: the file under the pointer and the folder that
    // would receive it.
    let mut source: Option<u32> = None;
    let mut target: Option<crate::model::DropTarget> = None;
    for (entity, rect) in rects {
        let Some((_, zone)) = zones.get(entity) else {
            continue;
        };
        let rect = rect.0;
        if pointer_x < rect.x
            || pointer_x >= rect.x + rect.w
            || pointer_y < rect.y
            || pointer_y >= rect.y + rect.h
        {
            continue;
        }
        match zone.place {
            Some(place) => {
                target = Some(crate::model::DropTarget::Place(place));
            }
            None => {
                source = Some(zone.id);
                if zone.folder {
                    target = Some(crate::model::DropTarget::Tile(zone.id));
                }
            }
        }
    }

    if !cursor.down {
        state.end_drag();
        state.clear_press();
        return;
    }

    if state.dragging().is_none() {
        match state.pressed_tile() {
            // The pointer is down on a file: it becomes a drag once it has
            // travelled far enough, even if it left the tile already.
            Some(pressed) => {
                let (press_x, press_y) = state.press_position();
                if (x - press_x).abs() + (y - press_y).abs() >= DRAG_THRESHOLD {
                    state.start_drag(pressed, x, y);
                }
            }
            None => {
                if let Some(id) = source {
                    state.press_tile(id, x, y);
                }
            }
        }
    } else {
        state.drag_to(x, y);
    }

    state.set_drop_target(target);
}
