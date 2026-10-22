//! Application state.
//!
//! One registered model instance owns everything the UI reacts to. Selection,
//! search text, the current place and the view mode are *observed*, so the
//! reactive bindings in the widget tree re-evaluate when they change.
//!
//! On top of the static fake filesystem the model keeps the things a user can
//! actually change: relocations, renames, trashed files, newly created
//! folders, the open menu, the open dialog, the command palette, a drag in
//! flight and the transient status message.

use alloc::borrow::ToOwned;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use mirui::prelude::*;
use mirui::ui::widgets::TextDocument;

use crate::data::{self, Kind};

/// A sidebar place.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Place {
    Favourites,
    Documents,
    Downloads,
    Projects,
    Trash,
}

/// How the content area lays files out.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ViewMode {
    Icons,
    List,
}

/// How the file list is ordered.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SortMode {
    Name,
    Date,
    Size,
    Kind,
}

/// Where a dragged file would land.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DropTarget {
    /// A folder tile in the content area.
    Tile(u32),
    /// A place in the sidebar.
    Place(Place),
}

/// Which modal sheet is open.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DialogKind {
    NewFolder,
    Rename,
    Delete,
    About,
    GetInfo,
    Error,
}

/// A command offered by the command palette.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Command {
    NewFolder,
    ToggleView,
    ToggleTheme,
    ToggleInspector,
    ToggleHidden,
    SortByName,
    SortByDate,
    SortBySize,
    GoFavourites,
    GoDocuments,
    GoDownloads,
    GoProjects,
    GoTrash,
    About,
    Dismiss,
}

/// Every palette command: label, shortcut hint and what it does.
pub const COMMANDS: [(&str, &str, Command); 15] = [
    ("New Folder", "Cmd N", Command::NewFolder),
    ("Toggle Icon or List View", "Cmd 1", Command::ToggleView),
    ("Toggle Light or Dark", "Cmd T", Command::ToggleTheme),
    ("Show or Hide Inspector", "Cmd I", Command::ToggleInspector),
    ("Show or Hide Hidden Files", "Cmd H", Command::ToggleHidden),
    ("Sort by Name", "", Command::SortByName),
    ("Sort by Date", "", Command::SortByDate),
    ("Sort by Size", "", Command::SortBySize),
    ("Favourites", "", Command::GoFavourites),
    ("Documents", "", Command::GoDocuments),
    ("Downloads", "", Command::GoDownloads),
    ("Projects", "", Command::GoProjects),
    ("Trash", "", Command::GoTrash),
    ("About this Demo", "", Command::About),
    ("Dismiss", "Esc", Command::Dismiss),
];

/// Upper bound on files drawn in the icon grid. Five rows of eight tiles fit
/// on a 720 pixel window without scrolling, and every tile is a widget the
/// renderer has to draw, so the grid stays small on purpose.
pub const ICON_GRID_MAX: u32 = 40;

/// A search result is a short list: drawing forty hits is noise, and the
/// rebuild of forty tiles is what makes typing feel heavy.
pub const SEARCH_GRID_MAX: u32 = 12;

/// How many tiles fit in one row of the icon grid.
pub const GRID_COLUMNS: u32 = 8;

/// Height of one row of tiles, including the gap below it.
pub const GRID_ROW_HEIGHT: i32 = 108;

/// Height of one tile.
pub const TILE_HEIGHT: i32 = 98;

/// Ids of user created folders start here, above the static table.
pub const CREATED_BASE: u32 = 1_000_000;

/// A folder the user created in this session.
#[derive(Clone, Debug)]
pub struct CreatedFolder {
    pub id: u32,
    pub name: String,
    pub place: Place,
    pub day: u8,
}

