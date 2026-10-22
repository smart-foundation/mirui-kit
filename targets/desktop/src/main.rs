//! The desktop shell: an SDL2 window that also understands the right button.
//!
//! mirui's SDL surface maps every mouse button to the same pointer, so a right
//! click cannot reach the application as a gesture. SDL's global event filter
//! sees every event before the queue does, so the right button is recorded
//! here and dropped before mirui ever sees it; a small plugin then turns that
//! record into a context menu for the selected file.

use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

use mirui::app::App;
use mirui::app::plugin::Plugin;
use mirui::ecs::World;
use mirui::surface::sdl::SdlSurface;

use fm::model::FmStateHandle;

const WIDTH: u16 = 1180;
const HEIGHT: u16 = 720;

/// The position of a pending right click, in logical pixels.
static RIGHT_CLICK_X: AtomicI32 = AtomicI32::new(-1);
static RIGHT_CLICK_Y: AtomicI32 = AtomicI32::new(-1);
static RIGHT_CLICK_PENDING: AtomicBool = AtomicBool::new(false);

/// SDL's event filter: records the right button and hides it from mirui.
unsafe extern "C" fn capture_right_click(
    _userdata: *mut std::ffi::c_void,
    event: *mut sdl2::sys::SDL_Event,
) -> i32 {
    let Some(event) = (unsafe { event.as_ref() }) else {
        return 1;
    };
    let event_type = sdl2::event::EventType::MouseButtonDown as u32;
    let right = sdl2::mouse::MouseButton::Right as u8;
    let (kind, button, x, y) = unsafe {
        (
            event.type_,
            event.button.button,
            event.button.x,
            event.button.y,
        )
    };
    if kind == event_type && button == right {
        RIGHT_CLICK_X.store(x, Ordering::Relaxed);
        RIGHT_CLICK_Y.store(y, Ordering::Relaxed);
        RIGHT_CLICK_PENDING.store(true, Ordering::Release);
        // Drop the event: mirui would otherwise treat it as a left click.
        return 0;
    }
    1
}

/// Turns a recorded right click into a context menu.
struct RightClickPlugin {
    state: FmStateHandle,
}

impl<B, F> Plugin<B, F> for RightClickPlugin
where
    B: mirui::surface::Surface,
    F: mirui::app::RendererFactory<B>,
{
    fn build(&mut self, _app: &mut mirui::app::App<B, F>) {}

    fn pre_render(&mut self, _world: &mut World) {
        if !RIGHT_CLICK_PENDING.swap(false, Ordering::AcqRel) {
            return;
        }
        let x = RIGHT_CLICK_X.load(Ordering::Relaxed);
        let y = RIGHT_CLICK_Y.load(Ordering::Relaxed);
        self.state.open_menu_at_pointer(x, y);
    }
}

fn main() {
    // The filter has to be in place before the first event is queued.
    unsafe {
        sdl2::sys::SDL_SetEventFilter(
            Some(
                capture_right_click
                    as unsafe extern "C" fn(
                        *mut std::ffi::c_void,
                        *mut sdl2::sys::SDL_Event,
                    ) -> i32,
            ),
            std::ptr::null_mut(),
        );
    }

    let backend = SdlSurface::new("Files", WIDTH, HEIGHT);
    let mut app = App::new(backend);
    app.with_default_widgets().with_default_systems();
    // The gesture recogniser needs a clock to recognise a long press.
    app.add_plugin(mirui::app::plugins::StdInstantClockPlugin);
    // Without a frame rate cap the run loop spins at 100 % of a core and the
    // window stops responding to the compositor.
    app.add_plugin(mirui::app::plugins::FrameRateCapPlugin::new(60));
    let state = fm::install(&mut app);
    app.add_plugin(RightClickPlugin { state });
    app.run();
}
