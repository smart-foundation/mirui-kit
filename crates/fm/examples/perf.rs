//! Where does a frame go?
//!
//! Prints the mirui span breakdown (`PerfReportPlugin`) for the three
//! interaction scenarios the file manager has to be fast in:
//!
//!   * a selection change (one click),
//!   * a filter change (one keystroke in the search field),
//!   * a place change (one click in the sidebar).
//!
//! It also reports how much of the screen each frame had to repaint, because
//! that is what decides whether a frame costs 0.1 ms or 4 ms.
//!
//! Run with:
//!     cargo run -p fm --example perf --release

use std::cell::RefCell;
use std::time::Instant;

use mirui::app::App;
use mirui::app::plugins::{PerfReport, PerfReportPlugin};
use mirui::render::texture::ColorFormat;
use mirui::surface::framebuf::FramebufSurface;
use mirui::types::PhysicalRect;
use mirui::ui::render_system::LastDirtyRegions;

thread_local! {
    static REPORTS: RefCell<Vec<PerfReport>> = const { RefCell::new(Vec::new()) };
}

fn sink(report: &PerfReport) {
    REPORTS.with(|r| r.borrow_mut().push(report.clone()));
}

fn take_reports() -> Vec<PerfReport> {
    REPORTS.with(|r| std::mem::take(&mut *r.borrow_mut()))
}

/// Aggregate a phase's frames into one row per span name.
fn summarise(label: &str, reports: &[PerfReport]) {
    let mut totals: Vec<(&'static str, u64, u64)> = Vec::new();
    for report in reports {
        for stat in &report.stage_stats {
            match totals.iter_mut().find(|(name, _, _)| *name == stat.name) {
                Some((_, count, total)) => {
                    *count += stat.count as u64;
                    *total += stat.total_ns;
                }
                None => totals.push((stat.name, stat.count as u64, stat.total_ns)),
            }
        }
    }
    totals.sort_by_key(|(_, _, total)| core::cmp::Reverse(*total));
    let frames = reports.len().max(1) as u64;
    println!("=== {label} ({frames} frames, ms per frame) ===");
    for (name, count, total) in totals {
        if total == 0 {
            continue;
        }
        println!(
            "  {name:<26} n={count:<6} {:>8.3} ms",
            total as f64 / 1e6 / frames as f64
        );
    }
    println!();
}

/// How much of the screen the last frame had to repaint, and in how many pieces.
fn damage<F: FnMut(&[u8], PhysicalRect)>(
    app: &App<FramebufSurface<F>, mirui::app::SwRendererFactory>,
    label: &str,
) {
    let Some(last) = app.world.resource::<LastDirtyRegions>() else {
        return;
    };
    let rects = last.0.rects.len();
    let area: u64 = last
        .0
        .rects
        .iter()
        .map(|r| r.w.to_int() as u64 * r.h.to_int() as u64)
        .sum();
    let screen = 1180u64 * 720;
    println!(
        "  {label:<24} {rects:>3} rect(s), {area:>8} px repainted ({:.1}% of the screen)",
        area as f64 / screen as f64 * 100.0
    );
}

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
    // One frame per report: every frame reports its own span breakdown.
    app.add_plugin(PerfReportPlugin::new(1).with_sink(sink));
    let state = fm::install(&mut app);

    let first = Instant::now();
    app.render().expect("first frame");
    println!("first frame: {:?}", first.elapsed());
    let reports = take_reports();
    summarise("first frame", &reports);

    // --- selection ------------------------------------------------------
    // Select ids that really are in the grid: this is "the user clicks a tile".
    let ids: Vec<u32> = (0..60).map(|i| state.grid_entry(i)).collect();
    let click = Instant::now();
    for id in &ids {
        state.select(*id);
        app.tick();
    }
    let click_ms = click.elapsed().as_secs_f64() * 1000.0;
    println!(
        "60 selection changes: {click_ms:.3} ms total, {:.3} ms each",
        click_ms / 60.0
    );
    let reports = take_reports();
    summarise("selection change", &reports);
    damage(&app, "selection change");

    // --- list view ------------------------------------------------------
    state.set_view(fm::model::ViewMode::List);
    app.tick();
    let list = Instant::now();
    for _ in 0..60 {
        app.tick();
    }
    println!("60 idle ticks in the list view: {:?} total", list.elapsed());
    let reports = take_reports();
    summarise("list view", &reports);
    damage(&app, "list view");
    state.set_view(fm::model::ViewMode::Icons);
    app.tick();

    // --- filter ---------------------------------------------------------
    let filter = Instant::now();
    for _ in 0..20 {
        state.set_query("report");
        app.tick();
        state.set_query("");
        app.tick();
    }
    let filter_ms = filter.elapsed().as_secs_f64() * 1000.0;
    println!(
        "20 filter changes: {filter_ms:.3} ms total, {:.3} ms each",
        filter_ms / 20.0
    );
    let reports = take_reports();
    summarise("filter change", &reports);
    damage(&app, "filter change");

    // --- place ----------------------------------------------------------
    let place = Instant::now();
    for i in 0..20 {
        state.set_place(if i % 2 == 0 {
            fm::model::Place::Downloads
        } else {
            fm::model::Place::Documents
        });
        app.tick();
    }
    let place_ms = place.elapsed().as_secs_f64() * 1000.0;
    println!(
        "20 place changes: {place_ms:.3} ms total, {:.3} ms each",
        place_ms / 20.0
    );
    let reports = take_reports();
    summarise("place change", &reports);
    damage(&app, "place change");
}
