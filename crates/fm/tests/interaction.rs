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
    app.tick();
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
    println!("target {id} entity={entity:?} rect={rect:?} hit={:?}", app.root.and_then(|root| mirui::input::event::hit_test::hit_test(&app.world, root,rect.x+rect.w/2,rect.y+rect.h/2,WIDTH,HEIGHT)));
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
    app.tick();
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
                && rect.0.h == Fixed::from_int(32)
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
                && rect.0.w > Fixed::ZERO
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
fn dismissing_the_context_menu_does_not_activate_the_file_behind_it() {
    let (mut app, state) = app_with(Vec::new());
    let id = state.entry_id(0);
    state.open_menu_for(id, 300, 180);
    app.render().expect("menu frame");
    assert!(state.menu_open(), "the menu did not open");

    // Outside taps dismiss only; activating the file requires a new tap.
    let (x, y) = first_tile_centre(&app);
    app.backend.events = tap(x, y);
    app.backend.cursor = 0;
    app.tick();

    assert!(!state.menu_open(), "the menu was not dismissed");
    assert_eq!(
        state.selected(),
        None,
        "the dismissal tap activated a file behind the menu"
    );
}

#[test]
fn options_backdrop_dismisses_without_selecting_a_file() {
    let (mut app, state) = app_with(Vec::new());
    state.set_options_open(true);
    app.render().expect("options frame");
    let (x, y) = first_tile_centre(&app);
    app.backend.events = tap(x, y);
    app.backend.cursor = 0;
    app.tick();
    assert!(!state.options_open());
    assert_eq!(state.selected(), None);
    assert_eq!(state.dragging(), None);
}

#[test]
fn opening_an_overlay_cancels_a_drag_without_moving_the_file() {
    let (mut app, state) = app_with(Vec::new());
    let id = state.entry_id(0);
    let original = state.place_of(id);
    state.start_drag(id, 320, 200);
    state.set_drop_target(Some(fm::model::DropTarget::Place(Place::Projects)));
    state.open_palette();
    app.tick();
    assert_eq!(state.dragging(), None);
    assert_eq!(state.drop_target(), None);
    assert_eq!(state.place_of(id), original);
}

fn entity_named(
    app: &App<ScriptedSurface, mirui::render::SwRendererFactory>,
    name: &'static str,
) -> mirui::ecs::Entity {
    app.world
        .resource::<mirui::ui::IdMap>()
        .unwrap()
        .get(name)
        .unwrap()
}

#[test]
fn search_has_real_padding_and_a_centered_native_line_box() {
    let (mut app, _) = app_with(Vec::new());
    app.tick();
    app.render().unwrap();
    let frame = entity_named(&app, "toolbar_search_frame");
    let editor = entity_named(&app, "toolbar_search");
    let outer = app.world.get::<mirui::ui::ComputedRect>(frame).unwrap().0;
    let inner = app.world.get::<mirui::ui::ComputedRect>(editor).unwrap().0;
    assert_eq!(outer.h, Fixed::from_int(36));
    assert!(inner.x >= outer.x + Fixed::from_int(10));
    assert!(((inner.y + inner.h / 2) - (outer.y + outer.h / 2)).abs() <= Fixed::ONE);
    // Installing the editing layer must not replace the App's one active root.
    let mut node = editor;
    while let Some(parent) = app.world.get::<mirui::ui::Parent>(node) {
        node = parent.0;
    }
    assert_eq!(Some(node), app.root);
}

#[test]
fn padding_click_focuses_the_native_editor() {
    let (mut app, _) = app_with(Vec::new());
    let frame = entity_named(&app, "toolbar_search_frame");
    let rect = app.world.get::<mirui::ui::ComputedRect>(frame).unwrap().0;
    println!("padding frame={frame:?} rect={rect:?} hit={:?}",app.root.and_then(|root|mirui::input::event::hit_test::hit_test(&app.world,root,rect.x+Fixed::from_int(3),rect.y+rect.h/2,WIDTH,HEIGHT)));
    app.backend.events = tap(rect.x.to_int() + 3, (rect.y + rect.h / 2).to_int());
    app.backend.cursor = 0;
    app.tick();
    let editor = entity_named(&app, "toolbar_search");
    assert_eq!(
        app.world
            .resource::<mirui::input::event::focus::FocusState>()
            .unwrap()
            .focused,
        Some(editor)
    );
}

