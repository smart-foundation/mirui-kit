//! Print the representation ladder of a MIRX font asset.
//!
//! Usage: cargo run -p fm --example fontinfo -- assets/fonts/inter-regular.mirx

use std::env;

use mirui::render::font::mirx::MirxFontProvider;
use mirui::render::font::FontProvider;

fn main() {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "../../assets/fonts/inter-regular.mirx".to_string());
    let bytes = std::fs::read(&path).unwrap_or_else(|error| {
        eprintln!("{path}: {error}");
        std::process::exit(1);
    });
    let bytes: &'static [u8] = Box::leak(bytes.into_boxed_slice());
    let limits = mirx::reader::PayloadLimits::HOST;
    let provider = MirxFontProvider::from_mirx(bytes, &limits).expect("font decodes");
    println!("{path}: {} bytes", bytes.len());
    for size in [11u16, 12, 13, 14, 15, 16, 18, 20, 23, 24, 26, 28, 30, 32, 48] {
        let glyph = provider.map_char('A').expect("A");
        let raster = provider.raster(glyph, size, size);
        match raster {
            Some(r) => println!(
                "size {size}: {}x{} stride {} kind={:?} design_ppem={:?}",
                r.surface.width(),
                r.surface.height(),
                r.surface.stride(),
                r.representation.kind(),
                r.representation.design_ppem()
            ),
            None => println!("size {size}: no raster"),
        }
    }
}
