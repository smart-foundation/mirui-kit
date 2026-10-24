//! The features the file manager adds on top of a plain browser: menus, sheets,
//! the command palette, drag and drop, and the file operations behind them.

use std::vec::Vec;

use mirui::app::App;
use mirui::render::texture::{ColorFormat, Texture};
use mirui::surface::{DisplayInfo, FramebufferAccess, InputEvent, PointerKind, Surface};
use mirui::types::{Fixed, PhysicalRect};

use fm::model::{
    Command, DialogKind, DropTarget, FmStateHandle, MenuTarget, NoticeKind, Place, SortMode,
    ViewMode,
};

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
    // The gesture recogniser needs a clock to recognise a long press.
    app.add_plugin(mirui::app::plugins::StdInstantClockPlugin);
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
    let entity = app.world.query::<fm::ui::menu::MenuRow>().iter()
        .find(|(_, marker)| marker.item as usize == row)
        .map(|(entity, _)| entity).expect("menu action is mounted");
    let rect = app.world.get::<mirui::ui::ComputedRect>(entity)
        .expect("menu row is laid out").0;
    (
        (rect.x + rect.w / 2).to_int(),
        (rect.y + rect.h / 2).to_int(),
    )
}

#[test]
fn a_right_click_opens_the_context_menu() {
    let (mut app, state) = app_with(Vec::new());
    let id = state.entry_id(0);

    // The centre of the first tile of the icon grid.
    let query = app.world.query::<mirui::ui::ComputedRect>();
    let mut tiles: Vec<mirui::types::Rect> = query
        .iter()
        .filter(|(_, rect)| rect.0.w.to_int() == 104 && rect.0.h.to_int() == 98)
        .map(|(_, rect)| rect.0)
        .collect();
    tiles.sort_by_key(|rect| (rect.y.to_int(), rect.x.to_int()));
    let tile = tiles[0];
    let x = (tile.x + tile.w / 2).to_int();
    let y = (tile.y + tile.h / 2).to_int();

    app.backend.events = vec![InputEvent::PointerDown {
        kind: PointerKind::Mouse,
        id: 0,
        x: Fixed::from_int(x),
        y: Fixed::from_int(y),
    }];
    app.backend.cursor = 0;
    app.tick();

    // The desktop shell turns a right click into this call.
    state.select(id);
    state.open_menu_at_pointer(x, y);
    app.tick();

    assert!(state.menu_open(), "the right click opened the context menu");
    assert_eq!(
        state.menu_target(),
        Some(MenuTarget::Item(id)),
        "the menu is for the file"
    );
}

