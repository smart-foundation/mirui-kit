//! Headless snapshots of the file manager.
//!
//! Renders a fixed set of states into `artifacts/*.png` so the UI can be
//! reviewed — and diffed — without a window server.

use std::cell::RefCell;

use mirui::app::App;
use mirui::prelude::*;
use mirui::render::texture::{ColorFormat, TexturePixels};
use mirui::surface::framebuf::FramebufSurface;
use mirui::types::PhysicalRect;

use fm::model::{FmStateHandle, NoticeKind, Place, ViewMode};

const WIDTH: u16 = 1180;
const HEIGHT: u16 = 720;

thread_local! {
    static CAPTURED: RefCell<Option<Vec<u8>>> = const { RefCell::new(None) };
}

fn capture(bytes: &[u8], _area: PhysicalRect) {
    CAPTURED.with(|slot| {
        *slot.borrow_mut() = Some(bytes.to_vec());
    });
}

fn main() {
    std::fs::create_dir_all("artifacts").expect("artifacts directory");

    let backend = FramebufSurface::with_format(WIDTH, HEIGHT, ColorFormat::RGBA8888, capture);
    let mut app = App::new(backend);
    app.with_default_widgets().with_default_systems();
    app.add_plugin(mirui::app::plugins::StdInstantClockPlugin);
    let state = fm::install(&mut app);
    // The notifications retire themselves: the timer needs the clock of the
    // shell.
    fm::sync::install_notifications(&mut app, &state);

    shot(&mut app, "01-dark-icons.png");

    app.set_theme("light").expect("light theme");
    shot(&mut app, "02-light-icons.png");
    app.set_theme("dark").expect("dark theme");

    state.set_place(Place::Downloads);
    shot(&mut app, "03-downloads.png");

    state.set_place(Place::Projects);
    shot(&mut app, "03b-projects.png");

    state.set_view(ViewMode::List);
    shot(&mut app, "04-list.png");

    state.set_query("report");
    state.document().set_text("report").expect("query text");
    shot(&mut app, "05-search.png");

    state.set_query("");
    state.document().set_text("").expect("query text");
    state.set_view(ViewMode::Icons);
    state.set_place(Place::Documents);
    state.select(state.entry_id(0));
    shot(&mut app, "06-selected.png");

    // The floating layers.
    state.set_view(ViewMode::Icons);
    state.set_place(Place::Documents);
    state.select(state.entry_id(3));
    state.open_menu(state.entry_id(3), 300, 180);
    shot(&mut app, "07-context-menu.png");
    state.close_menu();

    // The menu of the empty space around the files.
    state.open_background_menu(300, 180);
    shot(&mut app, "07b-background-menu.png");
    state.close_menu();

    state.open_dialog(fm::model::DialogKind::NewFolder);
    shot(&mut app, "08-dialog-new-folder.png");
    state.close_dialog();

    state.open_dialog(fm::model::DialogKind::GetInfo);
    shot(&mut app, "09-dialog-info.png");
    state.close_dialog();

    state.open_palette();
    state.set_palette_text("new");
    shot(&mut app, "10-command-palette.png");
    state.close_palette();

    state.toggle_inspector();
    shot(&mut app, "11-inspector.png");
    state.toggle_inspector();

    state.toggle_options();
    shot(&mut app, "12-view-options.png");
    state.toggle_options();

    // A file being dragged over a folder.
    let dragged = state.entry_id(60);
    state.start_drag(dragged, 420, 300);
    state.set_drop_target(Some(fm::model::DropTarget::Place(Place::Projects)));
    shot(&mut app, "13-drag-and-drop.png");
    state.end_drag();

    // The stack of notifications.
    state.notify(NoticeKind::Success, "New folder created", "Reports");
    state.notify(NoticeKind::Info, "Moved to Trash", "notes.txt");
    shot(&mut app, "14-notifications.png");
    state.clear_toast();

    // The screenshot the application asks for. The sheet stays open: the
    // capture happens before anything else changes, so the file holds the
    // frame the user is looking at.
    state.open_dialog(fm::model::DialogKind::GetInfo);
    state.request_screenshot();
    serve_screenshot(&mut app, &state);
    state.close_dialog();

    println!("wrote the snapshots to artifacts/");
}

/// Serve the screenshot the application asked for.
///
/// The counter of the model says how many screenshots were asked for; the
/// host compares it with the last one it served. The capture happens before
/// anything else changes, so a frame with a sheet open is captured exactly
/// as it is.
fn serve_screenshot<B: Surface, F: RendererFactory<B>>(
    app: &mut App<B, F>,
    state: &FmStateHandle,
) {
    if state.screenshot_requested() == 0 {
        return;
    }
    let texture = match app.snapshot() {
        Ok(Some(texture)) => texture,
        Ok(None) => {
            eprintln!("snapshot: there is no frame to capture");
            return;
        }
        Err(error) => {
            eprintln!("snapshot: the frame cannot be captured: {error:?}");
            return;
        }
    };
    let pixels = match texture.read_pixels() {
        Ok(pixels) => pixels,
        Err(error) => {
            eprintln!("snapshot: the captured pixels cannot be read: {error:?}");
            return;
        }
    };
    write_pixels("screenshot.png", &pixels);
    println!("  screenshot.png");
    state.notify(
        NoticeKind::Success,
        "Screenshot saved",
        "artifacts/screenshot.png",
    );
}

fn shot<B: Surface, F: RendererFactory<B>>(app: &mut App<B, F>, name: &str) {
    app.render().expect("frame renders");
    let raw = CAPTURED
        .with(|slot| slot.borrow().clone())
        .expect("frame captured");
    write_png(name, &raw);
    println!("  {name}");
}

fn write_png(name: &str, raw: &[u8]) {
    let mut buffer = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut buffer, WIDTH as u32, HEIGHT as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().expect("png header");
        writer.write_image_data(raw).expect("png data");
    }
    let path = std::path::Path::new("artifacts").join(name);
    std::fs::write(path, &buffer).expect("png written");
}

/// Write captured pixels to a PNG file.
///
/// The capture keeps the stride of the window, the PNG wants packed rows.
fn write_pixels(name: &str, pixels: &TexturePixels<'_>) {
    let (width, height) = (pixels.width(), pixels.height());
    let color = match pixels.format() {
        ColorFormat::RGBA8888 => png::ColorType::Rgba,
        ColorFormat::RGB888 => png::ColorType::Rgb,
        other => panic!("unsupported pixel format {other:?}"),
    };
    let row_bytes = usize::from(width) * match color {
        png::ColorType::Rgba => 4,
        _ => 3,
    };
    let stride = pixels.stride();
    let bytes = pixels.bytes();
    assert!(
        bytes.len() >= stride * usize::from(height),
        "the captured pixels are shorter than the window"
    );
    let mut packed = Vec::with_capacity(row_bytes * usize::from(height));
    for row in 0..usize::from(height) {
        let start = row * stride;
        packed.extend_from_slice(&bytes[start..start + row_bytes]);
    }

    let mut buffer = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut buffer, u32::from(width), u32::from(height));
        encoder.set_color(color);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().expect("png header");
        writer.write_image_data(&packed).expect("png data");
    }
    let path = std::path::Path::new("artifacts").join(name);
    std::fs::write(path, &buffer).expect("png written");
}
