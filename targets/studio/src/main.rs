//! SDL owns the 1400×900 stage; Studio remains a separate 1180×720 floating app.
use mirui::{app::App, surface::sdl::SdlSurface};
fn main() {
    let mut app = App::new(SdlSurface::new("Naledzi Studio", 1400, 900));
    app.with_default_widgets().with_default_systems();
    app.add_plugin(mirui::app::plugins::StdInstantClockPlugin);
    app.add_plugin(mirui::app::plugins::FrameRateCapPlugin::new(60));
    let _state = studio::install(&mut app);
    app.render().expect("Studio first frame");
    while !app.tick() {}
}