#[test]
fn dragging_a_file_onto_a_sidebar_place_moves_it() {
    let (mut app, state) = app_with(Vec::new());
    let id = state.entry_id(0);

    // The centre of the first tile of the icon grid.
    let query = app.world.query::<mirui::ui::ComputedRect>();
    let mut tiles: Vec<mirui::types::Rect> = query
        .iter()
        .filter(|(_, rect)| rect.0.w.to_int() == 104 && rect.0.h.to_int() == 98)
        .map(|(_, rect)| rect.0)
        .collect();
    tiles.sort_by_key(|rect| (rect.y.to_int(), rect.x.to_int()));
    let tile = tiles[0];
    let from_x = (tile.x + Fixed::from_int(8)).to_int();
    let from_y = (tile.y + Fixed::from_int(8)).to_int();

    // The "Projects" row of the sidebar.
    let ids = app.world.resource::<mirui::ui::IdMap>().expect("ids");
    let sidebar = ids.get("sidebar").expect("the sidebar");
    let sidebar_rect = app
        .world
        .get::<mirui::ui::ComputedRect>(sidebar)
        .expect("the sidebar rect")
        .0;
    let to_x = (sidebar_rect.x + sidebar_rect.w / 2).to_int();
    let to_y = (sidebar_rect.y + Fixed::from_int(130)).to_int();

    let mut events = vec![InputEvent::PointerDown {
        kind: PointerKind::Mouse,
        id: 0,
        x: Fixed::from_int(from_x),
        y: Fixed::from_int(from_y),
    }];
    for step in 1..=6 {
        let x = from_x + (to_x - from_x) * step / 6;
        let y = from_y + (to_y - from_y) * step / 6;
        events.push(InputEvent::PointerMove {
            kind: PointerKind::Mouse,
            id: 0,
            x: Fixed::from_int(x),
            y: Fixed::from_int(y),
        });
    }
    events.push(InputEvent::PointerUp {
        kind: PointerKind::Mouse,
        id: 0,
        x: Fixed::from_int(to_x),
        y: Fixed::from_int(to_y),
    });

    // One event per frame, the way a real pointer arrives.
    for event in events {
        app.backend.events = vec![event];
        app.backend.cursor = 0;
        app.tick();
        let cursor = app.world.resource::<mirui::input::event::PointerCursor>();
        let scroll_state = app
            .world
            .resource::<mirui::input::event::scroll::ScrollDragState>()
            .map(|s| (s.active, s.resolved, s.target));
        println!(
            "cursor={:?} scroll={scroll_state:?} pressed={:?} dragging={:?} drop={:?} pos={:?}",
            cursor.map(|c| (c.x.to_int(), c.y.to_int(), c.down, c.present)),
            state.pressed_tile(),
            state.dragging(),
            state.drop_target(),
            state.drag_position()
        );
    }

    assert_eq!(state.place_of(id), Place::Projects, "the file moved");
    assert!(state.toast_visible());
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

    let folder = state.entry_id(0);
    assert!(
        state.entry_is_kind(folder, 0),
        "the first entry is a folder"
    );
    state.start_drag(state.entry_id(20), 400, 300);
    state.set_drop_target(Some(DropTarget::Tile(folder)));
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
    assert_eq!(state.grid_rows(), 4);
    assert_eq!(state.grid_height(), 4 * 98 + 3 * 10);
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

#[test]
fn the_listing_revision_only_moves_when_the_files_change() {
    let (mut app, state) = app_with(Vec::new());
    let before = state.listing_revision();

    // The view does not change the files, so the grid and the list keep
    // their cells.
    state.set_view(ViewMode::List);
    app.render().expect("frame");
    assert_eq!(state.view(), ViewMode::List);
    assert_eq!(
        state.listing_revision(),
        before,
        "the listing did not change"
    );

    // The icon size does not change the files either.
    state.set_icon_size(64);
    app.render().expect("frame");
    assert_eq!(state.listing_revision(), before, "the listing did not change");

    // A change of the files does move the revision.
    state.set_query("report");
    app.render().expect("frame");
    assert!(
        state.listing_revision() > before,
        "the listing changed, so the revision moved"
    );
}

#[test]
fn the_notifications_stack_up_and_retire() {
    let (_app, state) = app_with(Vec::new());
    state.notify(NoticeKind::Info, "First", "one");
    state.notify(NoticeKind::Success, "Second", "two");
    assert_eq!(state.notice_count(), 2);
    assert_eq!(state.notice_title(0), "First");
    assert_eq!(state.notice_body(0), "one");
    assert_eq!(state.notice_kind(1), NoticeKind::Success);

    // The stack holds three notifications at most: the oldest one leaves.
    state.notify(NoticeKind::Warning, "Third", "three");
    state.notify(NoticeKind::Error, "Fourth", "four");
    assert_eq!(state.notice_count(), 3);
    assert_eq!(state.notice_title(0), "Second");

    // A click dismisses one of them.
    state.dismiss(0);
    assert_eq!(state.notice_count(), 2);
    assert_eq!(state.notice_title(0), "Third");

    // The timer retires the oldest notification after four seconds.
    for _ in 0..fm::model::NOTICE_TICKS {
        state.tick_notices();
    }
    assert_eq!(state.notice_count(), 1, "the oldest notification retired");
    assert_eq!(state.notice_title(0), "Fourth");
}

#[test]
fn a_status_message_is_a_notification() {
    let (_app, state) = app_with(Vec::new());
    state.say("Moved to Trash");
    assert_eq!(state.notice_count(), 1);
    assert_eq!(state.notice_title(0), "Moved to Trash");
    assert!(state.toast_visible(), "the status message is showing");

    state.clear_toast();
    assert_eq!(state.notice_count(), 0);
    assert!(!state.toast_visible());
}

#[test]
fn the_menu_belongs_to_the_file_under_the_pointer() {
    let (_app, state) = app_with(Vec::new());
    let hovered = state.entry_id(0);
    let selected = state.entry_id(1);
    state.set_hovered(Some(hovered));
    state.select(selected);

    state.open_menu_at_pointer(300, 180);
    assert_eq!(
        state.menu_target(),
        Some(MenuTarget::Item(hovered)),
        "the menu belongs to the file under the pointer"
    );

    // Without a file under the pointer the selection is used.
    state.close_menu();
    state.set_hovered(None);
    state.open_menu_at_pointer(300, 180);
    assert_eq!(state.menu_target(), Some(MenuTarget::Item(selected)));

    // Without a file and without a selection the empty space gets its own
    // menu.
    state.close_menu();
    state.select_optional(None);
    state.open_menu_at_pointer(300, 180);
    assert_eq!(state.menu_target(), Some(MenuTarget::Background));
    assert!(state.menu_open());
}

#[test]
fn a_screenshot_request_changes_nothing_else() {
    let (mut app, state) = app_with(Vec::new());
    let id = state.entry_id(0);
    state.select(id);
    state.open_dialog(DialogKind::GetInfo);
    app.render().expect("frame");

    let before = state.screenshot_requested();
    state.request_screenshot();
    assert_eq!(state.screenshot_requested(), before + 1);

    // The sheet is still open and the selection did not move: the frame the
    // host captures is the frame the user is looking at.
    assert_eq!(state.dialog(), Some(DialogKind::GetInfo));
    assert_eq!(state.selected(), Some(id));
    assert!(!state.menu_open());
    assert_eq!(state.dragging(), None);
}



#[test]
fn clicking_a_notification_dismisses_it() {
    let (mut app, state) = app_with(Vec::new());
    state.notify(NoticeKind::Info, "First", "one");
    state.notify(NoticeKind::Info, "Second", "two");
    app.render().expect("frame");

    // The cards of the stack, the newest one at the bottom.
    let query = app.world.query::<mirui::ui::ComputedRect>();
    let mut cards: Vec<mirui::types::Rect> = query
        .iter()
        .filter(|(_, rect)| rect.0.w.to_int() == 320)
        .map(|(_, rect)| rect.0)
        .collect();
    cards.sort_by_key(|rect| rect.y.to_int());
    let card = cards[0];

    app.backend.events = tap(
        (card.x + card.w / 2).to_int(),
        (card.y + card.h / 2).to_int(),
    );
    app.backend.cursor = 0;
    app.tick();

    assert_eq!(state.notice_count(), 1, "the notification was dismissed");
    assert_eq!(state.notice_title(0), "Second");
}


#[test]
fn every_context_action_fits_inside_the_menu_panel() {
    let (mut app, state)=app_with(Vec::new());
    state.open_menu(state.entry_id(0),300,180);
    app.render().expect("menu frame");
    let panel=app.world.resource::<mirui::ui::IdMap>().unwrap().get("menu_panel").unwrap();
    let bounds=app.world.get::<mirui::ui::ComputedRect>(panel).unwrap().0;
    let mut count=0;
    for (entity,_) in app.world.query::<fm::ui::menu::MenuRow>().iter() {
        let rect=app.world.get::<mirui::ui::ComputedRect>(entity).unwrap().0;
        assert!(rect.y>=bounds.y && rect.y+rect.h<=bounds.y+bounds.h,"context action is clipped");
        count+=1;
    }
    assert_eq!(count,8);
}
