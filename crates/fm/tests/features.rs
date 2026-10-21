//! The features the file manager adds on top of a plain browser: menus, sheets,
//! the command palette, drag and drop, and the file operations behind them.

use std::vec::Vec;

use mirui::app::App;
use mirui::render::texture::{ColorFormat, Texture};
use mirui::surface::{DisplayInfo, FramebufferAccess, InputEvent, PointerKind, Surface};
use mirui::types::{Fixed, PhysicalRect};

use fm::model::{Command, DialogKind, DropTarget, FmStateHandle, Place, SortMode, ViewMode};

const WIDTH: u16 = 1180;
const HEIGHT: u16 = 720;

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

/// The rectangle of a widget, by its size.
fn widget_with_width(
    app: &App<ScriptedSurface, mirui::render::SwRendererFactory>,
    width: i32,
) -> Option<mirui::types::Rect> {
    let query = app.world.query::<mirui::ui::ComputedRect>();
    query
        .iter()
        .find(|(_, rect)| rect.0.w.to_int() == width)
        .map(|(_, rect)| rect.0)
}

#[test]
fn the_context_menu_opens_at_the_pointer() {
    let (mut app, state) = app_with(Vec::new());
    let id = state.entry_id(0);
    state.open_menu(id, 300, 180);
    app.render().expect("menu frame");

    let panel = widget_with_width(&app, 214).expect("the menu panel is laid out");
    assert_eq!(panel.x.to_int(), 300);
    assert_eq!(panel.y.to_int(), 180);
    let _ = &panel;
}

#[test]
fn a_context_menu_row_runs_its_action() {
    let (mut app, state) = app_with(Vec::new());
    let id = state.entry_id(0);
    state.open_menu(id, 300, 180);
    app.render().expect("menu frame");

    // The first row of the menu sits below the 4 pixel panel padding.
    app.backend.events = tap(360, 180 + 4 + 12);
    app.backend.cursor = 0;
    app.tick();

    assert_eq!(state.selected(), Some(id), "the menu selected the file");
    assert!(!state.menu_open(), "the menu closed");
}

#[test]
fn a_context_menu_row_can_trash_a_file() {
    let (mut app, state) = app_with(Vec::new());
    let id = state.entry_id(0);
    let before = state.total();
    state.open_menu(id, 300, 180);
    app.render().expect("menu frame");

    // "Move to Trash" is the seventh row of the menu.
    let row = menu_row_centre(&app, 6);
    app.backend.events = tap(row.0, row.1);
    app.backend.cursor = 0;
    app.tick();

    assert_eq!(state.total(), before - 1, "the file left the listing");
    assert!(state.toast_visible(), "the status message is showing");
}

/// The centre of the n-th row of the open context menu.
fn menu_row_centre(
    app: &App<ScriptedSurface, mirui::render::SwRendererFactory>,
    row: usize,
) -> (i32, i32) {
    let query = app.world.query::<mirui::ui::ComputedRect>();
    let panel_entity = query
        .iter()
        .find(|(_, rect)| rect.0.w.to_int() == 214)
        .map(|(entity, _)| entity)
        .expect("the panel entity");
    let children = app
        .world
        .get::<mirui::ui::Children>(panel_entity)
        .expect("the panel children");
    let mut rows: Vec<mirui::types::Rect> = children
        .0
        .iter()
        .filter_map(|child| app.world.get::<mirui::ui::ComputedRect>(*child))
        .map(|rect| rect.0)
        .filter(|rect| rect.h == Fixed::from_int(24))
        .collect();
    rows.sort_by_key(|rect| rect.y);
    let rect = rows[row];
    (
        (rect.x + rect.w / 2).to_int(),
        (rect.y + rect.h / 2).to_int(),
    )
}

#[test]
fn the_new_folder_sheet_creates_a_folder() {
    let (mut app, state) = app_with(Vec::new());
    let before = state.total();
    state.open_dialog(DialogKind::NewFolder);
    state.set_folder_text("Reports");
    state.create_folder("Reports");
    app.render().expect("frame");

    assert_eq!(state.total(), before + 1);
    assert!(
        state
            .visible_ids()
            .iter()
            .any(|id| state.entry_title(*id) == "Reports")
    );
}

