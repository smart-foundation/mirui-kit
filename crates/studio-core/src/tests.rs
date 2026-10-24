use super::*;
#[test]
fn six_routes_and_seventeen_surfaces_have_distinct_names() {
    for (i, a) in Surface::ALL.iter().enumerate() {
        for b in &Surface::ALL[i + 1..] {
            assert_ne!(a.title(), b.title());
        }
    }
    assert_eq!(Route::ALL.len(), 6);
}
#[test]
fn idle_ticks_do_not_request_updates() {
    assert!(!Session::default().tick());
}
#[test]
fn preview_requires_valid_name() {
    let mut s = Session::default();
    s.edit_preset("  ", 80, true);
    assert_eq!(s.start_preview(), Err(Error::EmptyName));
}
#[test]
fn edited_preview_rejects_old_result_and_does_not_restart() {
    let mut s = Session::default();
    let v = s.start_preview().unwrap();
    s.edit_preset("Changed", 70, false);
    assert!(s.job.is_none());
    assert_eq!(s.complete_preview(v), Err(Error::Stale));
    assert!(s.preview.is_none());
}
#[test]
fn preview_completes_once_in_four_ticks() {
    let mut s = Session::default();
    s.start_preview().unwrap();
    for _ in 0..4 {
        assert!(s.tick());
    }
    assert!(s.preview.is_some());
    assert!(!s.tick());
}
#[test]
fn preset_conflicts_do_not_overwrite() {
    let mut s = Session::default();
    s.save_preset().unwrap();
    assert_eq!(s.save_preset(), Err(Error::DuplicateName));
    assert_eq!(s.presets.len(), 1);
}
#[test]
fn folder_draft_cancel_removes_only_draft() {
    let mut s = Session::default();
    let before = s.assets.len();
    let id = s.begin_new().unwrap();
    s.cancel_edit();
    assert_eq!(s.assets.len(), before);
    assert!(s.asset(id).is_none());
    assert!(s.selected.is_none());
}
#[test]
fn rename_validates_and_preserves_identity() {
    let mut s = Session::default();
    s.select(3).unwrap();
    s.begin_rename().unwrap();
    assert_eq!(s.commit_name("../bad"), Err(Error::InvalidName));
    s.commit_name("Brief.txt").unwrap();
    assert_eq!(s.selected, Some(3));
    assert_eq!(s.asset(3).unwrap().name, "Brief.txt");
}
#[test]
fn conflicting_rename_preserves_old_name_and_editor() {
    let mut s = Session::default();
    s.select(3).unwrap();
    s.begin_rename().unwrap();
    assert_eq!(s.commit_name("brand"), Err(Error::DuplicateName));
    assert_eq!(s.asset(3).unwrap().name, "Welcome.txt");
    assert_eq!(s.editing, Some(3));
}
#[test]
fn stable_ids_survive_sort_and_filter() {
    let mut s = Session::default();
    s.select(3).unwrap();
    s.descending = true;
    s.query = "welcome".into();
    assert_eq!(s.visible_assets(), vec![3]);
    assert_eq!(s.selected, Some(3));
}
#[test]
fn selection_cannot_cross_folders() {
    let mut s = Session::default();
    assert_eq!(s.select(4), Err(Error::MissingAsset));
}
#[test]
fn folder_cannot_move_into_itself_or_descendant() {
    let mut s = Session::default();
    s.select(1).unwrap();
    assert_eq!(s.move_selected(1), Err(Error::Cycle));
    s.folder = 1;
    let child = s.begin_new().unwrap();
    s.commit_name("Child").unwrap();
    s.selected = Some(1);
    assert_eq!(s.move_selected(child), Err(Error::Cycle));
}
#[test]
fn move_checks_conflicts_before_mutating() {
    let mut s = Session::default();
    s.folder = 1;
    s.begin_new().unwrap();
    s.commit_name("Welcome.txt").unwrap();
    s.folder = 0;
    s.select(3).unwrap();
    assert_eq!(s.move_selected(1), Err(Error::DuplicateName));
    assert_eq!(s.asset(3).unwrap().parent, 0);
}
#[test]
fn move_is_session_only_and_clears_selection() {
    let mut s = Session::default();
    s.select(3).unwrap();
    s.move_selected(2).unwrap();
    assert_eq!(s.asset(3).unwrap().parent, 2);
    assert!(s.selected.is_none());
}
#[test]
fn unselected_inspector_is_unavailable() {
    assert_eq!(
        Session::default().open(Surface::Inspector),
        Err(Error::NoSelection)
    );
}
#[test]
fn surface_ownership_blocks_replacement_and_navigation() {
    let mut s = Session::default();
    s.open(Surface::Help).unwrap();
    assert_eq!(s.open(Surface::Palette), Err(Error::Blocked));
    assert_eq!(s.navigate(Route::Controls), Err(Error::Blocked));
}
#[test]
fn note_guard_cancel_keeps_dirty_note() {
    let mut s = Session::default();
    s.route = Route::Workbench;
    s.note = "Edited".into();
    s.navigate(Route::Controls).unwrap();
    assert_eq!(s.surface, Some(Surface::Unsaved));
    s.dismiss();
    assert_eq!(s.route, Route::Workbench);
    assert!(s.dirty());
    assert!(s.pending_route.is_none());
}
#[test]
fn save_and_leave_saves_to_session() {
    let mut s = Session::default();
    s.route = Route::Workbench;
    s.note = "Edited".into();
    s.navigate(Route::Collections).unwrap();
    s.finish_navigation(true).unwrap();
    assert_eq!(s.route, Route::Collections);
    assert!(!s.dirty());
}
#[test]
fn discard_and_leave_restores_saved_note() {
    let mut s = Session::default();
    let saved = s.note.clone();
    s.route = Route::Workbench;
    s.note = "Edited".into();
    s.navigate(Route::Controls).unwrap();
    s.finish_navigation(false).unwrap();
    assert_eq!(s.note, saved);
}
#[test]
fn review_rejects_stale_data_and_double_commit() {
    let mut s = Session::default();
    s.open(Surface::Confirm).unwrap();
    s.revision += 1;
    assert_eq!(s.apply_review(), Err(Error::Stale));
    s.dismiss();
    s.open(Surface::Confirm).unwrap();
    s.apply_review().unwrap();
    assert_eq!(s.apply_review(), Err(Error::Stale));
    assert_eq!(s.applied_changes, 1);
}
#[test]
fn appearance_auto_reverts_after_ten_seconds() {
    let mut s = Session::default();
    let old = s.appearance;
    s.draft.theme = Theme::Light;
    s.apply_appearance().unwrap();
    for _ in 0..40 {
        s.tick();
    }
    assert_eq!(s.appearance, old);
    assert!(s.surface.is_none());
}
#[test]
fn appearance_keep_cancels_recovery() {
    let mut s = Session::default();
    s.draft.theme = Theme::Light;
    s.apply_appearance().unwrap();
    s.keep_appearance();
    for _ in 0..50 {
        s.tick();
    }
    assert_eq!(s.appearance.theme, Theme::Light);
}
#[test]
fn recovery_dismiss_is_revert_not_keep() {
    let mut s = Session::default();
    s.draft.theme = Theme::Light;
    s.apply_appearance().unwrap();
    s.dismiss();
    assert_eq!(s.appearance.theme, Theme::Dark);
}
#[test]
fn appearance_rejects_unreadable_scale() {
    let mut s = Session::default();
    s.draft.scale = 50;
    assert_eq!(s.apply_appearance(), Err(Error::InvalidScale));
    assert_eq!(s.appearance.scale, 100);
}
#[test]
fn notice_history_is_bounded() {
    let mut s = Session::default();
    for i in 0..100 {
        s.notify(format!("Notice {i}"));
    }
    assert_eq!(s.notices.len(), 3);
    assert_eq!(s.notices[0], "Notice 97");
}
#[test]
fn reset_cannot_bypass_unsaved_guard() {
    let mut s = Session::default();
    s.route = Route::Workbench;
    s.note = "Edited".into();
    s.navigate(Route::Overview).unwrap();
    assert_eq!(s.open(Surface::Reset), Err(Error::Blocked));
}