#[model]
pub struct FmState {
    /// Backing document of the search field.
    pub document: model!(owned TextDocument),
    /// Backing document of the command palette.
    pub palette_document: model!(owned TextDocument),
    /// Backing document of the rename field.
    pub rename_document: model!(owned TextDocument),
    /// Backing document of the new folder field.
    pub folder_document: model!(owned TextDocument),
    query: String,
    history: Vec<Place>,
    visible: Vec<u32>,
    relocations: Vec<(u32, Place)>,
    renames: Vec<(u32, String)>,
    trashed: Vec<u32>,
    created: Vec<CreatedFolder>,
    #[observe]
    pub menu_x: i32,
    #[observe]
    pub menu_y: i32,
    #[observe]
    pub drag_x: i32,
    #[observe]
    pub drag_y: i32,
    toast: Option<String>,
    #[observe]
    pub place: Place,
    #[observe]
    pub selected: Option<u32>,
    #[observe]
    pub view: ViewMode,
    #[observe]
    pub dark: bool,
    #[observe]
    pub shown: u32,
    #[observe]
    pub total: u32,
    #[observe]
    pub revision: u32,
    /// Bumped when the listing changes: the grid and the list rebuild.
    #[observe]
    pub listing_revision: u32,
    /// Bumped when a name changes: only the labels re-measure.
    #[observe]
    pub names_revision: u32,
    /// Bumped when the palette filter changes.
    #[observe]
    pub palette_revision: u32,
    /// The tile the pointer is pressing on, if any.
    pressed: Option<u32>,
    press_x: i32,
    press_y: i32,
    #[observe]
    pub menu: Option<u32>,
    #[observe]
    pub dialog: Option<DialogKind>,
    #[observe]
    pub palette: bool,
    #[observe]
    pub dragging: Option<u32>,
    #[observe]
    pub drop_target: Option<DropTarget>,
    #[observe]
    pub inspector: bool,
    #[observe]
    pub options_open: bool,
    #[observe]
    pub icon_size: u16,
    #[observe]
    pub show_hidden: bool,
    #[observe]
    pub sort: SortMode,
    #[observe]
    pub toast_visible: bool,
    #[observe]
    pub toast_id: u32,
}

impl FmState {
    /// A fresh state: Documents, icon view, dark theme.
    pub fn new() -> Self {
        Self {
            document: TextDocument::new().into(),
            palette_document: TextDocument::new().into(),
            rename_document: TextDocument::new().into(),
            folder_document: TextDocument::new().into(),
            query: String::new(),
            history: Vec::new(),
            visible: Vec::new(),
            relocations: Vec::new(),
            renames: Vec::new(),
            trashed: Vec::new(),
            created: Vec::new(),
            menu_x: 0,
            menu_y: 0,
            drag_x: 0,
            drag_y: 0,
            toast: None,
            place: Place::Documents,
            selected: None,
            view: ViewMode::Icons,
            dark: true,
            shown: 0,
            total: 0,
            revision: 0,
            listing_revision: 0,
            names_revision: 0,
            palette_revision: 0,
            pressed: None,
            press_x: 0,
            press_y: 0,
            menu: None,
            dialog: None,
            palette: false,
            dragging: None,
            drop_target: None,
            inspector: false,
            options_open: false,
            icon_size: 44,
            show_hidden: false,
            sort: SortMode::Name,
            toast_visible: false,
            toast_id: 0,
        }
    }
}

impl Default for FmState {
    fn default() -> Self {
        Self::new()
    }
}

#[model]
impl FmState {
    /// Whether the back chevron can be used.
    #[observe]
    pub fn can_go_back(&self) -> bool {
        !self.history.is_empty()
    }

    // --- navigation -------------------------------------------------------

    /// Navigate to a place, remembering the previous one.
    pub fn set_place(&mut self, place: Place) {
        if self.place != place {
            self.history.push(self.place);
            self.place = place;
            self.selected = None;
            self.close_menu();
        }
        // The listing can change even when the place does not, so always
        // recount.
        self.recompute();
    }

    /// Return to the previous place.
    pub fn go_back(&mut self) {
        if let Some(previous) = self.history.pop() {
            self.place = previous;
            self.selected = None;
            self.close_menu();
            self.recompute();
        }
    }

    // --- selection --------------------------------------------------------

    /// Select a file by id.
    pub fn select(&mut self, id: u32) {
        self.selected = Some(id);
        self.revision += 1;
    }

    /// Select a file by id, or clear the selection.
    pub fn select_optional(&mut self, id: Option<u32>) {
        self.selected = id;
        self.revision += 1;
    }

    // --- view -------------------------------------------------------------

    /// Switch between the icon grid and the virtualised list.
    pub fn set_view(&mut self, view: ViewMode) {
        if self.view != view {
            self.view = view;
            self.recompute();
        }
    }

    /// Flip between the light and the dark palette.
    pub fn toggle_theme(&mut self) {
        self.dark = !self.dark;
        self.revision += 1;
    }

