//! The typeface only carries a subset of ASCII, and a missing glyph renders as
//! nothing at all. These tests make sure every string the UI can put on screen
//! is actually drawable.

use mirui::render::font::FontProvider;
use mirui::render::font::mirx::MirxFontProvider;

use fm::data;

fn provider() -> MirxFontProvider {
    MirxFontProvider::from_mirx(fm::FONT_BYTES, &mirx::reader::PayloadLimits::HOST)
        .expect("MiSans UI MIRX asset decodes")
}

fn assert_drawable(provider: &MirxFontProvider, text: &str, context: &str) {
    for ch in text.chars() {
        assert!(
            provider.map_char(ch).is_some(),
            "{context}: {ch:?} (U+{:04X}) is missing from the typeface",
            ch as u32
        );
    }
}

#[test]
fn every_file_name_is_drawable() {
    let provider = provider();
    for index in 0..data::ENTRY_COUNT {
        let entry = data::entry(index);
        assert_drawable(&provider, entry.name, "file name");
        assert_drawable(&provider, entry.ext, "file extension");
        assert_drawable(&provider, &entry.display_name(), "display name");
        assert_drawable(&provider, &entry.size_label(), "size label");
        assert_drawable(&provider, &entry.date_label(), "date label");
    }
}

#[test]
fn every_chrome_label_is_drawable() {
    let provider = provider();
    let labels = [
        "PLACES",
        "Search",
        "Users demo Cloud",
        "128.4 GB available",
        "Favourites",
        "Documents",
        "Downloads",
        "Projects",
        "Trash",
        "Folder",
        "Document",
        "Image",
        "Source",
        "Archive",
        "Audio",
        "items",
        "of",
        "Oct ",
        " MB",
        " KB",
    ];
    for label in labels {
        assert_drawable(&provider, label, "chrome label");
    }
}

#[test]
fn every_place_and_kind_label_is_drawable() {
    let provider = provider();
    for place in data::PLACES {
        assert_drawable(&provider, data::place_name(place), "place name");
    }
    for kind in [
        data::Kind::Folder,
        data::Kind::Document,
        data::Kind::Image,
        data::Kind::Code,
        data::Kind::Archive,
        data::Kind::Audio,
    ] {
        assert_drawable(&provider, kind.label(), "kind label");
    }
}

#[test]
fn the_typeface_reports_its_own_limits() {
    let provider = provider();
    // The characters this project relies on being present.
    for ch in ['A', 'z', '0', ' ', '.', ',', ':', '!', '?'] {
        assert!(provider.map_char(ch).is_some(), "{ch:?} should be drawable");
    }
    // Known gaps: the asset has no punctuation beyond this set, which is why
    // the chrome avoids dashes, slashes and brackets.
    for ch in ['-', '/', '%', '(', ')', '_'] {
        assert!(
            provider.map_char(ch).is_none(),
            "{ch:?} is unexpectedly present"
        );
    }
}
