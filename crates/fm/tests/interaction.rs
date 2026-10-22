//! End-to-end interaction tests.
//!
//! A scripted surface feeds pointer events into the application, so the tests
//! exercise the real hit testing, the real `on Tap` handlers and the real
//! reactive updates — not the model in isolation.

use std::vec::Vec;

use mirui::app::App;
use mirui::render::texture::{ColorFormat, Texture};
use mirui::surface::{DisplayInfo, FramebufferAccess, InputEvent, PointerKind, Surface};
use mirui::types::{Fixed, PhysicalRect};

use fm::model::{FmStateHandle, Place, ViewMode};

const WIDTH: u16 = 1180;
const HEIGHT: u16 = 720;

/// A surface that hands out a fixed script of input events.
struct ScriptedSurface {
    events: Vec<InputEvent>,
    cursor: usize,
    pixels: Vec<u8>,
}

impl ScriptedSurface {
    fn new(events: Vec<InputEvent>) -> Self {
        Self {
            events,
            cursor: 0,
            pixels: vec![0; WIDTH as usize * HEIGHT as usize * 4],
        }
    }
}

impl mirui::core::cache::InspectCaches for ScriptedSurface {}

impl Surface for ScriptedSurface {
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

impl FramebufferAccess for ScriptedSurface {
    fn framebuffer(&mut self) -> Texture<'_> {
        Texture::new(&mut self.pixels, WIDTH, HEIGHT, ColorFormat::RGBA8888)
            .expect("valid pixel storage")
    }
}

fn tap(x: i32, y: i32) -> Vec<InputEvent> {
    let point = Fixed::from_int(x);
    vec![
        InputEvent::PointerDown {
            kind: PointerKind::Mouse,
            id: 0,
            x: point,
            y: Fixed::from_int(y),
        },
        InputEvent::PointerUp {
            kind: PointerKind::Mouse,
            id: 0,
            x: point,
            y: Fixed::from_int(y),
        },
    ]
}

fn app_with(
    events: Vec<InputEvent>,
) -> (
    App<ScriptedSurface, mirui::render::SwRendererFactory>,
    FmStateHandle,
) {
    let mut app = App::new(ScriptedSurface::new(events));
    app.with_default_widgets().with_default_systems();
    let state = fm::install(&mut app);
    app.render().expect("first frame");
    (app, state)
}

/// Centre of a widget that carries a stable identity.
fn named_centre(
    app: &App<ScriptedSurface, mirui::render::SwRendererFactory>,
    id: &'static str,
) -> (i32, i32) {
    let ids = app.world.resource::<mirui::ui::IdMap>().expect("id map");
    let entity = ids
        .get(id)
        .unwrap_or_else(|| panic!("no widget named {id}"));
    let rect = app
        .world
        .get::<mirui::ui::ComputedRect>(entity)
        .unwrap_or_else(|| panic!("{id} has no layout rect"))
        .0;
    (
        (rect.x + rect.w / 2).to_int(),
        (rect.y + rect.h / 2).to_int(),
    )
}

#[test]
fn tapping_the_theme_button_flips_the_palette() {
    let (mut app, state) = app_with(Vec::new());
    assert!(state.dark());

    let (x, y) = named_centre(&app, "toolbar_theme");
    app.backend.events = tap(x, y);
    app.backend.cursor = 0;
    app.tick();

    assert!(!state.dark(), "the theme button did not flip the state");
}

#[test]
fn tapping_a_sidebar_row_changes_the_place() {
    let (mut app, state) = app_with(Vec::new());
    assert_eq!(state.place(), Place::Documents);

    // The third sidebar row is "Downloads".
    let (x, y) = sidebar_row_centre(&app, 2);
    app.backend.events = tap(x, y);
    app.backend.cursor = 0;
    app.tick();

    assert_eq!(state.place(), Place::Downloads);
}

#[test]
fn tapping_a_file_tile_selects_it() {
    let (mut app, state) = app_with(Vec::new());

    let (x, y) = first_tile_centre(&app);
    app.backend.events = tap(x, y);
    app.backend.cursor = 0;
    app.tick();

    let expected = state.entry_id(0);
    assert_eq!(state.selected(), Some(expected));
}

#[test]
fn tapping_the_list_view_button_switches_the_layout() {
    let (mut app, state) = app_with(Vec::new());
    assert_eq!(state.view(), ViewMode::Icons);

    let (x, y) = named_centre(&app, "toolbar_view_list");
    app.backend.events = tap(x, y);
    app.backend.cursor = 0;
    app.tick();

    assert_eq!(state.view(), ViewMode::List);
}

