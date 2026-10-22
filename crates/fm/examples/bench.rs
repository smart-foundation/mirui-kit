use mirui::app::App;
use mirui::render::texture::ColorFormat;
use mirui::surface::framebuf::FramebufSurface;
use mirui::types::PhysicalRect;
use std::time::Instant;

fn main() {
    let backend = FramebufSurface::with_format(
        1180,
        720,
        ColorFormat::RGBA8888,
        |_: &[u8], _: PhysicalRect| {},
    );
    let mut app = App::new(backend);
    app.with_default_widgets().with_default_systems();
    app.add_plugin(mirui::app::plugins::StdInstantClockPlugin);
    app.add_plugin(mirui::app::plugins::PerfReportPlugin::new(60));
    let state = fm::install(&mut app);

    let t0 = Instant::now();
    app.render().expect("first frame");
    println!("first frame: {:?}", t0.elapsed());

    // What the app really does: a tick, which only redraws what changed.
    let t = Instant::now();
    for _ in 0..60 {
        app.tick();
    }
    println!("60 idle ticks: {:?}", t.elapsed());

    let t = Instant::now();
    for i in 0..60 {
        state.select(i as u32);
        app.tick();
    }
    println!("60 selection changes: {:?}", t.elapsed());

    let t = Instant::now();
    for _ in 0..20 {
        state.set_query("report");
        app.tick();
        state.set_query("");
        app.tick();
    }
    println!("20 filter changes: {:?}", t.elapsed());

    let t = Instant::now();
    for i in 0..20 {
        state.set_place(if i % 2 == 0 {
            fm::model::Place::Downloads
        } else {
            fm::model::Place::Documents
        });
        app.tick();
    }
    println!("20 place changes: {:?}", t.elapsed());

    // The view toggle: the case that used to melt down.
    let t = Instant::now();
    for i in 0..20 {
        state.set_view(if i % 2 == 0 {
            fm::model::ViewMode::List
        } else {
            fm::model::ViewMode::Icons
        });
        app.tick();
    }
    println!("20 view toggles: {:?}", t.elapsed());

    // How does the list view behave once it is up?
    state.set_view(fm::model::ViewMode::List);
    app.tick();
    let t = Instant::now();
    for _ in 0..60 {
        app.tick();
    }
    println!("60 idle ticks in the list view: {:?}", t.elapsed());

    let t = Instant::now();
    for i in 0..20u32 {
        state.select(i);
        app.tick();
    }
    println!("20 selections in the list view: {:?}", t.elapsed());
}
