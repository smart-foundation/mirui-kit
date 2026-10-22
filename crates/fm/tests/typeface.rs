//! The typeface tests.
//!
//! Three things matter here: that every asset decodes under `mirui` 0.51,
//! that the sizes the interface uses are served by a coverage atlas at their
//! exact pixel size (a coverage atlas is blitted one to one, so the glyphs
//! stay sharp), and that every character the interface can put on screen is
//! drawable.

use std::path::{Path, PathBuf};

use mirui::render::font::FontProvider;
use mirui::render::font::mirx::MirxFontProvider;
use mirx::font::FontRepresentationKind;

use fm::data;
use fm::text;

/// The weights that are baked into the repository.
const WEIGHTS: [&str; 9] = [
    "thin",
    "extralight",
    "light",
    "regular",
    "medium",
    "semibold",
    "bold",
    "extrabold",
    "black",
];

/// The directory the assets live in.
fn assets() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets/fonts")
        .canonicalize()
        .expect("the asset directory exists")
}

/// The output sizes a logical size is drawn at: 1x, at 1.5x and at 2x.
///
/// `mirui` picks the representation by the output ppem, that is the logical
/// size multiplied by the viewport scale and rounded up.
fn output_sizes(size: u16) -> [u16; 3] {
    [
        size,
        (u32::from(size) * 3 / 2).min(u32::from(u16::MAX)) as u16,
        size.saturating_mul(2),
    ]
}

fn provider(bytes: &'static [u8]) -> MirxFontProvider {
    MirxFontProvider::from_mirx(bytes, &mirx::reader::PayloadLimits::HOST)
        .expect("the MIRX asset decodes")
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
fn every_baked_weight_decodes() {
    for weight in WEIGHTS {
        let path = assets().join(format!("inter-{weight}.mirx"));
        let Ok(bytes) = std::fs::read(&path) else {
            panic!("{} is missing", path.display());
        };
        let bytes: &'static [u8] = Box::leak(bytes.into_boxed_slice());
        let provider = provider(bytes);
        assert!(
            provider.map_char('A').is_some(),
            "{weight}: the typeface has no glyphs"
        );
    }
}

#[test]
fn the_scale_is_served_by_coverage_at_the_exact_size() {
    let bytes: &'static [u8] = fm::font::REGULAR;
    let provider = provider(bytes);
    let glyph = provider.map_char('A').expect("A");

    // The sizes the interface uses, and the same sizes on a scaled display.
    for size in [text::SMALL, text::BODY, text::TITLE] {
        for output in output_sizes(size) {
            let raster = provider.raster(glyph, size, output).expect("a raster");
            assert!(
                matches!(
                    raster.representation.kind(),
                    FontRepresentationKind::Coverage { .. }
                ),
                "{size}px at {output}px is served by {:?}, not by a coverage atlas",
                raster.representation.kind()
            );
            assert_eq!(
                raster.representation.design_ppem(),
                output,
                "{size}px at {output}px is served by a {}ppem atlas",
                raster.representation.design_ppem()
            );
        }
    }
}

#[test]
fn an_unbaked_size_falls_back_to_the_signed_distance_field() {
    let bytes: &'static [u8] = fm::font::REGULAR;
    let provider = provider(bytes);
    let glyph = provider.map_char('A').expect("A");
    // 19ppem is in neither the 1x nor the 1.5x band.
    let raster = provider.raster(glyph, 19, 19).expect("a raster");
    assert!(
        matches!(
            raster.representation.kind(),
            FontRepresentationKind::SignedDistance { .. }
        ),
        "19ppem is served by {:?}",
        raster.representation.kind()
    );
}

#[test]
fn every_file_name_is_drawable() {
    let provider = provider(fm::font::REGULAR);
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
    let provider = provider(fm::font::REGULAR);
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
    let provider = provider(fm::font::REGULAR);
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
fn the_typeface_carries_the_punctuation_the_chrome_uses() {
    let provider = provider(fm::font::REGULAR);
    // The characters this project relies on being present.
    for ch in ['A', 'z', '0', ' ', '.', ',', ':', '!', '?'] {
        assert!(provider.map_char(ch).is_some(), "{ch:?} should be drawable");
    }
    // The punctuation the reduced MiSans asset lacked.
    for ch in ['-', '/', '%', '(', ')', '_', '&', '+', '·', '—'] {
        assert!(
            provider.map_char(ch).is_some(),
            "{ch:?} is unexpectedly missing"
        );
    }
}

#[test]
fn the_interface_lays_out_without_a_text_error() {
    use mirui::app::App;
    use mirui::render::texture::{ColorFormat, Texture};
    use mirui::surface::{DisplayInfo, FramebufferAccess, InputEvent, Surface};
    use mirui::types::{Fixed, PhysicalRect};

    const WIDTH: u16 = 1180;
    const HEIGHT: u16 = 720;

    struct ScriptedSurface {
        pixels: Vec<u8>,
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
            None
        }
    }

    impl FramebufferAccess for ScriptedSurface {
        fn framebuffer(&mut self) -> Texture<'_> {
            Texture::new(&mut self.pixels, WIDTH, HEIGHT, ColorFormat::RGBA8888)
                .expect("valid pixel storage")
        }
    }

    let mut app = App::new(ScriptedSurface {
        pixels: vec![0; WIDTH as usize * HEIGHT as usize * 4],
    });
    app.with_default_widgets().with_default_systems();
    app.add_plugin(mirui::app::plugins::StdInstantClockPlugin);
    fm::install(&mut app);
    let result = app.render();
    if let Some(failure) = app
        .world
        .resource::<mirui::ui::render_system::TextLayoutFailure>()
    {
        panic!(
            "the layout failed: stage {:?} kind {:?} entity {:?}",
            failure.stage, failure.kind, failure.entity
        );
    }
    if let Some(failure) = app.last_text_content_failure() {
        let text = app
            .world
            .query::<mirui::ui::widgets::Text>()
            .iter()
            .find(|(entity, _)| *entity == failure.entity)
            .map(|(_, text)| text.clone());
        panic!(
            "a text widget failed to lay out: {:?} for {:?}",
            failure.error, text
        );
    }
    result.expect("the interface lays out");
}

/// Collect the contents of the string literals of a source file.
fn literals(source: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut current = String::new();
    let mut in_string = false;
    let mut escaped = false;
    for ch in source.chars() {
        if in_string {
            if escaped {
                current.push(ch);
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
                found.push(std::mem::take(&mut current));
            } else {
                current.push(ch);
            }
        } else if ch == '"' {
            in_string = true;
            current.clear();
        }
    }
    found
}

#[test]
fn every_character_in_the_source_is_drawable() {
    let provider = provider(fm::font::REGULAR);
    let source_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files: Vec<PathBuf> = walk(&source_dir);
    files.sort();
    assert!(!files.is_empty(), "the crate has sources");
    for file in files {
        let source = std::fs::read_to_string(&file).expect("the source is text");
        for literal in literals(&source) {
            assert_drawable(&provider, &literal, &file.display().to_string());
        }
    }
}

/// Every Rust file below a directory.
fn walk(dir: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return found;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            found.extend(walk(&path));
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            found.push(path);
        }
    }
    found
}
