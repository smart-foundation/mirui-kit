//! Platform-independent, deterministic showcase workflows. All data is session-only.
#![no_std]
extern crate alloc;
use alloc::{
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Route {
    Overview,
    Controls,
    Collections,
    Workbench,
    Surfaces,
    Foundations,
}
impl Route {
    pub const ALL: [Self; 6] = [
        Self::Overview,
        Self::Controls,
        Self::Collections,
        Self::Workbench,
        Self::Surfaces,
        Self::Foundations,
    ];
    pub const fn title(self) -> &'static str {
        match self {
            Self::Overview => "Overview",
            Self::Controls => "Controls",
            Self::Collections => "Collections",
            Self::Workbench => "Workbench",
            Self::Surfaces => "Surfaces",
            Self::Foundations => "Foundations",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Surface {
    Palette,
    Inspector,
    Context,
    Options,
    Confirm,
    Conflict,
    Notices,
    Diagnostics,
    Tooltip,
    Unsaved,
    Navigation,
    Reset,
    Catalog,
    Help,
    Recovery,
    AssetPreview,
    Move,
}
impl Surface {
    pub const ALL: [Self; 17] = [
        Self::Palette,
        Self::Inspector,
        Self::Context,
        Self::Options,
        Self::Confirm,
        Self::Conflict,
        Self::Notices,
        Self::Diagnostics,
        Self::Tooltip,
        Self::Unsaved,
        Self::Navigation,
        Self::Reset,
        Self::Catalog,
        Self::Help,
        Self::Recovery,
        Self::AssetPreview,
        Self::Move,
    ];
    pub const fn title(self) -> &'static str {
        match self {
            Self::Palette => "Commands",
            Self::Inspector => "Selection inspector",
            Self::Context => "Asset actions",
            Self::Options => "Collection options",
            Self::Confirm => "Review changes",
            Self::Conflict => "Name already exists",
            Self::Notices => "Notice history",
            Self::Diagnostics => "Runtime capabilities",
            Self::Tooltip => "Control help",
            Self::Unsaved => "Unsaved session note",
            Self::Navigation => "Navigate Studio",
            Self::Reset => "Reset sample session?",
            Self::Catalog => "Component catalog",
            Self::Help => "Help & keyboard",
            Self::Recovery => "Keep this appearance?",
            Self::AssetPreview => "Sample asset preview",
            Self::Move => "Move sample asset",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Theme {
    Dark,
    Light,
}
impl Theme {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Dark => "dark",
            Self::Light => "light",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Appearance {
    pub theme: Theme,
    pub scale: u16,
}
impl Default for Appearance {
    fn default() -> Self {
        Self {
            theme: Theme::Dark,
            scale: 100,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    EmptyName,
    InvalidName,
    DuplicateName,
    MissingAsset,
    NotFolder,
    Cycle,
    Stale,
    InvalidScale,
    Blocked,
    NoSelection,
    Capacity,
}
impl Error {
    pub const fn message(&self) -> &'static str {
        match self {
            Self::EmptyName => "Enter a name.",
            Self::InvalidName => "Use 1–64 characters, without slash or control characters.",
            Self::DuplicateName => "That name is already used here. Choose another.",
            Self::MissingAsset => "This sample asset no longer exists.",
            Self::NotFolder => "Choose a folder as the destination.",
            Self::Cycle => "A folder cannot be moved inside itself.",
            Self::Stale => "The data changed. Review the current state before trying again.",
            Self::InvalidScale => "Choose a text scale between 100% and 150%.",
            Self::Blocked => "Finish or dismiss the current surface first.",
            Self::NoSelection => "Select an asset first.",
            Self::Capacity => "The session fixture is full. Reset it before adding more.",
        }
    }
}
fn validate_name(name: &str) -> Result<&str, Error> {
    let name = name.trim();
    if name.is_empty() {
        return Err(Error::EmptyName);
    }
    if name.chars().count() > 64
        || name
            .chars()
            .any(|c| c.is_control() || c == '/' || c == '\\')
        || name == "."
        || name == ".."
    {
        return Err(Error::InvalidName);
    }
    Ok(name)
}
#[derive(Clone, Debug)]
pub struct Asset {
    pub id: u32,
    pub parent: u32,
    pub name: String,
    pub folder: bool,
    pub text: &'static str,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Job {
    pub version: u64,
    pub remaining: u8,
}
#[derive(Clone, Copy, Debug)]
pub struct Recovery {
    pub previous: Appearance,
    pub remaining: u8,
}
#[derive(Clone, Debug)]
pub struct Session {
    pub route: Route,
    pub surface: Option<Surface>,
    pub selected: Option<u32>,
    pub folder: u32,
    pub assets: Vec<Asset>,
    pub query: String,
    pub descending: bool,
    pub note: String,
    pub saved_note: String,
    pub pending_route: Option<Route>,
    pub preset_name: String,
    pub quality: u16,
    pub include_metadata: bool,
    pub preset_version: u64,
    pub job: Option<Job>,
    pub preview: Option<String>,
    pub presets: Vec<String>,
    pub appearance: Appearance,
    pub draft: Appearance,
    pub recovery: Option<Recovery>,
    pub notices: Vec<String>,
    pub error: Option<Error>,
    pub revision: u64,
    pub editing: Option<u32>,
    pub new_draft: Option<u32>,
    pub applied_changes: u32,
    next_id: u32,
    reviewed_revision: Option<u64>,
}
impl Default for Session {
    fn default() -> Self {
        let note = "A small, useful workspace.\n\nEdit this note, then save it to this session. No disk access is claimed.".to_string();
        Self {
            route: Route::Overview,
            surface: None,
            selected: None,
            folder: 0,
            query: String::new(),
            descending: false,
            assets: vec![
                Asset {
                    id: 1,
                    parent: 0,
                    name: "Brand".into(),
                    folder: true,
                    text: "",
                },
                Asset {
                    id: 2,
                    parent: 0,
                    name: "Notes".into(),
                    folder: true,
                    text: "",
                },
                Asset {
                    id: 3,
                    parent: 0,
                    name: "Welcome.txt".into(),
                    folder: false,
                    text: "Welcome to Naledzi Studio. These assets live only in memory.",
                },
                Asset {
                    id: 4,
                    parent: 1,
                    name: "Palette.txt".into(),
                    folder: false,
                    text: "Ink, paper, and a clear blue accent. Semantic tokens, not per-screen colors.",
                },
                Asset {
                    id: 5,
                    parent: 2,
                    name: "Ideas.txt".into(),
                    folder: false,
                    text: "Build the real task with the same controls as the specimens.",
                },
            ],
            note: note.clone(),
            saved_note: note,
            pending_route: None,
            preset_name: "Web preview".into(),
            quality: 80,
            include_metadata: true,
            preset_version: 0,
            job: None,
            preview: None,
            presets: Vec::new(),
            appearance: Appearance::default(),
            draft: Appearance::default(),
            recovery: None,
            notices: Vec::new(),
            error: None,
            revision: 0,
            editing: None,
            new_draft: None,
            applied_changes: 0,
            next_id: 6,
            reviewed_revision: None,
        }
    }
}
impl Session {
    pub fn dirty(&self) -> bool {
        self.note != self.saved_note
    }
    pub fn asset(&self, id: u32) -> Option<&Asset> {
        self.assets.iter().find(|a| a.id == id)
    }
    pub fn set_query(&mut self, query: &str) {
        self.cancel_edit();
        self.query = query.into();
        if self
            .selected
            .is_some_and(|id| !self.visible_assets().contains(&id))
        {
            self.selected = None;
        }
    }
    pub fn visible_assets(&self) -> Vec<u32> {
        let query = self.query.to_lowercase();
        let mut rows: Vec<&Asset> = self
            .assets
            .iter()
            .filter(|a| a.parent == self.folder && a.name.to_lowercase().contains(&query))
            .collect();
        rows.sort_by(|a, b| {
            b.folder
                .cmp(&a.folder)
                .then_with(|| {
                    if self.descending {
                        b.name.to_lowercase().cmp(&a.name.to_lowercase())
                    } else {
                        a.name.to_lowercase().cmp(&b.name.to_lowercase())
                    }
                })
                .then(a.id.cmp(&b.id))
        });
        rows.iter().map(|a| a.id).collect()
    }
    pub fn notify(&mut self, message: impl Into<String>) {
        if self.notices.len() == 3 {
            self.notices.remove(0);
        }
        self.notices.push(message.into());
    }
    pub fn open(&mut self, surface: Surface) -> Result<(), Error> {
        if self.surface.is_some() {
            return Err(Error::Blocked);
        }
        if surface == Surface::Recovery {
            return Err(Error::Blocked);
        }
        if matches!(
            surface,
            Surface::Inspector | Surface::Context | Surface::AssetPreview | Surface::Move
        ) && self.selected.is_none()
        {
            return Err(Error::NoSelection);
        }
        if surface == Surface::Confirm {
            self.reviewed_revision = Some(self.revision);
        }
        self.surface = Some(surface);
        self.error = None;
        Ok(())
    }
    pub fn dismiss(&mut self) {
        if self.surface == Some(Surface::Recovery) {
            self.revert_appearance();
            return;
        }
        if self.surface == Some(Surface::Unsaved) {
            self.pending_route = None;
        }
        self.reviewed_revision = None;
        self.surface = None;
        self.error = None;
    }
    pub fn navigate(&mut self, route: Route) -> Result<(), Error> {
        if self.surface.is_some() {
            return Err(Error::Blocked);
        }
        if route == self.route {
            return Ok(());
        }
        if self.dirty() && self.route == Route::Workbench {
            self.pending_route = Some(route);
            self.surface = Some(Surface::Unsaved);
            return Ok(());
        }
        self.cancel_edit();
        self.job = None;
        self.route = route;
        self.error = None;
        Ok(())
    }
    pub fn finish_navigation(&mut self, save: bool) -> Result<(), Error> {
        if self.surface != Some(Surface::Unsaved) {
            return Err(Error::Stale);
        }
        let route = self.pending_route.take().ok_or(Error::Stale)?;
        if save {
            self.saved_note = self.note.clone();
        } else {
            self.note = self.saved_note.clone();
        }
        self.surface = None;
        self.route = route;
        Ok(())
    }
    pub fn edit_preset(&mut self, name: &str, quality: u16, metadata: bool) {
        let quality = quality.min(100);
        if self.preset_name == name && self.quality == quality && self.include_metadata == metadata
        {
            return;
        }
        self.preset_name = name.into();
        self.quality = quality.min(100);
        self.include_metadata = metadata;
        self.preset_version = self.preset_version.wrapping_add(1);
        self.job = None;
        self.preview = None;
    }
    pub fn start_preview(&mut self) -> Result<u64, Error> {
        validate_name(&self.preset_name)?;
        self.job = Some(Job {
            version: self.preset_version,
            remaining: 4,
        });
        self.preview = None;
        Ok(self.preset_version)
    }
    pub fn complete_preview(&mut self, version: u64) -> Result<(), Error> {
        if self.job.is_none_or(|j| j.version != version) || self.preset_version != version {
            return Err(Error::Stale);
        }
        self.preview = Some(format!(
            "{} · quality {}% · metadata {}. Preview calculation only; no encoder or file output.",
            self.preset_name.trim(),
            self.quality,
            if self.include_metadata {
                "included"
            } else {
                "omitted"
            }
        ));
        self.job = None;
        Ok(())
    }
    pub fn save_preset(&mut self) -> Result<(), Error> {
        let name = validate_name(&self.preset_name)?;
        if self.presets.iter().any(|p| p.eq_ignore_ascii_case(name)) {
            return Err(Error::DuplicateName);
        }
        if self.presets.len() >= 32 {
            return Err(Error::Capacity);
        }
        self.presets.push(name.into());
        self.notify("Preset saved to this session.");
        Ok(())
    }
    pub fn select(&mut self, id: u32) -> Result<(), Error> {
        if self.asset(id).is_none_or(|a| a.parent != self.folder) {
            return Err(Error::MissingAsset);
        }
        if self.editing != Some(id) {
            self.cancel_edit();
        }
        self.selected = Some(id);
        Ok(())
    }
    pub fn enter(&mut self, id: u32) -> Result<(), Error> {
        if !self.asset(id).ok_or(Error::MissingAsset)?.folder {
            return Err(Error::NotFolder);
        }
        self.cancel_edit();
        self.folder = id;
        self.selected = None;
        self.query.clear();
        Ok(())
    }
    pub fn up(&mut self) {
        self.cancel_edit();
        self.folder = self.asset(self.folder).map_or(0, |a| a.parent);
        self.selected = None;
        self.query.clear();
    }
    pub fn begin_new(&mut self) -> Result<u32, Error> {
        if self.assets.len() >= 128 || self.next_id >= 1000 {
            return Err(Error::Capacity);
        }
        self.cancel_edit();
        let id = self.next_id;
        self.next_id += 1;
        self.assets.push(Asset {
            id,
            parent: self.folder,
            name: String::new(),
            folder: true,
            text: "",
        });
        self.new_draft = Some(id);
        self.editing = Some(id);
        self.selected = Some(id);
        Ok(id)
    }
    pub fn begin_rename(&mut self) -> Result<String, Error> {
        let id = self.selected.ok_or(Error::NoSelection)?;
        let name = self.asset(id).ok_or(Error::MissingAsset)?.name.clone();
        self.editing = Some(id);
        Ok(name)
    }
    pub fn commit_name(&mut self, name: &str) -> Result<(), Error> {
        let name = validate_name(name)?;
        let id = self.editing.ok_or(Error::Stale)?;
        let parent = self.asset(id).ok_or(Error::MissingAsset)?.parent;
        if self
            .assets
            .iter()
            .any(|a| a.id != id && a.parent == parent && a.name.eq_ignore_ascii_case(name))
        {
            return Err(Error::DuplicateName);
        }
        self.assets
            .iter_mut()
            .find(|a| a.id == id)
            .ok_or(Error::MissingAsset)?
            .name = name.into();
        self.editing = None;
        self.new_draft = None;
        self.revision += 1;
        Ok(())
    }
    pub fn cancel_edit(&mut self) {
        if let Some(id) = self.new_draft.take() {
            self.assets.retain(|a| a.id != id);
            if self.selected == Some(id) {
                self.selected = None;
            }
        }
        self.editing = None;
    }
    pub fn move_selected(&mut self, destination: u32) -> Result<(), Error> {
        if self.editing.is_some() {
            return Err(Error::Blocked);
        }
        let id = self.selected.ok_or(Error::NoSelection)?;
        let asset = self.asset(id).ok_or(Error::MissingAsset)?;
        if destination != 0 && !self.asset(destination).ok_or(Error::MissingAsset)?.folder {
            return Err(Error::NotFolder);
        }
        let mut parent = destination;
        for _ in 0..=self.assets.len() {
            if parent == id {
                return Err(Error::Cycle);
            }
            if parent == 0 {
                break;
            }
            parent = self.asset(parent).ok_or(Error::MissingAsset)?.parent;
        }
        if self.assets.iter().any(|a| {
            a.id != id && a.parent == destination && a.name.eq_ignore_ascii_case(&asset.name)
        }) {
            return Err(Error::DuplicateName);
        }
        self.assets
            .iter_mut()
            .find(|a| a.id == id)
            .ok_or(Error::MissingAsset)?
            .parent = destination;
        self.selected = None;
        self.surface = None;
        self.revision += 1;
        self.notify("Sample asset moved.");
        Ok(())
    }
    pub fn apply_review(&mut self) -> Result<(), Error> {
        if self.surface != Some(Surface::Confirm) || self.reviewed_revision != Some(self.revision) {
            return Err(Error::Stale);
        }
        self.applied_changes += 1;
        self.revision += 1;
        self.dismiss();
        self.notify("Sample change applied once.");
        Ok(())
    }
    pub fn apply_appearance(&mut self) -> Result<(), Error> {
        if !(100..=150).contains(&self.draft.scale) {
            return Err(Error::InvalidScale);
        }
        if self.surface.is_some() {
            return Err(Error::Blocked);
        }
        self.recovery = Some(Recovery {
            previous: self.appearance,
            remaining: 40,
        });
        self.appearance = self.draft;
        self.surface = Some(Surface::Recovery);
        Ok(())
    }
    pub fn keep_appearance(&mut self) {
        self.recovery = None;
        if self.surface == Some(Surface::Recovery) {
            self.surface = None;
        }
    }
    pub fn revert_appearance(&mut self) {
        if let Some(recovery) = self.recovery.take() {
            self.appearance = recovery.previous;
            self.draft = recovery.previous;
        }
        if self.surface == Some(Surface::Recovery) {
            self.surface = None;
        }
    }
    /// One 250 ms timer tick; false means no observable work, so the UI stays idle.
    pub fn tick(&mut self) -> bool {
        let mut changed = false;
        if let Some(job) = self.job.as_mut() {
            job.remaining = job.remaining.saturating_sub(1);
            let version = job.version;
            let done = job.remaining == 0;
            changed = true;
            if done {
                let _ = self.complete_preview(version);
            }
        }
        if let Some(recovery) = self.recovery.as_mut() {
            recovery.remaining = recovery.remaining.saturating_sub(1);
            let expired = recovery.remaining == 0;
            changed = true;
            if expired {
                self.revert_appearance();
            }
        }
        changed
    }
}

#[cfg(test)]
mod tests;
