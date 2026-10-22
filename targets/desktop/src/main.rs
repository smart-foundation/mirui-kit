//! Interactive desktop shell: an SDL2 window showing the file manager.

use mirui::app::App;
use mirui::surface::sdl::SdlSurface;

const WIDTH: u16 = 1180;
const HEIGHT: u16 = 720;

fn main() {
    let backend = SdlSurface::new("Files", WIDTH, HEIGHT);
    let mut app = App::new(backend);
    app.with_default_widgets().with_default_systems();
    // The gesture recogniser needs a clock to recognise a long press.
    app.add_plugin(mirui::app::plugins::StdInstantClockPlugin);
    fm::install(&mut app);
    app.run();
}