#[test]
fn a_status_message_does_not_swallow_the_next_tap() {
    let (mut app, state) = app_with(Vec::new());

    // Raise a status message the way the toolbar does.
    state.trash(state.entry_id(3));
    assert!(state.toast_visible(), "the message was not raised");

    // The message floats over the window: the tap below it still has to
    // reach the tile.
    app.render().expect("the frame with the message");
    let (x, y) = first_tile_centre(&app);
    app.backend.events = tap(x, y);
    app.backend.cursor = 0;
    app.tick();

    assert_eq!(
        state.selected(),
        Some(state.entry_id(0)),
        "the status message swallowed the tap"
    );
}

#[test]
fn a_status_message_dismisses_itself() {
    let mut app = App::new(ScriptedSurface::new(Vec::new()));
    app.with_default_widgets().with_default_systems();
    // The countdown runs on the wall clock, so the application needs one.
    app.add_plugin(mirui::app::plugins::StdInstantClockPlugin);
    let state = fm::install(&mut app);
    app.add_plugin(fm::sync::ToastDismissPlugin::new(state.clone()));
    app.render().expect("first frame");

    state.trash(state.entry_id(3));
    assert!(state.toast_visible(), "the message was not raised");

    // The message goes after about two and a half seconds.
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while state.toast_visible() && std::time::Instant::now() < deadline {
        app.tick();
        std::thread::sleep(std::time::Duration::from_millis(20));
    }

    assert!(!state.toast_visible(), "the message did not dismiss itself");
}

/// Centre of the n-th sidebar row (0 based).
fn sidebar_row_centre(
    app: &App<ScriptedSurface, mirui::render::SwRendererFactory>,
    row: usize,
) -> (i32, i32) {
    let panel = {
        let ids = app.world.resource::<mirui::ui::IdMap>().expect("id map");
        let entity = ids.get("sidebar").expect("sidebar");
        app.world
            .get::<mirui::ui::ComputedRect>(entity)
            .expect("rect")
            .0
    };
    let mut rows: Vec<_> = app
        .world
        .query::<mirui::ui::ComputedRect>()
        .iter()
        .filter(|(_, rect)| {
            rect.0.x >= panel.x
                && rect.0.x < panel.x + panel.w
                && rect.0.y > panel.y + Fixed::from_int(20)
                && rect.0.h == Fixed::from_int(28)
        })
        .map(|(_, rect)| rect.0)
        .collect();
    rows.sort_by_key(|rect| rect.y);
    let rect = rows[row];
    (
        (rect.x + rect.w / 2).to_int(),
        (rect.y + rect.h / 2).to_int(),
    )
}

/// Centre of the first file tile in the icon grid.
fn first_tile_centre(app: &App<ScriptedSurface, mirui::render::SwRendererFactory>) -> (i32, i32) {
    let panel = {
        let ids = app.world.resource::<mirui::ui::IdMap>().expect("id map");
        let entity = ids.get("sidebar").expect("sidebar");
        app.world
            .get::<mirui::ui::ComputedRect>(entity)
            .expect("rect")
            .0
    };
    let mut tiles: Vec<_> = app
        .world
        .query::<mirui::ui::ComputedRect>()
        .iter()
        .filter(|(_, rect)| {
            rect.0.x >= panel.x + panel.w
                && rect.0.w == Fixed::from_int(104)
                && rect.0.h == Fixed::from_int(98)
        })
        .map(|(_, rect)| rect.0)
        .collect();
    tiles.sort_by_key(|rect| (rect.y, rect.x));
    let rect = tiles[0];
    (
        (rect.x + rect.w / 2).to_int(),
        (rect.y + rect.h / 2).to_int(),
    )
}

#[test]
fn the_context_menu_does_not_swallow_a_tap_outside_it() {
    let (mut app, state) = app_with(Vec::new());
    let id = state.entry_id(0);
    state.open_menu_for(id, 300, 180);
    app.render().expect("menu frame");
    assert!(state.menu_open(), "the menu did not open");

    // The backdrop of the menu covers the window: the tap below it still
    // has to reach the tile, and the menu has to go.
    let (x, y) = first_tile_centre(&app);
    app.backend.events = tap(x, y);
    app.backend.cursor = 0;
    app.tick();

    assert!(!state.menu_open(), "the menu was not dismissed");
    assert_eq!(
        state.selected(),
        Some(state.entry_id(0)),
        "the tap did not reach the tile"
    );
}
