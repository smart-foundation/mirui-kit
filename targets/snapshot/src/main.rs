//! Headless snapshots of the file manager.
//!
//! Renders a fixed set of states into `artifacts/*.png` so the UI can be
//! reviewed — and diffed — without a window server.

use std::cell::RefCell;

use mirui::app::App;
use mirui::prelude::*;
use mirui::render::texture::ColorFormat;
use mirui::surface::framebuf::FramebufSurface;
use mirui::types::PhysicalRect;

use fm::model::{Place, ViewMode};

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
    let state = fm::install(&mut app);

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

    println!("wrote 13 snapshots to artifacts/");
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