#[test]
fn selecting_another_asset_cancels_only_the_inline_draft() {
    let mut s = Session::default();
    let draft = s.begin_new().unwrap();
    s.select(3).unwrap();
    assert!(s.asset(draft).is_none());
    assert_eq!(s.selected, Some(3));
    assert!(s.editing.is_none());
}
#[test]
fn moving_an_uncommitted_name_is_blocked() {
    let mut s = Session::default();
    let draft = s.begin_new().unwrap();
    assert_eq!(s.move_selected(2), Err(Error::Blocked));
    assert_eq!(s.asset(draft).unwrap().parent, 0);
}
#[test]
fn session_asset_growth_is_bounded() {
    let mut s = Session::default();
    for i in 0..123 {
        s.begin_new().unwrap();
        s.commit_name(&format!("Folder {i}")).unwrap();
    }
    assert_eq!(s.assets.len(), 128);
    assert_eq!(s.begin_new(), Err(Error::Capacity));
    assert_eq!(s.assets.len(), 128);
}
#[test]
fn saved_preset_growth_is_bounded() {
    let mut s = Session::default();
    for i in 0..32 {
        s.edit_preset(&format!("Preset {i}"), 80, true);
        s.save_preset().unwrap();
    }
    s.edit_preset("One too many", 80, true);
    assert_eq!(s.save_preset(), Err(Error::Capacity));
    assert_eq!(s.presets.len(), 32);
}
#[test]
fn normalized_quality_does_not_restart_or_invalidate_a_job() {
    let mut s = Session::default();
    s.edit_preset("Web preview", 100, true);
    let version = s.start_preview().unwrap();
    s.edit_preset("Web preview", 200, true);
    assert_eq!(s.preset_version, version);
    assert!(s.job.is_some());
}

#[test]
fn filtering_clears_a_hidden_selection() {
    let mut s = Session::default();
    s.select(3).unwrap();
    s.set_query("No matching sample");
    assert!(s.selected.is_none());
    assert!(s.visible_assets().is_empty());
}
#[test]
fn filtering_cancels_inline_naming_without_leaving_an_invisible_draft() {
    let mut s = Session::default();
    let draft = s.begin_new().unwrap();
    s.set_query("Brand");
    assert!(s.editing.is_none());
    assert!(s.asset(draft).is_none());
    assert_eq!(s.visible_assets(), vec![1]);
}