    /// Show or hide the inspector panel.
    pub fn toggle_inspector(&mut self) {
        self.inspector = !self.inspector;
        self.revision += 1;
    }

    /// Show or hide the hidden files.
    pub fn toggle_hidden(&mut self) {
        self.show_hidden = !self.show_hidden;
        self.recompute();
    }

    /// Choose the sort order.
    pub fn set_sort(&mut self, sort: SortMode) {
        if self.sort != sort {
            self.sort = sort;
            self.recompute();
        }
    }

    /// Choose the icon size, in logical pixels.
    pub fn set_icon_size(&mut self, size: u16) {
        let size = size.clamp(24, 72);
        if self.icon_size != size {
            self.icon_size = size;
            self.revision += 1;
        }
    }

    // --- search -----------------------------------------------------------

    /// Replace the search filter.
    pub fn set_query(&mut self, text: &str) {
        self.query.clear();
        self.query.push_str(text);
        self.recompute();
    }

    /// The current search filter.
    pub fn query_text(&self) -> String {
        self.query.clone()
    }

    /// The text currently held by the search editor.
    pub fn document_text(&self) -> String {
        self.document.read(|text| String::from(text))
    }

    // --- menus ------------------------------------------------------------

    /// Open the context menu for a file at a screen position.
    pub fn open_menu(&mut self, id: u32, x: i32, y: i32) {
        self.menu = Some(id);
        self.menu_x = x;
        self.menu_y = y;
        self.revision += 1;
    }

    /// Open the context menu for the selected file at a pointer position.
    pub fn open_menu_at_pointer(&mut self, x: i32, y: i32) {
        if let Some(id) = self.selected {
            self.open_menu(id, x, y);
        }
    }

    /// Close the context menu.
    pub fn close_menu(&mut self) {
        if self.menu.is_some() {
            self.menu = None;
            self.revision += 1;
        }
    }

    /// The file the open context menu belongs to.
    pub fn menu_target(&self) -> Option<u32> {
        self.menu
    }

    /// Whether a context menu is open.
    pub fn menu_open(&self) -> bool {
        self.menu.is_some()
    }

    /// Screen position of the open context menu.
    pub fn menu_position(&self) -> (i32, i32) {
        (self.menu_x, self.menu_y)
    }

    // --- dialogs ----------------------------------------------------------

    /// Open a modal sheet.
    pub fn open_dialog(&mut self, kind: DialogKind) {
        self.dialog = Some(kind);
        self.menu = None;
        self.revision += 1;
    }

    /// Close the modal sheet.
    pub fn close_dialog(&mut self) {
        if self.dialog.is_some() {
            self.dialog = None;
            self.revision += 1;
        }
    }

    /// The text typed into the rename field.
    pub fn rename_text(&self) -> String {
        self.rename_document.read(|text| String::from(text))
    }

    /// Replace the text in the rename field.
    pub fn set_rename_text(&mut self, text: &str) {
        let _ = self.rename_document.set_text(text);
    }

    /// The text typed into the new folder field.
    pub fn folder_text(&self) -> String {
        self.folder_document.read(|text| String::from(text))
    }

    /// Replace the text in the new folder field.
    pub fn set_folder_text(&mut self, text: &str) {
        let _ = self.folder_document.set_text(text);
    }

    // --- command palette --------------------------------------------------

    /// Open the command palette.
    pub fn open_palette(&mut self) {
        self.palette = true;
        self.menu = None;
        let _ = self.palette_document.set_text("");
        self.palette_revision += 1;
        self.revision += 1;
    }

    /// Close the command palette.
    pub fn close_palette(&mut self) {
        if self.palette {
            self.palette = false;
            let _ = self.palette_document.set_text("");
            self.palette_revision += 1;
            self.revision += 1;
        }
    }

    /// Replace the text in the palette field.
    pub fn set_palette_text(&mut self, text: &str) {
        let _ = self.palette_document.set_text(text);
        self.palette_revision += 1;
    }

    /// The text typed into the palette field.
    pub fn palette_text(&self) -> String {
        self.palette_document.read(|text| String::from(text))
    }

    /// Whether a command matches the palette filter.
    pub fn command_matches(&self, index: usize) -> bool {
        let Some((label, _, _)) = COMMANDS.get(index) else {
            return false;
        };
        let filter = self.palette_text();
        if filter.is_empty() {
            return true;
        }
        data::contains_ci(label, &filter)
    }

