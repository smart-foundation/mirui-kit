use crate::commands::{COMMANDS, Command, from_key};
use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use mirui::prelude::*;
use mirui::ui::widgets::TextDocument;
use mirui_kit::behavior::{
    collection::SelectionMode,
    group::{GroupItem, GroupKind, Orientation},
    tree::TreeNode,
};
use mirui_kit::table::DataTable;
use mirui_kit::{Actions, Number, Toggle};
use mirui_kit::{
    controls::{Check, ChoiceGroup, Hierarchy},
    forms::NumericEntry,
};
fn example_choices() -> ChoiceGroup {
    ChoiceGroup::new(
        GroupKind::Single { required: true },
        Orientation::Horizontal,
        [
            (1, "Balanced", false),
            (2, "Detailed", false),
            (3, "Unavailable", true),
        ]
        .into_iter()
        .map(|(id, label, disabled)| GroupItem {
            id,
            label: label.into(),
            disabled,
        })
        .collect(),
    )
    .expect("unique specimen IDs")
}
fn example_tree() -> Hierarchy {
    Hierarchy::new(
        [
            (1, None, "Workspace", true),
            (2, Some(1), "Documents", true),
            (3, Some(2), "Readme.txt", false),
            (4, Some(1), "Images", true),
            (5, Some(4), "Preview.png", false),
            (6, None, "Archive", true),
        ]
        .into_iter()
        .map(|(id, parent, label, branch)| TreeNode {
            id,
            parent,
            label: label.into(),
            branch,
            disabled: false,
        })
        .collect(),
        SelectionMode::Multiple,
    )
    .expect("valid specimen tree")
}
use studio_core::{Error, Route, Session, Surface, Theme};

