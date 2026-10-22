//! Static fake filesystem.
//!
//! Everything here is `const` or `static`: the browser never touches a disk, never
//! allocates to build the table, and never blocks. The only allocation in the data
//! path is the filtered index list produced by [`visible`].

use alloc::string::String;
use alloc::vec::Vec;

use crate::icons;
use crate::model::Place;

/// Total number of synthetic entries.
pub const ENTRY_COUNT: usize = 2048;

/// What an entry pretends to be.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Folder,
    Document,
    Image,
    Code,
    Archive,
    Audio,
}

impl Kind {
    /// Human readable kind, shown in the inspector strip.
    pub const fn label(self) -> &'static str {
        match self {
            Kind::Folder => "Folder",
            Kind::Document => "Document",
            Kind::Image => "Image",
            Kind::Code => "Source",
            Kind::Archive => "Archive",
            Kind::Audio => "Audio",
        }
    }

    /// Vector icon for the kind.
    pub const fn icon(self) -> &'static mirui::render::path::Path {
        match self {
            Kind::Folder => &icons::FOLDER,
            Kind::Document => &icons::DOCUMENT,
            Kind::Image => &icons::IMAGE,
            Kind::Code => &icons::CODE,
            Kind::Archive => &icons::ARCHIVE,
            Kind::Audio => &icons::AUDIO,
        }
    }

    /// Accent colour used to tint the icon.
    pub const fn tint(self) -> mirui::types::Color {
        use mirui::types::Color;
        match self {
            Kind::Folder => Color::rgb(0x3B, 0x9E, 0xF5),
            Kind::Document => Color::rgb(0x8E, 0x8E, 0x93),
            Kind::Image => Color::rgb(0x34, 0xC7, 0x59),
            Kind::Code => Color::rgb(0xFF, 0x95, 0x0A),
            Kind::Archive => Color::rgb(0xAF, 0x52, 0xDE),
            Kind::Audio => Color::rgb(0xFF, 0x37, 0x50),
        }
    }
}

/// One synthetic file.
#[derive(Clone, Copy, Debug)]
pub struct Entry {
    pub id: u32,
    pub name: &'static str,
    pub ext: &'static str,
    pub kind: Kind,
    pub size_kb: u32,
    pub day: u8,
}

impl Entry {
    /// Name plus extension, without allocating for the common case.
    pub fn display_name(&self) -> String {
        let mut out = String::from(self.name);
        if !self.ext.is_empty() {
            out.push('.');
            out.push_str(self.ext);
        }
        out
    }

    /// Human readable size. Folders have no size.
    pub fn size_label(&self) -> String {
        if self.kind == Kind::Folder {
            return String::new();
        }
        if self.size_kb >= 1024 {
            let mut out = alloc::format!("{} MB", self.size_kb / 1024);
            out.shrink_to_fit();
            out
        } else {
            let mut out = alloc::format!("{} KB", self.size_kb);
            out.shrink_to_fit();
            out
        }
    }

    /// Modified date label.
    pub fn date_label(&self) -> String {
        let mut out = String::from("Oct ");
        out.push_str(month_day(self.day));
        out
    }
}

const MONTH_DAYS: [&str; 31] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16", "17",
    "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30", "31",
];

const fn month_day(day: u8) -> &'static str {
    let index = if (day as usize) > 30 {
        30
    } else {
        day as usize
    };
    MONTH_DAYS[index]
}

const NAMES: [&str; 60] = [
    "Quarterly Report",
    "Design Notes",
    "Roadmap",
    "Meeting Minutes",
    "Invoices",
    "Passport",
    "Contracts",
    "Budget 2026",
    "Roadmap Draft",
    "Screenshots",
    "Build Log",
    "Kernel Notes",
    "Bootloader",
    "Device Tree",
    "Linker Script",
    "Compiler Explorer",
    "Profiling",
    "Trace Dump",
    "Heap Snapshot",
    "Benchmarks",
    "Wireframes",
    "Mockups",
    "Colour Palette",
    "Icon Set",
    "Typography Spec",
    "Release Notes",
    "Changelog",
    "Migration Plan",
    "Test Matrix",
    "Coverage Report",
    "Architecture",
    "ADR 001",
    "ADR 002",
    "Meeting Room",
    "Recipes",
    "Groceries",
    "Fitness Log",
    "Workout Plan",
    "Travel Itinerary",
    "Expense Sheet",
    "Tax Records",
    "Property Deeds",
    "Insurance",
    "Warranty",
    "Receipts",
    "Subscriptions",
    "Music Library",
    "Podcasts",
    "Recordings",
    "Backups",
    "Archives",
    "Photos 2024",
    "Photos 2025",
    "Photos 2026",
    "Scans",
    "Documents",
    "Templates",
    "Signatures",
    "Handover",
    "Retro Notes",
];