    /// How many commands match the palette filter.
    pub fn command_count(&self) -> u32 {
        COMMANDS
            .iter()
            .enumerate()
            .filter(|(index, _)| self.command_matches(*index))
            .count() as u32
    }

    /// Run a command.
    pub fn run_command(&mut self, command: Command) {
        self.close_palette();
        match command {
            Command::NewFolder => self.open_dialog(DialogKind::NewFolder),
            Command::ToggleView => {
                let view = if self.view == ViewMode::Icons {
                    ViewMode::List
                } else {
                    ViewMode::Icons
                };
                self.set_view(view);
            }
            Command::ToggleTheme => self.toggle_theme(),
            Command::ToggleInspector => self.toggle_inspector(),
            Command::ToggleHidden => self.toggle_hidden(),
            Command::SortByName => self.set_sort(SortMode::Name),
            Command::SortByDate => self.set_sort(SortMode::Date),
            Command::SortBySize => self.set_sort(SortMode::Size),
            Command::GoFavourites => self.set_place(Place::Favourites),
            Command::GoDocuments => self.set_place(Place::Documents),
            Command::GoDownloads => self.set_place(Place::Downloads),
            Command::GoProjects => self.set_place(Place::Projects),
            Command::GoTrash => self.set_place(Place::Trash),
            Command::About => self.open_dialog(DialogKind::About),
            Command::Dismiss => {
                self.close_palette();
                self.close_menu();
                self.close_dialog();
            }
        }
    }

    // --- file operations --------------------------------------------------

    /// Move a file into the trash.
    pub fn trash(&mut self, id: u32) {
        if self.trashed.contains(&id) {
            return;
        }
        self.trashed.push(id);
        if self.selected == Some(id) {
            self.selected = None;
        }
        self.close_menu();
        self.recompute();
        self.say("Moved to Trash");
    }

    /// Bring a file back from the trash.
    pub fn restore(&mut self, id: u32) {
        self.trashed.retain(|entry| *entry != id);
        self.recompute();
        self.say("Restored from Trash");
    }

    /// Copy a file into the same place.
    pub fn duplicate(&mut self, id: u32) {
        let name = format!("{} copy", self.entry_title(id));
        self.created.push(CreatedFolder {
            id: CREATED_BASE + self.created.len() as u32,
            name,
            place: self.place_of(id),
            day: 1,
        });
        self.names_revision += 1;
        self.close_menu();
        self.recompute();
        self.say("Duplicated");
    }

    /// Rename a file.
    pub fn rename(&mut self, id: u32, name: &str) {
        let name = name.trim();
        if name.is_empty() {
            return;
        }
        self.renames.retain(|(entry, _)| *entry != id);
        self.renames.push((id, name.to_owned()));
        self.names_revision += 1;
        self.close_dialog();
        self.recompute();
        self.say("Renamed");
    }

    /// Create a new folder in the current place.
    pub fn create_folder(&mut self, name: &str) {
        let name = name.trim();
        if name.is_empty() {
            return;
        }
        self.created.push(CreatedFolder {
            id: CREATED_BASE + self.created.len() as u32,
            name: name.to_owned(),
            place: self.place,
            day: 1,
        });
        self.names_revision += 1;
        self.close_dialog();
        self.recompute();
        self.say("New folder created");
    }

    /// Move a file into another place.
    pub fn move_to(&mut self, id: u32, place: Place) {
        self.relocations.retain(|(entry, _)| *entry != id);
        self.relocations.push((id, place));
        self.close_menu();
        self.recompute();
        self.say("Moved");
    }

    // --- drag and drop ----------------------------------------------------

    /// The pointer went down on a file.
    pub fn press_tile(&mut self, id: u32, x: i32, y: i32) {
        self.pressed = Some(id);
        self.press_x = x;
        self.press_y = y;
    }

    /// The tile the pointer is pressing on.
    pub fn pressed_tile(&self) -> Option<u32> {
        self.pressed
    }

    /// Where the pointer went down.
    pub fn press_position(&self) -> (i32, i32) {
        (self.press_x, self.press_y)
    }

    /// Forget the pressed tile without starting a drag.
    pub fn clear_press(&mut self) {
        self.pressed = None;
    }

