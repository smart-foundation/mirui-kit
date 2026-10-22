//! The desktop shell: an SDL2 window that also understands the right button.
//!
//! mirui's SDL surface maps every mouse button to the same pointer, so a right
//! click cannot reach the application as a gesture. SDL's global event filter
//! sees every event before the queue does, so the right button is recorded
//! here and dropped before mirui ever sees it; a small plugin then turns that
//! record into a context menu for the file under the pointer.
//!
//! The filter understands two shortcuts as well: `Ctrl P` opens the command
//! palette and `Ctrl Shift S` asks the host for a screenshot. Both are
//! dropped, so mirui never sees the bare key press.
//!
//! The window can also be photographed: `--shot <path>` (or the `MIRUI_SHOT`
//! environment variable) writes the first frame to a PNG file and leaves the
//! application running.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

use mirui::app::App;
use mirui::app::plugin::Plugin;
use mirui::ecs::World;
use mirui::render::texture::{ColorFormat, TexturePixels};
use mirui::surface::sdl::SdlSurface;

use fm::model::{FmStateHandle, NoticeKind};

const WIDTH: u16 = 1180;
const HEIGHT: u16 = 720;

/// Where a screenshot is written when the shell is asked for one.
const SCREENSHOT_PATH: &str = "artifacts/screenshot.png";

/// The position of a pending right click, in logical pixels.
static RIGHT_CLICK_X: AtomicI32 = AtomicI32::new(-1);
static RIGHT_CLICK_Y: AtomicI32 = AtomicI32::new(-1);
static RIGHT_CLICK_PENDING: AtomicBool = AtomicBool::new(false);
/// The command palette was asked for.
static PALETTE_PENDING: AtomicBool = AtomicBool::new(false);
/// A screenshot was asked for.
static SCREENSHOT_PENDING: AtomicBool = AtomicBool::new(false);

/// SDL's event filter: records the right button and the shortcuts, and hides
/// them from mirui.
unsafe extern "C" fn capture_shortcuts(
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
    if capture_shortcut(event) {
        // Drop the event: mirui would otherwise see a bare key press.
        return 0;
    }
    1
}

/// Records the shortcuts the shell owns.
///
/// `Ctrl P` opens the command palette, `Ctrl Shift S` asks for a screenshot.
/// Both are dropped, so mirui never sees them.
fn capture_shortcut(event: &sdl2::sys::SDL_Event) -> bool {
    let key_down = sdl2::event::EventType::KeyDown as u32;
    // The type is read first: the event is a union, and only a key event
    // carries a key code.
    if unsafe { event.type_ } != key_down {
        return false;
    }
    let (sym, modifiers, state, repeat) = unsafe {
        (
            event.key.keysym.sym,
            event.key.keysym.mod_,
            event.key.state,
            event.key.repeat,
        )
    };
    if state != sdl2::sys::SDL_PRESSED as u8 || repeat != 0 {
        return false;
    }
    let modifiers = sdl2::keyboard::Mod::from_bits_truncate(modifiers);
    let control = modifiers
        .intersects(sdl2::keyboard::Mod::LCTRLMOD | sdl2::keyboard::Mod::RCTRLMOD);
    if !control {
        return false;
    }
    let shift = modifiers
        .intersects(sdl2::keyboard::Mod::LSHIFTMOD | sdl2::keyboard::Mod::RSHIFTMOD);
    if sym == sdl2::keyboard::Keycode::P.into_i32() && !shift {
        PALETTE_PENDING.store(true, Ordering::Release);
        return true;
    }
    if sym == sdl2::keyboard::Keycode::S.into_i32() && shift {
        SCREENSHOT_PENDING.store(true, Ordering::Release);
        return true;
    }
    false
}

/// Turns the recorded input into the commands of the application.
struct ShellPlugin {
    state: FmStateHandle,
}

impl<B, F> Plugin<B, F> for ShellPlugin
where
    B: mirui::surface::Surface,
    F: mirui::app::RendererFactory<B>,
{
    fn build(&mut self, _app: &mut mirui::app::App<B, F>) {}

    fn pre_render(&mut self, _world: &mut World) {
        if RIGHT_CLICK_PENDING.swap(false, Ordering::AcqRel) {
            let x = RIGHT_CLICK_X.load(Ordering::Relaxed);
            let y = RIGHT_CLICK_Y.load(Ordering::Relaxed);
            // The file under the pointer gets the file menu; the empty space
            // around the files gets the background menu.
            if self.state.hovered_item().is_some() {
                self.state.open_menu_at_pointer(x, y);
            } else {
                self.state.open_background_menu(x, y);
            }
        }
        if PALETTE_PENDING.swap(false, Ordering::AcqRel) {
            self.state.open_palette();
        }
        if SCREENSHOT_PENDING.swap(false, Ordering::AcqRel) {
            self.state.request_screenshot();
        }
    }
}

