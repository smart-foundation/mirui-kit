//! Cross-widget synchronisation.
//!
//! The theme lives in the state model, but the palette itself is an
//! application resource: a small system installs the theme the model asks
//! for, and only when the model actually changes its mind.
//!
//! A second system follows the pointer. While a file is held down it works
//! out which folder is underneath it, so the file can be dropped there. The
//! same system keeps the floating layers honest: it tells the model how big
//! the window is, so the context menu can stay on screen. Overlay backdrops
//! own dismissal; content tracking never bypasses them.
//!
//! The last system is a timer. It retires the notifications that have been
//! on screen long enough, so a message never lingers.

use mirui::app::RenderViewport;
use mirui::prelude::plugin::Plugin;
use mirui::prelude::*;

use crate::ui::content::DropZone;

/// How far the pointer travels before a press turns into a drag.
const DRAG_THRESHOLD: i32 = 6;

/// Applies the palette the state model asks for.
///
/// The applied palette is tracked in the model, so the theme is installed on
/// a real transition only: a palette that fails to install cannot turn into
/// a full re-layout of the tree on every frame.
#[system]
pub fn theme_sync(state: model!(crate::model::FmState), commands: commands!()) {
    let wanted = state.dark();
    if state.theme_applied() == Some(wanted) {
        return;
    }
    state.set_theme_applied(wanted);
    commands.set_theme(if wanted { "dark" } else { "light" });
}

/// Follows the pointer, and keeps the floating layers in step with it.
///
/// While the pointer is held down the system looks for the drop zone under
/// it: that is the file the drag started from and the folder the file would
/// land in. The scan only walks the zones themselves, not every widget in
/// the tree.
///
/// The system also reports the window size, so the model can keep the
/// context menu on screen. Overlay backdrops handle dismissal. Content
/// tracking respects overlay ownership for the full pointer press.
#[system]
pub fn drag_tracking(
    cursor: res!(mirui::input::event::PointerCursor),
    viewport: res!(RenderViewport),
    zones: query!(Entity, DropZone),
    fields: query!(Entity, mirui_kit::field::EditingSurface),
    hidden: query!(mirui::ui::Hidden),
    parents: query!(mirui::ui::Parent),
    rects: query!(Entity, mirui::ui::ComputedRect),
    state: model!(crate::model::FmState),
) {
    let x = cursor.x.to_int();
    let y = cursor.y.to_int();
    let pointer_x = Fixed::from_int(x);
    let pointer_y = Fixed::from_int(y);

    // The window size, for the context menu.
    let (width, height) = viewport.0.logical_size();
    let size = (i32::from(width), i32::from(height));
    if state.viewport() != size {
        state.set_viewport(size.0, size.1);
    }

    // Sample ownership before any dismissal can close the overlay. Ownership
    // survives dismissal for the entire press, not just this frame.
    let pointer_started = cursor.down && !state.content_pointer_down();
    if pointer_started
        && fields.into_iter().any(|(entity, _)| {
            let mut node = entity;
            for _ in 0..64 {
                if hidden.get(node).is_some() {
                    return false;
                }
                if let Some(parent) = parents.get(node) {
                    node = parent.0;
                } else {
                    break;
                }
            }
            rects.get(entity).is_some_and(|(_, rect)| {
                pointer_x >= rect.0.x
                    && pointer_x < rect.0.x + rect.0.w
                    && pointer_y >= rect.0.y
                    && pointer_y < rect.0.y + rect.0.h
            })
        })
    {
        state.block_content_pointer();
    }
    let content_allowed = state.update_content_pointer(cursor.down);
    if !content_allowed {
        state.set_hovered(None);
        state.cancel_drag();
        return;
    }

    // The zone under the pointer: the file under the pointer, and the folder
    // that would receive a drop. Only the zones are looked at, not every
    // widget in the tree.
    let mut source: Option<u32> = None;
    let mut target: Option<crate::model::DropTarget> = None;
    for (entity, zone) in zones {
        let Some((_, rect)) = rects.get(entity) else {
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
        break;
    }

    // The file under the pointer, whether the button is down or not: a right
    // click opens the menu of that file, not the menu of the selection. While
    // a menu is open the background stays unhovered.
    state.set_hovered(source);

    if !cursor.down {
        // The pointer is up: there is nothing to drag. Clean up, but only
        // when there is something to clean up, so an idle frame does not
        // write to the model at all.
        if state.dragging().is_some() || state.drop_target().is_some() {
            state.end_drag();
        } else if state.pressed_tile().is_some() {
            state.clear_press();
        }
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
                // Only a down transition may acquire a file. Moving a
                // held pointer from blank space must not start a drag.
                if pointer_started && let Some(id) = source {
                    state.press_tile(id, x, y);
                }
            }
        }
    } else {
        state.drag_to(x, y);
    }

    if state.dragging().is_some() {
        state.set_drop_target(target);
    }
}

/// Keeps the notifications from lingering.
///
/// The notifications are dismissed after a fixed number of ticks, so they can
/// never sit on screen forever, and a new notification restarts the
/// countdown.
///
/// The timer needs a clock: the desktop shell installs one, a target without
/// a clock simply never retires a notification.
///
/// **Inserts**
/// - entity:   a timer that ticks the countdown
pub struct ToastDismissPlugin {
    state: crate::model::FmStateHandle,
}

impl ToastDismissPlugin {
    /// Watch the state model of an installed application.
    pub fn new(state: crate::model::FmStateHandle) -> Self {
        Self { state }
    }
}

