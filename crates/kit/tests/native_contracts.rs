//! Real Mirui model/property regressions, not macro-only validation.
use mirui::{prelude::*, ui::{UserState, property::Property}};
use mirui_kit::{Actions, availability::{InputAvailability, InputBlocked}, controls::{ChoiceGroup, ActivationError}, behavior::group::{GroupItem, GroupKind, Orientation}, forms::NumericEntry};
#[test]
fn native_availability_sets_native_state_and_query_marker_together() {
    let mut world = mirui::ecs::World::new();
    let node = world.spawn_empty();
    InputAvailability::apply(&mut world, node, false);
    assert!(world.has::<InputBlocked>(node));
    assert!(matches!(world.get::<UserState>(node), Some(UserState::Disabled)));
    InputAvailability::apply(&mut world, node, true);
    assert!(!world.has::<InputBlocked>(node));
    assert!(world.get::<UserState>(node).is_none());
}
#[test]
fn extraction_preserves_order_and_drains_through_native_transaction() {
    let mut app = App::headless(100,100);
    let actions = app.add_model(Actions::default()).finish();
    actions.activate(3,false); actions.activate(1,false);
    assert_eq!(actions.take(), [(3,false),(1,false)]);
    assert!(actions.take().is_empty());
}
#[test]
fn group_queue_backpressure_is_explicit_and_does_not_silently_drop() {
    let mut app = App::headless(100,100);
    let raw = ChoiceGroup::new(GroupKind::Actions,Orientation::Horizontal,
        alloc_items()).unwrap();
    let group = app.add_model(raw).finish();
    for _ in 0..32 { group.activate(9).unwrap(); }
    assert_eq!(group.activate(9),Err(ActivationError::Backpressured));
    for _ in 0..32 { assert_eq!(group.take_action(),Some(9)); }
    assert_eq!(group.take_action(),None);
    group.activate(9).unwrap();
}
fn alloc_items()->Vec<GroupItem> {vec![GroupItem{id:9,label:"Run".into(),disabled:false}]}
#[test]
fn numeric_error_getter_reads_owned_text_without_copy_constraints() {
    let mut app = App::headless(100,100);
    let number = app.add_model(NumericEntry::new(2,0,10,1).unwrap()).finish();
    number.document().set_text("-").unwrap(); number.commit();
    assert!(!number.error().is_empty()); assert_eq!(number.value(),2);
    number.document().set_text("3").unwrap(); number.commit();
    assert!(number.error().is_empty()); assert_eq!(number.value(),3);
}
#[test]
fn unblocking_restores_preexisting_native_error_or_disabled_state() {
    let mut world = mirui::ecs::World::new();
    let error = world.spawn_empty();
    world.insert(error,UserState::Errored);
    InputAvailability::apply(&mut world,error,false);
    InputAvailability::apply(&mut world,error,true);
    assert!(matches!(world.get::<UserState>(error),Some(UserState::Errored)));
    let disabled = world.spawn_empty();
    world.insert(disabled,UserState::Disabled);
    InputAvailability::apply(&mut world,disabled,false);
    InputAvailability::apply(&mut world,disabled,true);
    assert!(matches!(world.get::<UserState>(disabled),Some(UserState::Disabled)));
}

#[compose]
fn panels(group:model!(ChoiceGroup))->Entity {
    ui! {
        Column (width:100,height:100) {
            View (id:"first_panel",width:100,height:50) [mirui_kit::controls::TabPanel{group:group.clone(),item:1}]
            View (id:"second_panel",width:100,height:50) [mirui_kit::controls::TabPanel{group:group.clone(),item:2}]
        }
    }
}
#[test]
fn tab_panels_preserve_mounts_and_follow_explicit_activation() {
    let mut app=App::headless(100,100);
    app.with_default_widgets().with_default_systems();
    mirui_kit::install(&mut app);
    let group=app.add_model(ChoiceGroup::new(GroupKind::Single{required:true},Orientation::Horizontal,
        vec![GroupItem{id:1,label:"One".into(),disabled:false},GroupItem{id:2,label:"Two".into(),disabled:false}]).unwrap()).finish();
    let root=app.spawn_root().id();app.compose(root,panels(group.clone()));app.tick();
    let first=app.world.find_by_id("first_panel").unwrap();
    let second=app.world.find_by_id("second_panel").unwrap();
    assert!(!app.world.has::<mirui::ui::Hidden>(first));
    assert!(app.world.has::<mirui::ui::Hidden>(second));
    group.navigate(mirui_kit::behavior::group::Move::Next,false);app.tick();
    assert!(!app.world.has::<mirui::ui::Hidden>(first));
    group.activate(2).unwrap();app.tick();
    assert!(app.world.has::<mirui::ui::Hidden>(first));
    assert!(!app.world.has::<mirui::ui::Hidden>(second));
    assert!(app.world.is_alive(first));
}
