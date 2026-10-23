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