    /// A drag started on a file.
    pub fn start_drag(&mut self, id: u32, x: i32, y: i32) {
        self.dragging = Some(id);
        self.drag_x = x;
        self.drag_y = y;
        self.drop_target = None;
        self.revision += 1;
    }

    /// The pointer moved during a drag.
    pub fn drag_to(&mut self, x: i32, y: i32) {
        self.drag_x = x;
        self.drag_y = y;
    }

    /// The pointer position during a drag.
    pub fn drag_position(&self) -> (i32, i32) {
        (self.drag_x, self.drag_y)
    }

    /// Highlight the target under the pointer.
    pub fn set_drop_target(&mut self, target: Option<DropTarget>) {
        if self.drop_target != target {
            self.drop_target = target;
            self.revision += 1;
        }
    }

    /// The drag finished: drop the file on whatever was under the pointer.
    pub fn end_drag(&mut self) {
        self.pressed = None;
        let dragged = self.dragging.take();
        let target = self.drop_target.take();
        if let (Some(dragged), Some(target)) = (dragged, target) {
            let destination = match target {
                DropTarget::Tile(id) => {
                    if self.entry_kind(id) == Kind::Folder {
                        Some(self.place_of(id))
                    } else {
                        None
                    }
                }
                DropTarget::Place(place) => Some(place),
            };
            if let Some(place) = destination
                && self.place_of(dragged) != place
            {
                self.relocations.retain(|(entry, _)| *entry != dragged);
                self.relocations.push((dragged, place));
                self.say("Dropped into folder");
            }
        }
        self.dragging = None;
        self.drop_target = None;
        self.revision += 1;
    }

    // --- status message ---------------------------------------------------

    /// Show a short status message.
    pub fn say(&mut self, message: &str) {
        self.toast = Some(message.to_owned());
        self.toast_visible = true;
        self.toast_id += 1;
        self.revision += 1;
    }

    /// Clear the status message.
    pub fn clear_toast(&mut self) {
        self.toast = None;
        self.toast_visible = false;
        self.toast_id += 1;
        self.revision += 1;
    }

    /// The text of the current status message.
    pub fn toast_text(&self) -> String {
        self.toast.clone().unwrap_or_default()
    }

    // --- options popover --------------------------------------------------

    /// Show or hide the view options.
    pub fn set_options_open(&mut self, open: bool) {
        if self.options_open != open {
            self.options_open = open;
            self.revision += 1;
        }
    }

    /// Flip the view options.
    pub fn toggle_options(&mut self) {
        let open = !self.options_open;
        self.set_options_open(open);
    }

    // --- derived labels ---------------------------------------------------

    /// The name of the file being dragged.
    pub fn drag_label(&self) -> String {
        match self.dragging {
            Some(id) => self.entry_title(id),
            None => String::new(),
        }
    }

    /// Name of the selected item.
    pub fn info_name(&self) -> String {
        match self.selected {
            Some(id) => self.entry_title(id),
            None => String::from("Nothing selected"),
        }
    }

    /// Kind of the selected item.
    pub fn info_kind(&self) -> String {
        match self.selected {
            Some(id) => String::from(self.entry_kind(id).label()),
            None => String::new(),
        }
    }

    /// Size of the selected item.
    pub fn info_size(&self) -> String {
        match self.selected {
            Some(id) => {
                let size = self.entry_size(id);
                if size.is_empty() {
                    String::from("None")
                } else {
                    size
                }
            }
            None => String::new(),
        }
    }

    /// Modification date of the selected item.
    pub fn info_date(&self) -> String {
        match self.selected {
            Some(id) => self.entry_date(id),
            None => String::new(),
        }
    }

    /// One field of the info panel, by index.
    pub fn info_field(&self, index: u8) -> String {
        match index {
            0 => self.info_name(),
            1 => self.info_kind(),
            2 => self.info_size(),
            3 => self.info_date(),
            _ => self.info_place(),
        }
    }

    /// How tall the open sheet should be.
    pub fn sheet_height(&self) -> i32 {
        match self.dialog {
            Some(DialogKind::NewFolder) | Some(DialogKind::Rename) => 168,
            Some(DialogKind::GetInfo) => 216,
            Some(DialogKind::About) => 200,
            Some(DialogKind::Delete) => 132,
            _ => 0,
        }
    }

    /// Where the selected item lives.
    pub fn info_place(&self) -> String {
        match self.selected {
            Some(id) => String::from(data::place_name(self.place_of(id))),
            None => String::new(),
        }
    }