#[test]
fn highlighting_text_automatically_mounts_actions_and_copy_keeps_focus() {
    use mirui::input::event::focus::FocusState;
    use mirui::ui::widgets::{TextEditing, TextSelection};
    let (mut app, state) = app_with(Vec::new());
    state.document().set_text("selection example").unwrap();
    let editor = entity_named(&app, "toolbar_search");
    app.world.resource_mut::<FocusState>().unwrap().focused = Some(editor);
    app.world
        .get_mut::<TextEditing>(editor)
        .unwrap()
        .set_selection(TextSelection {
            anchor: 0,
            cursor: 9,
        })
        .unwrap();
    app.tick();
    app.render().unwrap();
    app.tick();
    app.render().unwrap();
    let tools = app
        .world
        .resource::<mirui_kit::selection::SelectionToolsResource>()
        .unwrap()
        .0
        .clone();
    assert!(tools.visible());
    let (x, y) = named_centre(&app, "text_selection_copy");
    app.backend.events = tap(x, y);
    app.backend.cursor = 0;
    app.tick();
    assert_eq!(
        app.world.resource::<FocusState>().unwrap().focused,
        Some(editor)
    );
    assert!(!tools.visible());
    assert_eq!(state.document_text(), "selection example");
}

#[test]
fn dialog_focus_owns_editing_and_closing_restores_search() {
    use mirui::input::event::focus::FocusState;
    let (mut app, state) = app_with(Vec::new());
    let search = entity_named(&app, "toolbar_search");
    app.world.resource_mut::<FocusState>().unwrap().focused = Some(search);
    state.open_dialog(fm::model::DialogKind::NewFolder);
    app.tick();
    app.render().unwrap();
    app.tick();
    app.render().unwrap();
    let dialog = entity_named(&app, "sheet_new_folder");
    assert_eq!(
        app.world.resource::<FocusState>().unwrap().focused,
        Some(dialog)
    );
    assert!(matches!(
        app.world.get::<mirui::ui::UserState>(search),
        Some(mirui::ui::UserState::Disabled)
    ));
    state.close_dialog();
    app.tick();
    app.render().unwrap();
    app.tick();
    app.render().unwrap();
    assert_eq!(
        app.world.resource::<FocusState>().unwrap().focused,
        Some(search)
    );
    assert!(app.world.get::<mirui::ui::UserState>(search).is_none());
}

#[test]
fn search_clear_updates_document_and_filter_without_blurring() {
    let (mut app, state) = app_with(Vec::new());
    state.document().set_text("report").unwrap();
    state.set_query("report");
    let search = entity_named(&app, "toolbar_search");
    app.world
        .resource_mut::<mirui::input::event::focus::FocusState>()
        .unwrap()
        .focused = Some(search);
    app.tick();
    app.render().unwrap();
    let (x, y) = named_centre(&app, "toolbar_search_clear");
    app.backend.events = tap(x, y);
    app.backend.cursor = 0;
    app.tick();
    assert_eq!(state.document_text(), "");
    assert_eq!(state.query_text(), "");
    assert_eq!(
        app.world
            .resource::<mirui::input::event::focus::FocusState>()
            .unwrap()
            .focused,
        Some(search)
    );
}

#[test]
fn inline_naming_keeps_nonmodal_search_enabled() {
    let (mut app, state) = app_with(Vec::new());
    state.new_folder_inline();
    app.tick();
    app.tick();
    let search = entity_named(&app, "toolbar_search");
    assert!(state.editing_folder_id().is_some());
    assert!(app.world.get::<mirui::ui::UserState>(search).is_none());
}