const fn make_entry(i: usize) -> Entry {
    let kind = match i % 11 {
        0 => Kind::Folder,
        1 | 2 => Kind::Image,
        3..=5 => Kind::Code,
        6 | 7 => Kind::Archive,
        8 => Kind::Audio,
        _ => Kind::Document,
    };
    let ext = match kind {
        Kind::Folder => "",
        Kind::Document => "pdf",
        Kind::Image => "png",
        Kind::Code => "rs",
        Kind::Archive => "zip",
        Kind::Audio => "mp3",
    };
    Entry {
        id: i as u32,
        name: NAMES[i % NAMES.len()],
        ext,
        kind,
        size_kb: ((i as u32).wrapping_mul(3_131) % 480_000) + 4,
        day: (i % 28) as u8 + 1,
    }
}

/// The whole fake filesystem, built at compile time.
pub static ENTRIES: [Entry; ENTRY_COUNT] = {
    let mut table = [Entry {
        id: 0,
        name: "",
        ext: "",
        kind: Kind::Document,
        size_kb: 0,
        day: 0,
    }; ENTRY_COUNT];
    let mut i = 0;
    while i < ENTRY_COUNT {
        table[i] = make_entry(i);
        i += 1;
    }
    table
};

/// Every place shown in the sidebar.
pub const PLACES: [Place; 5] = [
    Place::Favourites,
    Place::Documents,
    Place::Downloads,
    Place::Projects,
    Place::Trash,
];

/// Display name of a place.
pub const fn place_name(place: Place) -> &'static str {
    match place {
        Place::Favourites => "Favourites",
        Place::Documents => "Documents",
        Place::Downloads => "Downloads",
        Place::Projects => "Projects",
        Place::Trash => "Trash",
    }
}

/// Icon of a place.
pub const fn place_icon(place: Place) -> &'static mirui::render::path::Path {
    match place {
        Place::Favourites => &icons::STAR,
        Place::Documents => &icons::DOCUMENT,
        Place::Downloads => &icons::DOWNLOAD,
        Place::Projects => &icons::LAYERS,
        Place::Trash => &icons::TRASH,
    }
}

/// Inclusive start and exclusive end of a place inside [`ENTRIES`].
pub const fn place_range(place: Place) -> (usize, usize) {
    match place {
        Place::Favourites => (0, 8),
        Place::Documents => (8, 520),
        Place::Downloads => (520, 900),
        Place::Projects => (900, 1300),
        Place::Trash => (1300, 1400),
    }
}

/// Number of entries in a place.
pub const fn place_count(place: Place) -> usize {
    let (start, end) = place_range(place);
    end - start
}

/// The place an absolute entry index belongs to.
pub fn place_of(index: usize) -> crate::model::Place {
    for place in PLACES {
        let (start, end) = place_range(place);
        if index >= start && index < end {
            return place;
        }
    }
    crate::model::Place::Documents
}

/// Entry at an absolute index.
pub fn entry(index: usize) -> &'static Entry {
    &ENTRIES[index.min(ENTRY_COUNT - 1)]
}

/// Case-insensitive substring test that never allocates.
pub fn contains_ci(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return true;
    }
    let hay = haystack.as_bytes();
    let ned = needle.as_bytes();
    if ned.len() > hay.len() {
        return false;
    }
    let last = hay.len() - ned.len();
    let mut start = 0;
    while start <= last {
        let mut offset = 0;
        while offset < ned.len() {
            let a = hay[start + offset].to_ascii_lowercase();
            let b = ned[offset].to_ascii_lowercase();
            if a != b {
                break;
            }
            offset += 1;
        }
        if offset == ned.len() {
            return true;
        }
        start += 1;
    }
    false
}

/// Indices of the entries of `place` that match `query`, in display order.
///
/// Folders sort first, then by name — the way a file manager is expected to behave.
pub fn visible(place: Place, query: &str) -> Vec<usize> {
    let (start, end) = place_range(place);
    let mut folders: Vec<usize> = Vec::new();
    let mut files: Vec<usize> = Vec::new();
    let mut index = start;
    while index < end {
        let entry = &ENTRIES[index];
        let matched =
            query.is_empty() || contains_ci(entry.name, query) || contains_ci(entry.ext, query);
        if matched {
            if entry.kind == Kind::Folder {
                folders.push(index);
            } else {
                files.push(index);
            }
        }
        index += 1;
    }
    files.sort_by(|a, b| ENTRIES[*a].name.cmp(ENTRIES[*b].name));
    folders.extend(files);
    folders
}