#[model]
pub struct Studio {
    pub demo_calendar: model!(owned mirui_kit::calendar::Calendar),
    pub demo_options: model!(owned mirui_kit::choices::ChoiceList),
    pub demo_table: model!(owned DataTable),
    pub demo_choices: model!(owned ChoiceGroup),
    pub demo_hierarchy: model!(owned Hierarchy),
    pub demo_check: model!(owned Check),
    pub demo_numeric: model!(owned NumericEntry),
    pub actions: model!(owned Actions),
    pub quality: model!(owned Number),
    pub metadata: model!(owned Toggle),
    pub preset: model!(owned TextDocument),
    pub query: model!(owned TextDocument),
    pub note: model!(owned TextDocument),
    pub name: model!(owned TextDocument),
    pub palette: model!(owned TextDocument),
    #[observe]
    pub route: Route,
    #[observe]
    pub surface: Option<Surface>,
    #[observe]
    pub revision: u64,
    #[observe]
    pub palette_index: usize,
    #[observe]
    pub listing_revision: u64,
    core: Session,
    documents: [u64; 5],
    theme_applied: Option<Theme>,
}
fn document(text: &str, capacity: usize) -> TextDocument {
    let mut document = TextDocument::new();
    document
        .set_capacity(capacity)
        .expect("valid document capacity");
    document.set_text(text).expect("initial text fits");
    document
}
impl Default for Studio {
    fn default() -> Self {
        let core = Session::default();
        Self {
            actions: Actions::default().into(),
            demo_choices: example_choices().into(),
            demo_calendar: {
                use mirui_kit::behavior::{forms::Date,calendar::{CalendarMode,WeekStart}};
                mirui_kit::calendar::Calendar::new(Date::new(2026,10,24).unwrap(),Date::new(2026,1,1).unwrap(),Date::new(2027,12,31).unwrap(),CalendarMode::Range,WeekStart::Monday).unwrap().into()
            },
            demo_options: {
                use mirui_kit::behavior::choices::OptionItem;
                mirui_kit::choices::ChoiceList::new(alloc::vec![OptionItem{id:1,label:"Documents".into(),disabled:false},OptionItem{id:2,label:"Unavailable volume".into(),disabled:true},OptionItem{id:3,label:"Images".into(),disabled:false}],SelectionMode::Multiple).unwrap().into()
            },
            demo_table: {
                use mirui_kit::behavior::table::{Cell, Column, TableRow};
                mirui_kit::table::DataTable::new(
                    alloc::vec![
                        Column {
                            id: 1,
                            label: "Name".into(),
                            width: 240,
                            minimum: 120,
                            maximum: 480,
                            sortable: true
                        },
                        Column {
                            id: 2,
                            label: "Size".into(),
                            width: 120,
                            minimum: 80,
                            maximum: 240,
                            sortable: true
                        }
                    ],
                    alloc::vec![
                        TableRow {
                            id: 1,
                            cells: alloc::vec![Cell::Text("Readme.txt".into()), Cell::Integer(100)],
                            disabled: false
                        },
                        TableRow {
                            id: 2,
                            cells: alloc::vec![Cell::Text("Notes.txt".into()), Cell::Integer(9)],
                            disabled: false
                        }
                    ],
                    SelectionMode::Multiple,
                )
                .expect("valid table fixture")
                .into()
            },
            demo_hierarchy: example_tree().into(),
            demo_check: Check::default().into(),
            demo_numeric: NumericEntry::new(4, 1, 64, 1).expect("valid bounds").into(),
            quality: Number {
                value: Fixed::from_int(80),
            }
            .into(),
            metadata: Toggle::default().into(),
            preset: document(&core.preset_name, 256).into(),
            query: document("", 256).into(),
            note: document(&core.note, 8192).into(),
            name: document("", 256).into(),
            palette: document("", 256).into(),
            route: core.route,
            surface: None,
            revision: 0,
            palette_index: 0,
            listing_revision: 0,
            core,
            documents: [u64::MAX; 5],
            theme_applied: None,
        }
    }
}
#[model]
impl Studio {
    pub fn initialize(&mut self) {
        self.metadata.set(true);
        self.actions.set_scale(100);
    }
    fn publish(&mut self) {
        self.route = self.core.route;
        self.surface = self.core.surface;
        self.actions.set_modal(self.surface.is_some());
        self.actions.set_scale(self.core.appearance.scale);
        self.revision = self.revision.wrapping_add(1);
    }
    fn sync_inputs(&mut self) {
        let versions = [
            self.preset.revision(),
            self.query.revision(),
            self.note.revision(),
            self.name.revision(),
            self.palette.revision(),
        ];
        let quality = self.quality.value().to_int().clamp(0, 100) as u16;
        let metadata = self.metadata.checked();
        let changed = versions != self.documents
            || quality != self.core.quality
            || metadata != self.core.include_metadata;
        if changed {
            if versions[0] != self.documents[0]
                || quality != self.core.quality
                || metadata != self.core.include_metadata
            {
                let name = self.preset.read(str::to_string);
                self.core.edit_preset(&name, quality, metadata);
            }
            if versions[1] != self.documents[1] {
                let query = self.query.read(str::to_string);
                self.core.set_query(&query);
                self.listing_revision += 1;
            }
            if versions[2] != self.documents[2] {
                self.core.note = self.note.read(str::to_string);
            }
            if versions[4] != self.documents[4] {
                self.palette_index = 0;
            }
            self.documents = versions;
            self.publish();
        }
    }
    pub fn sync(&mut self) {
        while self.demo_choices.take_action().is_some() {}
        self.sync_inputs();
        for (key, overlay) in self.actions.take() {
            // A surface transition invalidates the rest of a sampled background press.
            if overlay != self.core.surface.is_some() {
                continue;
            }
            self.execute(key);
        }
    }
    pub fn execute(&mut self, key: u16) {
        self.sync_inputs();
        self.core.error = None;
        let listing = (
            self.core.assets.len(),
            self.core.folder,
            self.core.revision,
            self.core.editing,
        );
        let result = self.run(key);
        if listing
            != (
                self.core.assets.len(),
                self.core.folder,
                self.core.revision,
                self.core.editing,
            )
        {
            self.listing_revision = self.listing_revision.wrapping_add(1);
        }
        if let Err(error) = result {
            self.core.error = Some(error);
        }
        self.sync_inputs();
        self.publish();
    }
    fn run(&mut self, key: u16) -> Result<(), Error> {
        if key == 90 {
            if self.core.surface != Some(Surface::Palette) {
                return Err(Error::Blocked);
            }
            let rows = self.palette_rows();
            let index = *rows.get(self.palette_index).ok_or(Error::Stale)?;
            return self.run(COMMANDS[index].command as u16);
        }
        if (1000..2000).contains(&key) {
            if self.core.surface.is_some() {
                return Err(Error::Blocked);
            }
            let editing = self.core.editing;
            self.core.select(u32::from(key - 1000))?;
            if editing != self.core.editing {
                self.listing_revision += 1;
            }
            return Ok(());
        }
        if (2000..3000).contains(&key) {
            if self.core.surface != Some(Surface::Move) {
                return Err(Error::Blocked);
            }
            self.core.move_selected(u32::from(key - 2000))?;
            self.listing_revision += 1;
            return Ok(());
        }
        if (3000..3017).contains(&key) {
            let surface = Surface::ALL[usize::from(key - 3000)];
            if matches!(surface, Surface::Palette | Surface::Catalog) {
                self.palette.set_text("").map_err(|_| Error::InvalidName)?;
            }
            return self.core.open(surface);
        }
        let Some(command) = from_key(key) else {
            return Err(Error::Stale);
        };
        if self.core.surface == Some(Surface::Palette) && command != Command::Close {
            if !self.available(command) {
                return Err(Error::Blocked);
            }
            self.core.dismiss();
        }
        use Command::*;
        match command {
            Overview | Controls | Collections | Workbench | Surfaces | Foundations => {
                if matches!(
                    self.core.surface,
                    Some(Surface::Navigation | Surface::Catalog)
                ) {
                    self.core.dismiss();
                }
                self.core
                    .navigate(Route::ALL[usize::from(command as u16 - 1)])?;
            }
            Palette => {
                self.palette.set_text("").map_err(|_| Error::InvalidName)?;
                self.core.open(Surface::Palette)?;
            }
            Inspector => self.core.open(Surface::Inspector)?,
            Options => self.core.open(Surface::Options)?,
            Help => self.core.open(Surface::Help)?,
            Diagnostics => self.core.open(Surface::Diagnostics)?,
            Catalog => {
                self.palette.set_text("").map_err(|_| Error::InvalidName)?;
                self.core.open(Surface::Catalog)?;
            }
            Reset => self.core.open(Surface::Reset)?,
            Notices => self.core.open(Surface::Notices)?,
            NewFolder => {
                self.require_background(Route::Collections)?;
                // Clear/synchronize the filter before creating the draft.
                // set_query intentionally cancels edits; replaying this document
                // revision after begin_new would immediately delete the draft.
                self.query.set_text("").map_err(|_| Error::InvalidName)?;
                self.sync_inputs();
                self.core.begin_new()?;
                self.name.set_text("").map_err(|_| Error::InvalidName)?;
                self.listing_revision += 1;
            }
            Rename => {
                if self.core.surface == Some(Surface::Context) {
                    self.core.dismiss();
                }
                self.require_background(Route::Collections)?;
                let text = self.core.begin_rename()?;
                self.name.set_text(&text).map_err(|_| Error::InvalidName)?;
                self.listing_revision += 1;
            }
            Preview => {
                self.require_background(Route::Controls)?;
                self.core.start_preview()?;
            }
            SavePreset => {
                self.require_background(Route::Controls)?;
                if let Err(error) = self.core.save_preset() {
                    if error == Error::DuplicateName {
                        self.core.open(Surface::Conflict)?;
                    }
                    return Err(error);
                }
            }
            SaveNote => {
                self.require_background(Route::Workbench)?;
                self.core.saved_note = self.core.note.clone();
                self.core.notify("Note saved to this session, not to disk.");
            }
            Review => self.core.open(Surface::Confirm)?,
            ApplyAppearance => {
                self.require_background(Route::Foundations)?;
                self.core.apply_appearance()?;
            }
            Dark | Light => {
                self.require_background(Route::Foundations)?;
                self.core.draft.theme = if command == Dark {
                    Theme::Dark
                } else {
                    Theme::Light
                };
            }
            Smaller | Larger => {
                self.require_background(Route::Foundations)?;
                self.core.draft.scale = if command == Smaller {
                    self.core.draft.scale.saturating_sub(10).max(100)
                } else {
                    (self.core.draft.scale + 10).min(150)
                };
            }
            Up => {
                self.require_background(Route::Collections)?;
                self.core.up();
                self.query.set_text("").map_err(|_| Error::InvalidName)?;
                self.listing_revision += 1;
            }
            OpenAsset => {
                if matches!(
                    self.core.surface,
                    Some(Surface::Context | Surface::Inspector)
                ) {
                    self.core.dismiss();
                }
                self.require_background(Route::Collections)?;
                let id = self.core.selected.ok_or(Error::NoSelection)?;
                if self.core.asset(id).ok_or(Error::MissingAsset)?.folder {
                    self.core.enter(id)?;
                    self.query.set_text("").map_err(|_| Error::InvalidName)?;
                    self.listing_revision += 1;
                } else {
                    self.core.open(Surface::AssetPreview)?;
                }
            }
            Move => {
                if self.core.surface == Some(Surface::Context) {
                    self.core.dismiss();
                }
                self.core.open(Surface::Move)?;
            }
            CommitName => {
                self.require_background(Route::Collections)?;
                let name = self.name.read(str::to_string);
                self.core.commit_name(&name)?;
                self.listing_revision += 1;
            }
            CancelName => {
                self.core.cancel_edit();
                self.listing_revision += 1;
            }
            Close => self.core.dismiss(),
            Keep => self.core.keep_appearance(),
            Revert => self.core.revert_appearance(),
            SaveLeave | DiscardLeave => {
                self.core.finish_navigation(command == SaveLeave)?;
                self.note
                    .set_text(&self.core.note)
                    .map_err(|_| Error::InvalidName)?;
            }
            ApplyReview => self.core.apply_review()?,
            ConfirmReset => {
                if self.core.surface != Some(Surface::Reset) {
                    return Err(Error::Blocked);
                }
                self.core = Session::default();
                self.preset
                    .set_text(&self.core.preset_name)
                    .map_err(|_| Error::InvalidName)?;
                self.note
                    .set_text(&self.core.note)
                    .map_err(|_| Error::InvalidName)?;
                self.query.set_text("").map_err(|_| Error::InvalidName)?;
                self.quality.set(Fixed::from_int(80));
                self.metadata.set(true);
                self.listing_revision += 1;
            }
            ToggleSort => {
                if self.core.surface != Some(Surface::Options) {
                    return Err(Error::Blocked);
                }
                self.core.descending = !self.core.descending;
                self.listing_revision += 1;
            }
            AppendParagraph => {
                self.require_background(Route::Workbench)?;
                let text = format!("{}\n\n", self.core.note);
                self.note.set_text(&text).map_err(|_| Error::InvalidName)?;
            }
            Context => self.core.open(Surface::Context)?,
            Tooltip => self.core.open(Surface::Tooltip)?,
        }
        Ok(())
    }
    fn require_background(&self, route: Route) -> Result<(), Error> {
        if self.core.surface.is_none() && self.core.route == route {
            Ok(())
        } else {
            Err(Error::Blocked)
        }
    }
    pub fn available(&self, command: Command) -> bool {
        use Command::*;
        match command {
            Overview | Controls | Collections | Workbench | Surfaces | Foundations | Help
            | Diagnostics | Catalog | Reset | Notices => true,
            Inspector | Context | OpenAsset | Move | Rename => {
                self.core.route == Route::Collections && self.core.selected.is_some()
            }
            NewFolder | Options | Up => self.core.route == Route::Collections,
            Preview | SavePreset => self.core.route == Route::Controls,
            SaveNote | AppendParagraph => self.core.route == Route::Workbench,
            Review => self.core.route == Route::Surfaces,
            ApplyAppearance | Dark | Light | Smaller | Larger => {
                self.core.route == Route::Foundations
            }
            _ => false,
        }
    }
    pub fn move_palette(&mut self, delta: i32) {
        let len = self.palette_rows().len();
        if len > 0 {
            self.palette_index =
                (self.palette_index as i32 + delta).rem_euclid(len as i32) as usize;
        }
    }
    pub fn catalog_rows(&self) -> Vec<usize> {
        let query = self.palette.read(|s| s.trim().to_lowercase());
        crate::catalog::COMPONENTS
            .iter()
            .enumerate()
            .filter(|(_, c)| {
                c.title.to_lowercase().contains(&query) || c.api.to_lowercase().contains(&query)
            })
            .map(|(i, _)| i)
            .collect()
    }
    pub fn palette_rows(&self) -> Vec<usize> {
        let query = self.palette.read(|s| s.trim().to_lowercase());
        COMMANDS
            .iter()
            .enumerate()
            .filter(|(_, c)| self.available(c.command) && c.label.to_lowercase().contains(&query))
            .map(|(i, _)| i)
            .collect()
    }
    pub fn tick(&mut self) {
        if self.core.tick() {
            self.publish();
        }
    }
    pub fn wanted_theme(&self) -> Theme {
        self.core.appearance.theme
    }
    pub fn theme_applied(&self) -> Option<Theme> {
        self.theme_applied
    }
    pub fn set_theme_applied(&mut self, theme: Theme) {
        self.theme_applied = Some(theme);
    }
    pub fn rows(&self) -> Vec<u32> {
        self.core.visible_assets()
    }
    pub fn asset_label(&self, id: u32) -> String {
        self.core
            .asset(id)
            .map_or_else(|| "Missing asset".into(), |a| a.name.clone())
    }
    pub fn is_folder(&self, id: u32) -> bool {
        self.core.asset(id).is_some_and(|a| a.folder)
    }
    pub fn editing(&self) -> Option<u32> {
        self.core.editing
    }
    pub fn selected(&self) -> Option<u32> {
        self.core.selected
    }
    pub fn folder_label(&self) -> String {
        self.core
            .asset(self.core.folder)
            .map_or_else(|| "Sample assets".into(), |a| a.name.clone())
    }
    pub fn folders(&self) -> Vec<u32> {
        core::iter::once(0)
            .chain(self.core.assets.iter().filter(|a| a.folder).map(|a| a.id))
            .collect()
    }
    pub fn error(&self) -> String {
        self.core
            .error
            .as_ref()
            .map_or_else(String::new, |e| e.message().into())
    }
    pub fn latest_notice(&self) -> String {
        self.core.notices.last().cloned().unwrap_or_else(|| {
            "Session-only fixtures · no filesystem, encoder, or network access".into()
        })
    }
    pub fn preview(&self) -> String {
        if let Some(job) = self.core.job {
            format!("Calculating fixture preview… {}%", (4 - job.remaining) * 25)
        } else {
            self.core
                .preview
                .clone()
                .unwrap_or_else(|| "Edit the preset, then explicitly calculate a preview.".into())
        }
    }
    pub fn note_status(&self) -> String {
        if self.core.dirty() {
            "Unsaved changes · this session only".into()
        } else {
            "Saved in session memory · not on disk".into()
        }
    }
    pub fn appearance_caption(&self) -> String {
        format!(
            "Draft: {} · {}% text. Global: {} · {}%.",
            self.core.draft.theme.name(),
            self.core.draft.scale,
            self.core.appearance.theme.name(),
            self.core.appearance.scale
        )
    }
    pub fn draft_light(&self) -> bool {
        self.core.draft.theme == Theme::Light
    }
    pub fn draft_size(&self) -> u16 {
        14 * self.core.draft.scale / 100
    }
    pub fn selected_text(&self) -> String {
        self.core
            .selected
            .and_then(|id| self.core.asset(id))
            .map_or_else(
                || "Select a sample asset.".into(),
                |a| {
                    format!(
                        "{}\n\n{}",
                        a.name,
                        if a.folder {
                            "Folder in the session fixture."
                        } else {
                            a.text
                        }
                    )
                },
            )
    }
    pub fn surface_body(&self) -> String {
        match self.core.surface {
            Some(Surface::Inspector|Surface::AssetPreview|Surface::Context) => self.selected_text(),
            Some(Surface::Confirm) => format!("Apply one sample review transaction? Already applied: {}. Cancellation changes nothing.",self.core.applied_changes),
            Some(Surface::Conflict) => "No preset was overwritten. Close this sheet and choose another preset name.".into(),
            Some(Surface::Notices) => if self.core.notices.is_empty() { "No notices yet. Saved operations will appear here.".into() } else { self.core.notices.join("\n\n") },
            Some(Surface::Diagnostics) => "Mirui 0.51.0 · no_std + alloc core\nNative: text documents, slider, switch, gestures\nRepository host targets: SDL desktop / RGBA snapshots (not active-host detection)\nFixtures: session assets, presets, plain-text note\nUnavailable: disk persistence, encoder, OS file access\nBuild and visual verification: see docs/showcase/IMPLEMENTATION.md".into(),
            Some(Surface::Tooltip) => "Preview calculates an in-memory fixture summary. Editing any export setting cancels the previous calculation; click Calculate preview again.".into(),
            Some(Surface::Unsaved) => "Your note has unsaved session edits. Save or discard before leaving; Close keeps you here with the note intact.".into(),
            Some(Surface::Reset) => "This clears sample assets, presets, notices, the session note, and appearance. Nothing on disk is affected. Close cancels.".into(),
            Some(Surface::Help) => "Ctrl P: commands · Ctrl S: save note\nTab / Shift Tab: move focus within the active layer\nEnter / Space: activate a button · Escape: close / revert\nPlain-text fields use native Mirui selection and clipboard.\nAppend a paragraph adds a line break to the session note.\nNaledzi Studio · MIT · Inter / Solar assets retain their licenses.".into(),
            Some(Surface::Recovery) => format!("Keep this global appearance? Automatically reverts in {} seconds.",self.core.recovery.map_or(0,|r|r.remaining.div_ceil(4))),
            Some(Surface::Options) => format!("Current order: {} name. Folders remain first. Selection uses stable asset identity.",if self.core.descending { "descending" } else { "ascending" }),
            Some(Surface::Move) => "Choose a sample folder. Cycles and name conflicts are rejected before mutation.".into(),
            Some(Surface::Catalog) => "Shared kit: action, field, number, toggle, input gate, keyboard focus order, Inter typography, semantic theme, Solar icons. Every specimen uses these same constructors. Recipes: crates/kit/src/widgets.rs.".into(),
            _ => String::new(),
        }
    }
}