impl<B, F> Plugin<B, F> for ToastDismissPlugin
where
    B: Surface,
    F: RendererFactory<B>,
{
    fn build(&mut self, app: &mut App<B, F>) {
        install_notifications(app, &self.state);
    }
}

/// Marks a world that already runs the notification timer.
///
/// The timer is installed once: a second one would age the notifications
/// twice as fast.
struct NotificationTimerInstalled;

/// Install the timer that retires the notifications.
///
/// The timer ticks four times a second and retires the oldest notification
/// after ten ticks, so a message is on screen long enough to be read and
/// short enough to stay out of the way.
///
/// The timer needs a clock. The desktop shell installs one; a target without
/// a clock never retires a notification, which is the same as a message that
/// stays on screen.
pub fn install_notifications<B: Surface, F: RendererFactory<B>>(
    app: &mut App<B, F>,
    state: &crate::model::FmStateHandle,
) {
    if app.world.resource::<NotificationTimerInstalled>().is_some() {
        return;
    }
    app.world.insert_resource(NotificationTimerInstalled);
    app.add_timer(notification_timer::timer(state.clone()));
}

/// Retires the notifications that have been on screen long enough.
///
/// The tick period belongs to the timer: four ticks a second times ten ticks
/// is about two and a half seconds.
#[timer(every = 250)]
pub fn notification_timer(state: model!(crate::model::FmState)) {
    state.tick_notices();
}

/// Focus and native editing capability belong to the active Files layer.
#[component]
pub struct FieldScope(pub u8);
#[derive(Default)]
pub struct FieldFocusState {
    applied: Option<(u8, Option<crate::model::DialogKind>, Option<u32>)>,
    saved: Option<Entity>,
    pending: bool,
}
#[system]
pub fn field_focus(
    state: model!(crate::model::FmState),
    memory: res!(mut FieldFocusState),
    focus: res!(mut mirui::input::event::focus::FocusState),
    ids: res!(mirui::ui::IdMap),
    fields: query!(Entity, FieldScope),
    restore_fields: query!(FieldScope),
    editors: query!(mirui::ui::widgets::TextEditing),
    disabled: query!(mirui_kit::availability::InputBlocked),
    commands: commands!(),
) {
    use crate::model::DialogKind;
    let layer = if state.palette() {
        2
    } else if state.dialog().is_some() {
        1
    } else if state.has_input_overlay() {
        4
    } else if state.editing_folder_id().is_some() {
        3
    } else {
        0
    };
    for (entity, scope) in fields {
        // Inline naming is nonmodal: clicking search must still focus it and
        // let the native FocusChanged handler commit the name.
        let blocked = scope.0 != layer && !(layer == 3 && scope.0 == 0);
        if blocked && disabled.get(entity).is_none() {
            commands.set::<mirui_kit::availability::InputAvailability>(entity, false);
        } else if !blocked && disabled.get(entity).is_some() {
            commands.set::<mirui_kit::availability::InputAvailability>(entity, true);
        }
    }
    let key = (layer, state.dialog(), state.editing_folder_id());
    if memory.applied == Some(key) && !memory.pending {
        return;
    }
    if memory.applied.is_none_or(|old| old.0 == 0) && layer != 0 && !memory.pending {
        memory.saved = focus.focused;
    }
    let name = match layer {
        1 => match state.dialog() {
            Some(DialogKind::NewFolder) => Some("sheet_new_folder"),
            Some(DialogKind::Rename) => Some("sheet_rename"),
            _ => None,
        },
        2 => Some("palette_filter"),
        3 => Some("inline_rename"),
        _ => None,
    };
    let next = if let Some(name) = name {
        let target = ids
            .get(name)
            .filter(|&entity| editors.get(entity).is_some());
        // A freshly mounted inline editor may not exist until the next prepare.
        if target.is_none() {
            focus.focused = None;
            memory.applied = Some(key);
            memory.pending = true;
            return;
        }
        target
    } else if layer == 0 {
        if memory.applied.is_none() {
            focus.focused
        } else {
            let saved = memory.saved.take();
            focus
                .focused
                .filter(|&entity| restore_fields.get(entity).is_some_and(|scope| scope.0 == 0))
                .or_else(|| saved.filter(|&entity| editors.get(entity).is_some()))
        }
    } else {
        None
    };
    if let Some(previous) = focus.focused {
        commands.insert(previous, mirui::ui::dirty::Dirty);
    }
    focus.focused = next;
    if let Some(next) = next {
        commands.insert(next, mirui::ui::dirty::Dirty);
        if layer == 1 || layer == 3 {
            commands.selection().select_all(next);
        }
    }
    memory.applied = Some(key);
    memory.pending = false;
}

/// Escape dismisses the top editing surface before cancelling its owning task.
#[event]
pub fn escape(
    event: event!(Key),
    state: model!(crate::model::FmState),
    selection: res!(optional mirui_kit::selection::SelectionToolsResource),
) -> bool {
    if !event.pressed || event.repeat == Some(true) || event.code != 27 {
        return false;
    }
    if let Some(selection) = selection {
        if selection.0.visible() || !selection.0.feedback().is_empty() {
            selection.0.dismiss();
            return true;
        }
    }
    if state.palette() {
        state.close_palette();
    } else if state.dialog().is_some() {
        state.close_dialog();
    } else if state.options_open() {
        state.set_options_open(false);
    } else if state.menu_open() {
        state.close_menu();
    } else if state.editing_folder_id().is_some() {
        state.cancel_inline_rename();
    } else {
        return false;
    }
    true
}