    // --- entries ----------------------------------------------------------

    /// The place an entry currently lives in.
    pub fn place_of(&self, id: u32) -> Place {
        if let Some((_, place)) = self.relocations.iter().find(|(entry, _)| *entry == id) {
            return *place;
        }
        if id >= CREATED_BASE
            && let Some(folder) = self.created.iter().find(|folder| folder.id == id)
        {
            return folder.place;
        }
        data::place_of(id as usize)
    }

    /// Display name of an entry, extension included.
    pub fn entry_title(&self, id: u32) -> String {
        if let Some((_, name)) = self.renames.iter().find(|(entry, _)| *entry == id) {
            return name.clone();
        }
        if id >= CREATED_BASE
            && let Some(folder) = self.created.iter().find(|folder| folder.id == id)
        {
            return folder.name.clone();
        }
        data::entry(id as usize).display_name()
    }

    /// Kind of an entry. An empty tile has no kind to show.
    pub fn entry_kind(&self, id: u32) -> Kind {
        if id == u32::MAX {
            return Kind::Folder;
        }
        if id >= CREATED_BASE {
            return Kind::Folder;
        }
        data::entry(id as usize).kind
    }

    /// Numeric kind of an entry.
    pub fn entry_kind_id(&self, id: u32) -> u8 {
        match self.entry_kind(id) {
            Kind::Folder => 0,
            Kind::Document => 1,
            Kind::Image => 2,
            Kind::Code => 3,
            Kind::Archive => 4,
            Kind::Audio => 5,
        }
    }

    /// Whether an entry has the given kind.
    pub fn entry_is_kind(&self, id: u32, kind: u8) -> bool {
        self.entry_kind_id(id) == kind
    }

    /// Size label of an entry.
    pub fn entry_size(&self, id: u32) -> String {
        if id >= CREATED_BASE {
            return String::new();
        }
        data::entry(id as usize).size_label()
    }

    /// Date label of an entry.
    pub fn entry_date(&self, id: u32) -> String {
        if id >= CREATED_BASE {
            return String::from("Today");
        }
        data::entry(id as usize).date_label()
    }

    /// The ids of the files in the current listing.
    pub fn visible_ids(&self) -> Vec<u32> {
        self.visible.clone()
    }

    /// Entry indices for the icon grid, capped at the grid limit.
    pub fn grid_indices(&self) -> Vec<u32> {
        let limit = ICON_GRID_MAX as usize;
        self.visible.iter().take(limit).copied().collect()
    }

    /// How many tiles the icon grid shows.
    pub fn grid_count(&self) -> u32 {
        let limit = if self.query.is_empty() {
            ICON_GRID_MAX
        } else {
            SEARCH_GRID_MAX
        };
        self.visible.len().min(limit as usize) as u32
    }

    /// How many rows of tiles the icon grid has.
    pub fn grid_rows(&self) -> u32 {
        let count = self.grid_count();
        count.div_ceil(GRID_COLUMNS)
    }

    /// The row indexes of the icon grid.
    pub fn grid_row_indexes(&self) -> Vec<u32> {
        (0..self.grid_rows()).collect()
    }

    /// How tall the icon grid is.
    pub fn grid_height(&self) -> i32 {
        let rows = self.grid_rows() as i32;
        if rows == 0 {
            0
        } else {
            rows * TILE_HEIGHT + (rows - 1) * 10
        }
    }

    /// Number of rows the virtualised list has to offer.
    pub fn list_count(&self) -> u32 {
        self.visible.len() as u32
    }

    /// Id of the file at a position in the filtered list.
    pub fn entry_id(&self, index: u32) -> u32 {
        self.visible.get(index as usize).copied().unwrap_or(0)
    }

    /// Display name of the file at a position in the filtered list.
    pub fn entry_label(&self, index: u32) -> String {
        self.entry_title(self.entry_id(index))
    }

    /// Kind of the file at a position in the filtered list.
    pub fn entry_kind_at(&self, index: u32) -> String {
        String::from(self.entry_kind(self.entry_id(index)).label())
    }

    /// Size of the file at a position in the filtered list.
    pub fn entry_size_at(&self, index: u32) -> String {
        self.entry_size(self.entry_id(index))
    }

    /// Modification date of the file at a position in the filtered list.
    pub fn entry_date_at(&self, index: u32) -> String {
        self.entry_date(self.entry_id(index))
    }

