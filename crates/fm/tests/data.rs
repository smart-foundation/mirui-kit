//! Headless assertions about the data layer and the state model.

use std::string::String;
use std::vec::Vec;

use mirui::app::App;

use fm::data::{self, Kind};
use fm::model::{FmState, Place, ViewMode};

#[test]
fn entry_table_is_fully_populated() {
    assert_eq!(data::ENTRIES.len(), data::ENTRY_COUNT);
    assert_ne!(data::entry(0).name, "");
    assert!(data::entry(0).id == 0);
    assert_eq!(data::entry(7).id, 7);
}

#[test]
fn size_labels_are_populated() {
    for index in [0usize, 1, 2, 3, 17, 250, 1000, 2047] {
        let entry = data::entry(index);
        let label = entry.size_label();
        if entry.kind == Kind::Folder {
            // Folders have no size, and the typeface has no dash to say so.
            assert!(label.is_empty(), "unexpected size for folder {index}");
        } else {
            assert!(!label.is_empty(), "empty size for {index}");
            assert!(label.ends_with(" KB") || label.ends_with(" MB"), "{label}");
        }
    }
}

#[test]
fn dates_are_populated() {
    for index in [0usize, 5, 99, 2047] {
        let label = data::entry(index).date_label();
        assert!(label.starts_with("Oct "), "{label}");
        assert!(label.len() > 4, "{label}");
    }
}

#[test]
fn display_names_carry_the_extension() {
    let document = data::entry(1);
    assert_eq!(document.kind, Kind::Image);
    let name = document.display_name();
    assert!(name.ends_with(".png"), "{name}");

    let folder = data::entry(0);
    assert_eq!(folder.kind, Kind::Folder);
    assert!(!folder.display_name().contains('.'));
}

#[test]
fn filtering_is_case_insensitive_and_substring_based() {
    assert!(data::contains_ci("Quarterly Report", "report"));
    assert!(data::contains_ci("Quarterly Report", "REPORT"));
    assert!(!data::contains_ci("Quarterly Report", "zzz"));
    assert!(data::contains_ci("anything", ""));
}

#[test]
fn folders_sort_before_files() {
    let visible = data::visible(Place::Documents, "");
    assert_eq!(visible.len(), data::place_count(Place::Documents));
    let mut seen_file = false;
    for index in visible {
        let is_folder = data::entry(index).kind == Kind::Folder;
        if !is_folder {
            seen_file = true;
        } else {
            assert!(!seen_file, "a folder appeared after a file");
        }
    }
}

#[test]
fn a_query_narrows_the_result_set() {
    let all = data::visible(Place::Documents, "");
    let filtered = data::visible(Place::Documents, "report");
    assert!(filtered.len() < all.len());
    for index in filtered {
        let entry = data::entry(index);
        assert!(data::contains_ci(entry.name, "report") || data::contains_ci(entry.ext, "report"));
    }
}

#[test]
fn places_do_not_overlap() {
    let mut seen = Vec::new();
    for place in data::PLACES {
        let (start, end) = data::place_range(place);
        assert!(start < end);
        for index in start..end {
            assert!(!seen.contains(&index), "place {place:?} overlaps");
            seen.push(index);
        }
    }
}

#[test]
fn the_model_tracks_the_filter_and_the_selection() {
    let mut app = App::headless(900, 560);
    app.with_default_widgets();
    let state = app.add_model(FmState::new()).finish();
    state.recompute();

    assert_eq!(state.place(), Place::Documents);
    assert_eq!(state.view(), ViewMode::Icons);
    assert!(state.dark());
    assert_eq!(state.total(), data::place_count(Place::Documents) as u32);
    assert_eq!(state.shown(), fm::model::ICON_GRID_MAX);

    state.set_place(Place::Downloads);
    assert_eq!(state.place(), Place::Downloads);
    assert_eq!(state.total(), data::place_count(Place::Downloads) as u32);
    assert!(state.can_go_back());

    state.go_back();
    assert_eq!(state.place(), Place::Documents);
    assert!(!state.can_go_back());

    state.set_query("report");
    assert_eq!(state.query_text(), String::from("report"));
    assert!(state.total() < data::place_count(Place::Documents) as u32);
    assert!(state.total() > 0);

    state.set_query("");
    assert_eq!(state.total(), data::place_count(Place::Documents) as u32);

    state.select(42);
    assert_eq!(state.selected(), Some(42));
    assert_eq!(state.entry_id(0), state.entry_id(0));
    assert!(!state.entry_label(0).is_empty());
    assert!(!state.entry_date(0).is_empty());
    assert!(!state.entry_kind_at(0).is_empty());
    let first_file = (0..state.list_count()).find(|index| !state.entry_size(*index).is_empty());
    assert!(first_file.is_some(), "the place has no files with a size");
    assert!(state.count_label().ends_with("items"));
}

#[test]
fn the_grid_is_capped_but_the_list_is_not() {
    let mut app = App::headless(900, 560);
    app.with_default_widgets();
    let state = app.add_model(FmState::new()).finish();
    state.recompute();

    assert_eq!(
        state.grid_indices().len(),
        fm::model::ICON_GRID_MAX as usize
    );
    state.set_view(ViewMode::List);
    assert_eq!(state.shown(), state.total());
    assert_eq!(state.list_count(), state.total());
}

#[test]
fn overlay_pointer_ownership_survives_dismissal_until_release() {
    let mut app = App::headless(900, 560);
    app.with_default_widgets();
    let state = app.add_model(FmState::new()).finish();
    state.set_options_open(true);
    assert!(!state.update_content_pointer(true));
    state.set_options_open(false);
    assert!(
        !state.update_content_pointer(true),
        "held press leaked after dismissal"
    );
    assert!(state.update_content_pointer(false));
    assert!(
        state.update_content_pointer(true),
        "new press should be available"
    );
}

#[test]
fn every_interactive_overlay_blocks_content_pointer_tracking() {
    use fm::model::DialogKind;
    for overlay in 0..4 {
        let mut app = App::headless(900, 560);
        app.with_default_widgets();
        let state = app.add_model(FmState::new()).finish();
        match overlay {
            0 => state.open_background_menu(300, 180),
            1 => state.set_options_open(true),
            2 => state.open_dialog(DialogKind::NewFolder),
            _ => state.open_palette(),
        }
        assert!(state.has_input_overlay());
        assert!(!state.update_content_pointer(false));
        assert!(!state.update_content_pointer(true));
    }
}

#[test]
fn cancelling_a_drag_never_executes_its_pending_drop() {
    use fm::model::DropTarget;
    let mut app = App::headless(900, 560);
    app.with_default_widgets();
    let state = app.add_model(FmState::new()).finish();
    state.recompute();
    let id = state.entry_id(0);
    let original_place = state.place_of(id);
    state.press_tile(id, 300, 180);
    state.start_drag(id, 320, 200);
    state.set_drop_target(Some(DropTarget::Place(Place::Projects)));
    state.cancel_drag();
    state.end_drag();
    assert_eq!(state.place_of(id), original_place);
    assert_eq!(state.pressed_tile(), None);
    assert_eq!(state.dragging(), None);
    assert_eq!(state.drop_target(), None);
}
