//! Application state.
//!
//! One registered model instance owns everything the UI reacts to. Selection,
//! search text, the current place and the view mode are *observed*, so the
//! reactive bindings in the widget tree re-evaluate when they change.

use alloc::string::String;
use alloc::vec::Vec;

use mirui::prelude::*;
use mirui::ui::widgets::TextDocument;

use crate::data;

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

/// Upper bound on files drawn in the icon grid.
pub const ICON_GRID_MAX: u32 = 60;

#[model]
pub struct FmState {
    pub document: model!(owned TextDocument),
    query: String,
    history: Vec<Place>,
    visible: Vec<u32>,
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
}

impl FmState {
    /// A fresh state: Documents, icon view, dark theme.
    pub fn new() -> Self {
        Self {
            document: TextDocument::new().into(),
            query: String::new(),
            history: Vec::new(),
            visible: Vec::new(),
            place: Place::Documents,
            selected: None,
            view: ViewMode::Icons,
            dark: true,
            shown: 0,
            total: 0,
            revision: 0,
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

    /// Navigate to a place, remembering the previous one.
    pub fn set_place(&mut self, place: Place) {
        if self.place == place {
            return;
        }
        self.history.push(self.place);
        self.place = place;
        self.selected = None;
        self.recompute();
    }

    /// Return to the previous place.
    pub fn go_back(&mut self) {
        if let Some(previous) = self.history.pop() {
            self.place = previous;
            self.selected = None;
            self.recompute();
        }
    }

    /// Select a file by id.
    pub fn select(&mut self, id: u32) {
        self.selected = Some(id);
        self.revision += 1;
    }

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

    /// Indices of the files that pass the current filter, in display order.
    pub fn visible(&self) -> Vec<usize> {
        data::visible(self.place, &self.query)
    }

    /// Entry indices for the icon grid, capped at the grid limit.
    pub fn grid_indices(&self) -> Vec<u32> {
        let limit = ICON_GRID_MAX as usize;
        self.visible.iter().take(limit).copied().collect()
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
        data::entry(self.entry_id(index) as usize).display_name()
    }

    /// Numeric kind of the file at a position in the filtered list.
    pub fn entry_kind_id(&self, index: u32) -> u8 {
        match data::entry(self.entry_id(index) as usize).kind {
            data::Kind::Folder => 0,
            data::Kind::Document => 1,
            data::Kind::Image => 2,
            data::Kind::Code => 3,
            data::Kind::Archive => 4,
            data::Kind::Audio => 5,
        }
    }

    /// Whether the file at a position has the given kind.
    pub fn entry_is_kind(&self, index: u32, kind: u8) -> bool {
        self.entry_kind_id(index) == kind
    }

    /// Kind of the file at a position in the filtered list.
    pub fn entry_kind(&self, index: u32) -> String {
        String::from(data::entry(self.entry_id(index) as usize).kind.label())
    }

    /// Size of the file at a position in the filtered list.
    pub fn entry_size(&self, index: u32) -> String {
        data::entry(self.entry_id(index) as usize).size_label()
    }

    /// Modification date of the file at a position in the filtered list.
    pub fn entry_date(&self, index: u32) -> String {
        data::entry(self.entry_id(index) as usize).date_label()
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
        self.visible = data::visible(self.place, &self.query)
            .into_iter()
            .map(|index| index as u32)
            .collect();
        self.total = self.visible.len() as u32;
        self.shown = if self.view == ViewMode::Icons {
            self.total.min(ICON_GRID_MAX)
        } else {
            self.total
        };
        self.revision += 1;
    }
}