    /// The current sort order, for the status bar.
    pub fn sort_label(&self) -> String {
        String::from(match self.sort {
            SortMode::Name => "Sorted by Name",
            SortMode::Date => "Sorted by Date",
            SortMode::Size => "Sorted by Size",
            SortMode::Kind => "Sorted by Kind",
        })
    }

    /// "N of M items" summary for the status bar.
    pub fn count_label(&self) -> String {
        let mut label = String::new();
        label.push_str(&alloc::format!("{}", self.shown));
        label.push_str(" of ");
        label.push_str(&alloc::format!("{}", self.total));
        label.push_str(" items");
        label
    }

    /// Recount the filtered files after a state change.
    pub fn recompute(&mut self) {
        self.listing_revision += 1;
        self.visible = self.collect_visible();
        self.total = self.visible.len() as u32;
        self.shown = if self.view == ViewMode::Icons {
            self.total.min(ICON_GRID_MAX)
        } else {
            self.total
        };
        self.revision += 1;
    }

    /// The entry ids the icon grid shows.
    pub fn grid_entries(&self) -> Vec<u32> {
        self.visible
            .iter()
            .take(ICON_GRID_MAX as usize)
            .copied()
            .collect()
    }

    /// The entry at a position in the icon grid.
    pub fn grid_entry(&self, position: u32) -> u32 {
        self.visible
            .get(position as usize)
            .copied()
            .unwrap_or(u32::MAX)
    }

    /// A key that changes whenever the contents of a grid row change.
    pub fn grid_row_key(&self, row: u32) -> u64 {
        let mut key = 0u64;
        for column in 0..GRID_COLUMNS {
            let id = self.grid_entry(row * GRID_COLUMNS + column);
            key = key.wrapping_mul(31).wrapping_add(id as u64);
        }
        key
    }

    /// The entries of the current place, filtered and sorted.
    fn collect_visible(&self) -> Vec<u32> {
        let mut entries: Vec<u32> = Vec::new();

        for place in data::PLACES {
            let (start, end) = data::place_range(place);
            for index in start..end {
                let id = index as u32;
                if self.trashed.contains(&id) {
                    continue;
                }
                let moved = self
                    .relocations
                    .iter()
                    .find(|(entry, _)| *entry == id)
                    .map(|(_, place)| *place);
                let effective = moved.unwrap_or(place);
                if effective != self.place {
                    continue;
                }
                entries.push(id);
            }
        }

        for (id, place) in &self.relocations {
            if *place == self.place && !self.trashed.contains(id) {
                entries.push(*id);
            }
        }

        for folder in &self.created {
            if folder.place == self.place {
                entries.push(folder.id);
            }
        }

        if !self.query.is_empty() {
            entries.retain(|id| {
                let title = self.entry_title(*id);
                data::contains_ci(&title, &self.query)
            });
        }

        match self.sort {
            SortMode::Name => entries.sort_by(|a, b| {
                let folder_a = self.entry_kind(*a) == Kind::Folder;
                let folder_b = self.entry_kind(*b) == Kind::Folder;
                folder_b
                    .cmp(&folder_a)
                    .then_with(|| self.entry_title(*a).cmp(&self.entry_title(*b)))
            }),
            SortMode::Date => entries.sort_by(|a, b| {
                let folder_a = self.entry_kind(*a) == Kind::Folder;
                let folder_b = self.entry_kind(*b) == Kind::Folder;
                folder_b
                    .cmp(&folder_a)
                    .then_with(|| self.entry_date(*a).cmp(&self.entry_date(*b)))
            }),
            SortMode::Size => entries.sort_by(|a, b| {
                let folder_a = self.entry_kind(*a) == Kind::Folder;
                let folder_b = self.entry_kind(*b) == Kind::Folder;
                folder_b
                    .cmp(&folder_a)
                    .then_with(|| self.entry_size(*a).cmp(&self.entry_size(*b)))
            }),
            SortMode::Kind => entries.sort_by(|a, b| {
                let folder_a = self.entry_kind(*a) == Kind::Folder;
                let folder_b = self.entry_kind(*b) == Kind::Folder;
                folder_b
                    .cmp(&folder_a)
                    .then_with(|| self.entry_kind(*a).label().cmp(self.entry_kind(*b).label()))
            }),
        }

        entries
    }
}
