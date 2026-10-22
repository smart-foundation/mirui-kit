//! The things a user actually notices: that a hovered row is visibly
//! different, and that a scrolling area shows a scroll bar.

use std::vec::Vec;

use mirui::app::App;
use mirui::render::texture::{ColorFormat, Texture};
use mirui::surface::{DisplayInfo, FramebufferAccess, InputEvent, PointerKind, Surface};
use mirui::types::{Fixed, PhysicalRect};

const WIDTH: u16 = 1180;
const HEIGHT: u16 = 720;

struct Scripted {
    events: Vec<InputEvent>,
    cursor: usize,
    pixels: Vec<u8>,
}

impl mirui::core::cache::InspectCaches for Scripted {}

impl Surface for Scripted {
    fn display_info(&self) -> DisplayInfo {
        DisplayInfo {
            width: WIDTH,
            height: HEIGHT,
            scale: Fixed::ONE,
            format: ColorFormat::RGBA8888,
        }
    }
    fn flush(&mut self, _area: PhysicalRect) -> Result<(), mirui::render::RenderError> {
        Ok(())
    }
    fn poll_event(&mut self) -> Option<InputEvent> {
        let event = self.events.get(self.cursor).cloned();
        self.cursor += 1;
        event
    }
}

impl FramebufferAccess for Scripted {
    fn framebuffer(&mut self) -> Texture<'_> {
        Texture::new(&mut self.pixels, WIDTH, HEIGHT, ColorFormat::RGBA8888).expect("pixels")
    }
}

fn app() -> (App<Scripted, mirui::render::SwRendererFactory>, fm::model::FmStateHandle) {
    let mut app = App::new(Scripted {
        events: Vec::new(),
        cursor: 0,
        pixels: vec![0; WIDTH as usize * HEIGHT as usize * 4],
    });
    app.with_default_widgets().with_default_systems();
    app.add_plugin(mirui::app::plugins::StdInstantClockPlugin);
    let state = fm::install(&mut app);
    app.render().expect("frame");
    (app, state)
}

/// The pixel at a point of the current frame.
fn pixel(app: &App<Scripted, mirui::render::SwRendererFactory>, x: i32, y: i32) -> (u8, u8, u8) {
    let index = ((y as usize) * WIDTH as usize + x as usize) * 4;
    let p = &app.backend.pixels[index..index + 4];
    (p[0], p[1], p[2])
}

#[test]
fn a_hovered_tile_is_visibly_different() {
    let (mut app, _state) = app();

    // The first tile of the icon grid.
    let query = app.world.query::<mirui::ui::ComputedRect>();
    let mut tiles: Vec<mirui::types::Rect> = query
        .iter()
        .filter(|(_, rect)| rect.0.w.to_int() == 104 && rect.0.h.to_int() == 98)
        .map(|(_, rect)| rect.0)
        .collect();
    tiles.sort_by_key(|rect| (rect.y.to_int(), rect.x.to_int()));
    let tile = tiles[0];
    // The corner of the tile: the icon covers the middle, the hover fills
    // the whole tile.
    let x = (tile.x + Fixed::from_int(4)).to_int();
    let y = (tile.y + Fixed::from_int(4)).to_int();

    let before = pixel(&app, x, y);

    // Move the pointer onto the tile.
    app.backend.events = vec![InputEvent::PointerMove {
        kind: PointerKind::Mouse,
        id: 0,
        x: Fixed::from_int(x),
        y: Fixed::from_int(y),
    }];
    app.backend.cursor = 0;
    app.tick();
    app.render().expect("hover frame");

    let after = pixel(&app, x, y);
    assert_ne!(
        before, after,
        "the tile did not change under the pointer ({before:?} -> {after:?})"
    );
}

#[test]
fn a_scrolling_area_draws_a_scroll_bar() {
    let (mut app, state) = app();
    state.set_view(fm::model::ViewMode::List);
    app.tick();
    app.render().expect("list frame");

    // The thumb sits on the trailing edge of the content area.
    let edge = pixel(&app, 1173, 200);
    assert!(
        edge.0 > 60,
        "no scroll bar on the trailing edge, found {edge:?}"
    );
}