/// Where a screenshot was asked for, if one was.
fn shot_path() -> Option<PathBuf> {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--shot" {
            return args.next().map(PathBuf::from);
        }
        if let Some(path) = arg.strip_prefix("--shot=") {
            return Some(PathBuf::from(path));
        }
    }
    std::env::var_os("MIRUI_SHOT")
        .filter(|path| !path.is_empty())
        .map(PathBuf::from)
}

/// Write the current frame to a PNG file.
///
/// The pixels are read back from the window itself, in the format the
/// backend uses. A failure is reported, never fatal: the window keeps
/// running. Returns whether the file was written.
fn capture<B: mirui::surface::Surface, F: mirui::app::RendererFactory<B>>(
    app: &mut App<B, F>,
    path: &Path,
) -> bool {
    if let Err(error) = app.render() {
        eprintln!("desktop: the frame did not render: {error:?}");
        return false;
    }
    let texture = match app.snapshot() {
        Ok(Some(texture)) => texture,
        Ok(None) => {
            eprintln!("desktop: the window has no frame to capture");
            return false;
        }
        Err(error) => {
            eprintln!("desktop: the window cannot be read back: {error:?}");
            return false;
        }
    };
    let pixels = match texture.read_pixels() {
        Ok(pixels) => pixels,
        Err(error) => {
            eprintln!("desktop: the captured pixels cannot be read: {error:?}");
            return false;
        }
    };
    match write_png(path, &pixels) {
        Ok(()) => {
            println!("desktop: wrote {}", path.display());
            true
        }
        Err(error) => {
            eprintln!("desktop: {} was not written: {error}", path.display());
            false
        }
    }
}

/// Write captured pixels to a PNG file.
fn write_png(path: &Path, pixels: &TexturePixels<'_>) -> Result<(), Box<dyn std::error::Error>> {
    let (width, height) = (pixels.width(), pixels.height());
    let color = match pixels.format() {
        ColorFormat::RGBA8888 => png::ColorType::Rgba,
        ColorFormat::RGB888 => png::ColorType::Rgb,
        other => return Err(format!("unsupported pixel format {other:?}").into()),
    };
    let row_bytes = usize::from(width) * match color {
        png::ColorType::Rgba => 4,
        _ => 3,
    };
    let stride = pixels.stride();
    let bytes = pixels.bytes();
    if bytes.len() < stride * usize::from(height) {
        return Err("the captured pixels are shorter than the window".into());
    }
    // PNG rows are packed, the capture may be padded.
    let mut packed = Vec::with_capacity(row_bytes * usize::from(height));
    for row in 0..usize::from(height) {
        let start = row * stride;
        packed.extend_from_slice(&bytes[start..start + row_bytes]);
    }

    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent)?;
    }
    let file = std::fs::File::create(path)?;
    let mut encoder = png::Encoder::new(
        std::io::BufWriter::new(file),
        u32::from(width),
        u32::from(height),
    );
    encoder.set_color(color);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(&packed)?;
    Ok(())
}

fn main() {
    // The filter has to be in place before the first event is queued.
    unsafe {
        sdl2::sys::SDL_SetEventFilter(
            Some(
                capture_shortcuts
                    as unsafe extern "C" fn(
                        *mut std::ffi::c_void,
                        *mut sdl2::sys::SDL_Event,
                    ) -> i32,
            ),
            std::ptr::null_mut(),
        );
    }

    let shot = shot_path();

    let backend = SdlSurface::new("Files", WIDTH, HEIGHT);
    let mut app = App::new(backend);
    app.with_default_widgets().with_default_systems();
    // The gesture recogniser needs a clock to recognise a long press.
    app.add_plugin(mirui::app::plugins::StdInstantClockPlugin);
    // Without a frame rate cap the run loop spins at 100 % of a core and the
    // window stops responding to the compositor.
    app.add_plugin(mirui::app::plugins::FrameRateCapPlugin::new(60));
    let state = fm::install(&mut app);
    app.add_plugin(ShellPlugin {
        state: state.clone(),
    });
    // The notifications dismiss themselves; they must never sit on screen
    // forever.
    app.add_plugin(fm::sync::ToastDismissPlugin::new(state.clone()));

    if let Some(path) = shot {
        capture(&mut app, &path);
    }

    // The screenshot host: the application asks for a screenshot by bumping
    // its counter, and the host answers with the frame it sees.
    let mut served = state.screenshot_requested();
    if let Err(error) = app.render() {
        eprintln!("desktop: the first frame did not render: {error:?}");
    }
    loop {
        if app.tick() {
            break;
        }
        if state.screenshot_requested() == served {
            continue;
        }
        served = state.screenshot_requested();
        // The capture happens before anything else changes: the sheet that is
        // open stays open and the selection stays where it is, so the file
        // holds the frame the user is looking at.
        if capture(&mut app, Path::new(SCREENSHOT_PATH)) {
            state.notify(
                NoticeKind::Success,
                "Screenshot saved",
                SCREENSHOT_PATH,
            );
        }
    }
}