#[test]
fn renaming_changes_the_name_of_a_file() {
    let (mut app, state) = app_with(Vec::new());
    let id = state.entry_id(0);
    state.select(id);
    state.set_rename_text("Budget 2027");
    state.rename(id, "Budget 2027");
    app.render().expect("frame");

    assert_eq!(state.entry_title(id), "Budget 2027");
    assert!(state.toast_visible());
}

#[test]
fn the_trash_can_be_emptied_and_restored() {
    let (mut app, state) = app_with(Vec::new());
    let id = state.entry_id(0);
    let before = state.total();
    state.trash(id);
    app.render().expect("frame");
    assert_eq!(state.total(), before - 1);

    state.restore(id);
    app.render().expect("frame");
    assert_eq!(state.total(), before);
}

#[test]
fn the_command_palette_filters_and_runs() {
    let (mut app, state) = app_with(Vec::new());
    state.open_palette();
    state.set_palette_text("trash");
    app.render().expect("frame");

    assert!(state.palette());
    // Only the commands that mention "trash" survive the filter.
    assert!(state.command_matches(12), "Trash matches the filter");
    assert!(!state.command_matches(1), "the view toggle does not match");
    assert_eq!(state.command_count(), 1);

    state.run_command(Command::GoTrash);
    app.render().expect("frame");
    assert_eq!(state.place(), Place::Trash);
    assert!(!state.palette(), "the palette closed after running");
}

#[test]
fn a_dropped_file_moves_to_the_place_under_the_pointer() {
    let (mut app, state) = app_with(Vec::new());
    let id = state.entry_id(0);
    assert_eq!(state.place_of(id), Place::Documents);

    state.start_drag(id, 400, 300);
    state.set_drop_target(Some(DropTarget::Place(Place::Projects)));
    state.end_drag();
    app.render().expect("frame");

    assert_eq!(state.place_of(id), Place::Projects);
    assert!(state.toast_visible());

    // The file is gone from Documents and shows up in Projects.
    state.set_place(Place::Documents);
    app.render().expect("frame");
    assert!(!state.visible_ids().contains(&id));

    state.set_place(Place::Projects);
    app.render().expect("frame");
    assert!(state.visible_ids().contains(&id));
}

#[test]
fn a_file_dropped_on_a_folder_stays_in_the_same_place() {
    let (mut app, state) = app_with(Vec::new());
    let id = state.entry_id(0);
    // The first tile of Documents is a folder.
    assert!(state.entry_is_kind(id, 0));

    state.start_drag(state.entry_id(20), 400, 300);
    state.set_drop_target(Some(DropTarget::Tile(0)));
    state.end_drag();
    app.render().expect("frame");

    assert_eq!(state.place_of(state.entry_id(20)), Place::Documents);
}

#[test]
fn the_inspector_and_the_options_panel_toggle() {
    let (mut app, state) = app_with(Vec::new());
    assert!(!state.inspector());
    assert!(!state.options_open());

    state.toggle_inspector();
    state.toggle_options();
    app.render().expect("frame");
    assert!(state.inspector());
    assert!(state.options_open());

    state.toggle_inspector();
    state.toggle_options();
    app.render().expect("frame");
    assert!(!state.inspector());
    assert!(!state.options_open());
}

#[test]
fn the_icon_size_and_the_sort_order_change_the_listing() {
    let (mut app, state) = app_with(Vec::new());
    let before = state.grid_indices();
    state.set_icon_size(64);
    state.set_sort(SortMode::Date);
    app.render().expect("frame");

    assert_eq!(state.icon_size(), 64);
    assert_eq!(state.sort_label(), "Sorted by Date");
    assert_eq!(state.grid_indices().len(), before.len());
    assert_eq!(state.grid_rows(), 8);
    assert_eq!(state.grid_height(), 8 * 98 + 7 * 10);
}

#[test]
fn the_view_switch_rebuilds_the_listing() {
    let (mut app, state) = app_with(Vec::new());
    assert_eq!(state.view(), ViewMode::Icons);
    assert_eq!(state.shown(), fm::model::ICON_GRID_MAX);
    assert_eq!(state.list_count(), state.total());

    state.set_view(ViewMode::List);
    app.render().expect("frame");
    assert_eq!(state.view(), ViewMode::List);
    assert_eq!(state.shown(), state.total());
}
