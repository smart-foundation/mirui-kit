//! Which representation serves each size?
fn main() {
    let bytes: &[u8] = include_bytes!("../../../assets/misans_ui.mirx");
    let limits = mirx::reader::PayloadLimits::HOST;
    let provider = fm::font::SdfFontProvider::from_mirx(bytes, &limits).expect("font");
    for size in [10u16, 11, 12, 13, 14, 15, 17, 22] {
        println!("size {size}: {:?}", provider.representation_kind(size));
    }
}
