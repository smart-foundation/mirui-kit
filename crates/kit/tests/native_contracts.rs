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

#[test]
fn option_replacement_and_selection_use_real_model_transactions() {
    use mirui_kit::{choices::ChoiceList,behavior::{choices::OptionItem,collection::{SelectionMode,Modifiers}}};
    let mut app=App::headless(100,100);
    let list=app.add_model(ChoiceList::new(vec![OptionItem{id:7,label:"Seven".into(),disabled:false}],SelectionMode::Multiple).unwrap()).finish();
    list.choose(7,Modifiers::default());
    assert_eq!(list.selected_ids(),[7]);
    assert_eq!(list.replace(vec![OptionItem{id:7,label:"Changed".into(),disabled:true}]),Ok(()));
    assert!(list.disabled(7));assert!(list.selected_ids().is_empty());
    assert_eq!(list.label(7),"Changed");
    assert_eq!(list.replace(vec![OptionItem{id:1,label:"One".into(),disabled:false},OptionItem{id:1,label:"Duplicate".into(),disabled:false}]),Err(1));
    assert_eq!(list.label(7),"Changed");
}
#[compose]
fn configured_button(actions:model!(Actions),state:model!(mirui_kit::button::ButtonState))->Entity {
    use mirui_kit::button::{button,ButtonSpec};
    ui! {button(actions,state,9,ButtonSpec::labelled("Run"),false)}
}
#[test]
fn pending_buttons_block_native_input_and_recover() {
    use mirui_kit::button::{ButtonState,ButtonAvailability,ControlName};
    let mut app=App::headless(200,100);
    app.with_default_widgets().with_default_systems();mirui_kit::install(&mut app);
    let actions=app.add_model(Actions::default()).finish();
    let state=app.add_model(ButtonState::default()).finish();
    let root=app.spawn_root().id();app.compose(root,configured_button(actions,state.clone()));app.tick();
    let node=app.world.query::<ButtonAvailability>().iter().next().unwrap().0;
    assert_eq!(app.world.get::<ControlName>(node).unwrap().label,"Run");
    state.set_pending(true);app.tick();
    assert!(app.world.has::<InputBlocked>(node));
    assert!(matches!(app.world.get::<UserState>(node),Some(UserState::Disabled)));
    state.disable(true);state.set_pending(false);app.tick();
    assert!(app.world.has::<InputBlocked>(node));
    state.disable(false);app.tick();
    assert!(!app.world.has::<InputBlocked>(node));
    assert!(app.world.get::<UserState>(node).is_none());
}

#[compose]
fn slotted_panel()->Entity {
    use mirui_kit::layout::Panel;
    ui! {
        Panel (id:"slotted_panel",width:200,height:160,header_height:20,footer_height:20) {
            @header {View (id:"panel_heading",width:100,height:20)}
            @body {View (id:"panel_body",width:100,height:30)}
            @footer {View (id:"panel_footer",width:100,height:20)}
        }
    }
}
#[test]
fn native_named_slots_keep_consumer_content_in_the_active_tree() {
    let mut app=App::headless(200,160);
    app.with_default_widgets().with_default_systems();mirui_kit::install(&mut app);
    let root=app.spawn_root().id();app.compose(root,slotted_panel());app.render().unwrap();
    let heading=app.world.find_by_id("panel_heading").unwrap();
    let body=app.world.find_by_id("panel_body").unwrap();
    let footer=app.world.find_by_id("panel_footer").unwrap();
    let y=|entity|app.world.get::<mirui::ui::ComputedRect>(entity).unwrap().0.y;
    assert!(y(heading)+Fixed::from_int(20)<=y(body));
    assert!(y(body)+Fixed::from_int(30)<=y(footer));
    let mut ancestor=body;
    while let Some(parent)=app.world.get::<mirui::ui::Parent>(ancestor) {ancestor=parent.0;}
    assert_eq!(Some(ancestor),app.root);
}

mod support;
#[test]
fn button_pointer_keyboard_and_pending_use_native_event_routing() {
    use mirui_kit::button::{ButtonState,ButtonAvailability};
    let mut app=support::app();
    let actions=app.add_model(Actions::default()).finish();
    let state=app.add_model(ButtonState::default()).finish();
    let root=app.spawn_root().id();app.compose(root,configured_button(actions.clone(),state.clone()));app.tick();app.render().unwrap();
    let node=app.world.query::<ButtonAvailability>().iter().next().unwrap().0;
    support::tap(&mut app,node);
    assert_eq!(actions.take(),[(9,false)]);
    app.world.resource_mut::<mirui::input::event::focus::FocusState>().unwrap().focused=Some(node);
    app.backend.events.push_back(mirui::surface::InputEvent::Key{physical:Some(PhysicalKey::Space),code:32,pressed:true,modifiers:Some(Default::default()),repeat:Some(false)});
    app.tick();assert_eq!(actions.take(),[(9,false)]);
    state.set_pending(true);app.tick();app.render().unwrap();
    support::tap(&mut app,node);
    assert!(actions.take().is_empty());
}
#[compose]
fn options_fixture(actions:model!(Actions),list:model!(mirui_kit::choices::ChoiceList))->Entity {
    use mirui_kit::choices::list_box;
    ui! {View (width:240,height:200) {list_box(actions,list,6,false)}}
}
#[test]
fn disabled_options_skip_real_pointer_activation_and_navigation_selects_on_space() {
    use mirui_kit::{choices::{ChoiceList,ChoiceOption},behavior::{choices::OptionItem,collection::SelectionMode}};
    let mut app=support::app();
    let actions=app.add_model(Actions::default()).finish();
    let list=app.add_model(ChoiceList::new(vec![OptionItem{id:1,label:"One".into(),disabled:false},OptionItem{id:2,label:"Unavailable".into(),disabled:true},OptionItem{id:3,label:"Three".into(),disabled:false}],SelectionMode::Multiple).unwrap()).finish();
    let root=app.spawn_root().id();app.compose(root,options_fixture(actions,list.clone()));app.tick();app.render().unwrap();
    let row=|app:&support::TestApp,id|app.world.query::<ChoiceOption>().iter().find(|(_,option)|option.item==id).unwrap().0;
    let first=row(&app,1);support::tap(&mut app,first);assert_eq!(list.selected_ids(),[1]);
    let disabled=row(&app,2);support::tap(&mut app,disabled);assert_eq!(list.selected_ids(),[1]);
    support::tap(&mut app,first);
    for physical in [PhysicalKey::ArrowDown,PhysicalKey::Space] {
        app.backend.events.push_back(mirui::surface::InputEvent::Key{physical:Some(physical),code:0,pressed:true,modifiers:Some(Default::default()),repeat:Some(false)});
        app.tick();
    }
    assert!(list.focused(3));assert_eq!(list.selected_ids(),[1,3]);
}

#[compose]
fn conditional_focus(toggle:model!(mirui_kit::Toggle))->Entity {
    use mirui_kit::FocusItem;
    ui! {Column (width:200,height:160) {
        View (id:"always_focus",width:200,height:40) [Focusable,FocusItem{overlay:false,order:1}]
        if ${toggle.checked()} {
            View (id:"conditional_focus",width:200,height:40) [Focusable,FocusItem{overlay:false,order:2}]
        } else {
            View (id:"alternate_focus",width:200,height:40) [Focusable,FocusItem{overlay:false,order:3}]
        }
    }}
}
#[test]
fn tab_skips_retained_but_unselected_native_conditional_branches() {
    let mut app=support::app();
    let actions=app.add_model(Actions::default()).finish();
    let toggle=app.add_model(mirui_kit::Toggle::default()).finish();
    mirui_kit::bind_actions(&mut app,actions);
    let root=app.spawn_root().id();app.compose(root,conditional_focus(toggle.clone()));app.tick();app.render().unwrap();
    let always=app.world.find_by_id("always_focus").unwrap();
    let alternate=app.world.find_by_id("alternate_focus").unwrap();
    let conditional=app.world.find_by_id("conditional_focus").unwrap();
    assert_eq!(app.world.resource::<mirui::input::event::focus::FocusState>().unwrap().focused,Some(always));
    let tab=mirui::surface::InputEvent::Key{physical:Some(PhysicalKey::Tab),code:9,pressed:true,modifiers:Some(Default::default()),repeat:Some(false)};
    app.backend.events.push_back(tab.clone());app.tick();
    assert_eq!(app.world.resource::<mirui::input::event::focus::FocusState>().unwrap().focused,Some(alternate));
    toggle.set(true);app.tick();app.render().unwrap();
    app.world.resource_mut::<mirui::input::event::focus::FocusState>().unwrap().focused=Some(always);
    app.backend.events.push_back(tab);app.tick();
    assert_eq!(app.world.resource::<mirui::input::event::focus::FocusState>().unwrap().focused,Some(conditional));
}

#[compose]
fn fitted_sections()->Entity {
    ui! {Column (id:"fitted_sections",width:200,row_gap:8,padding:Padding::all(4)) [mirui_kit::sizing::ContentFit] {
        Column (id:"first_section",width:200,row_gap:4) {
            View (width:100,height:20)
            View (width:100,height:30)
        }
        View (id:"following_section",width:100,height:40)
    }}
}
#[test]
fn content_sizing_reserves_nested_height_and_idle_frames_do_not_drift() {
    let mut app=support::app();let root=app.spawn_root().id();app.compose(root,fitted_sections());
    for _ in 0..3 {app.tick();app.render().unwrap();}
    let fitted=app.world.find_by_id("fitted_sections").unwrap();
    let first=app.world.find_by_id("first_section").unwrap();
    let following=app.world.find_by_id("following_section").unwrap();
    let rect=|app:&support::TestApp,entity|app.world.get::<mirui::ui::ComputedRect>(entity).unwrap().0;
    assert_eq!(rect(&app,first).h,Fixed::from_int(54));
    assert!(rect(&app,following).y>=rect(&app,first).y+rect(&app,first).h+Fixed::from_int(8));
    let before=rect(&app,fitted);
    for _ in 0..20 {app.tick();app.render().unwrap();}
    assert_eq!(rect(&app,fitted),before);
    assert!(!app.world.has::<mirui_kit::sizing::ContentFitIssue>(fitted));
}

#[compose]
fn calendar_fixture(actions:model!(Actions),state:model!(mirui_kit::calendar::Calendar))->Entity {
    use mirui_kit::calendar::calendar;
    ui! {calendar(actions,state,10,false)}
}
#[test]
fn calendar_pointer_and_keyboard_commit_real_dates_and_respect_disabled_state() {
    use mirui_kit::{calendar::{Calendar,CalendarNode,CalendarPart},behavior::{forms::Date,calendar::{CalendarMode,WeekStart}}};
    let mut app=support::app_with_size(360,400);
    let actions=app.add_model(Actions::default()).finish();
    let first=Date::new(2024,2,1).unwrap();
    let calendar=app.add_model(Calendar::new(first,first,Date::new(2024,2,29).unwrap(),CalendarMode::Single,WeekStart::Monday).unwrap()).finish();
    let root=app.spawn_root().id();app.compose(root,calendar_fixture(actions,calendar.clone()));app.tick();app.render().unwrap();
    let node=app.world.query::<CalendarNode>().iter().find(|(_,node)|matches!(node.part,CalendarPart::Cell(3))).unwrap().0;
    support::tap(&mut app,node);
    assert_eq!(calendar.selection(),Some((first,first)));
    for physical in [PhysicalKey::ArrowRight,PhysicalKey::Space] {
        app.backend.events.push_back(mirui::surface::InputEvent::Key{physical:Some(physical),code:0,pressed:true,modifiers:Some(Default::default()),repeat:Some(false)});app.tick();
    }
    let second=Date::new(2024,2,2).unwrap();assert_eq!(calendar.selection(),Some((second,second)));
    calendar.set_disabled(true);app.tick();app.render().unwrap();support::tap(&mut app,node);
    assert_eq!(calendar.selection(),Some((second,second)));
}

#[test]
fn shared_action_channel_reports_rejection_without_reordering_or_overwriting() {
    use mirui_kit::widgets::ActionError;
    let mut app=App::headless(100,100);let actions=app.add_model(Actions::default()).finish();
    assert_eq!(actions.activate(90,true),Err(ActionError::WrongLayer));
    for id in 0..32 {actions.activate(id,false).unwrap();}
    assert_eq!(actions.activate(99,false),Err(ActionError::Backpressured));
    assert_eq!(actions.take(),(0..32).map(|id|(id,false)).collect::<Vec<_>>());
    actions.activate(99,false).unwrap();assert_eq!(actions.take(),[(99,false)]);
}

#[compose]
fn changing_content(actions:model!(Actions),toggle:model!(mirui_kit::Toggle))->Entity {
    use mirui::ui::widgets::{Text,ParagraphStyle};
    ui! {Column (id:"changing_fit",width:180,row_gap:8) [mirui_kit::sizing::ContentFit] {
        Text (id:"scaled_body","A readable paragraph wraps at its container and grows with text scale.",width:Dimension::percent(100),paragraph:ParagraphStyle::default(),font:mirui_kit::font::MEDIUM,font_size:${mirui_kit::widgets::text_size(actions.text_scale(),14)})
        View (id:"conditional_extent",visible:${toggle.checked()},width:100,height:40)
        View (id:"fit_tail",width:100,height:20)
    }}
}
#[test]
fn native_content_sizing_tracks_visibility_wrapping_and_text_scaling() {
    let mut app=support::app_with_size(240,400);
    let actions=app.add_model(Actions::default()).finish();
    let toggle=app.add_model(mirui_kit::Toggle::default()).finish();
    let root=app.spawn_root().id();app.compose(root,changing_content(actions.clone(),toggle.clone()));
    for _ in 0..4 {app.tick();app.render().unwrap();}
    let fitted=app.world.find_by_id("changing_fit").unwrap();
    let body=app.world.find_by_id("scaled_body").unwrap();
    let height=|app:&support::TestApp,node|app.world.get::<mirui::ui::ComputedRect>(node).unwrap().0.h;
    let before=height(&app,fitted);
    assert!(height(&app,body)>Fixed::from_int(20),"body should actually wrap");
    toggle.set(true);for _ in 0..4 {app.tick();app.render().unwrap();}
    assert_eq!(height(&app,fitted),before+Fixed::from_int(48));
    toggle.set(false);for _ in 0..4 {app.tick();app.render().unwrap();}
    assert_eq!(height(&app,fitted),before);
    let small=height(&app,body);actions.set_scale(150);
    for _ in 0..4 {app.tick();app.render().unwrap();}
    assert!(height(&app,body)>small);
    assert!(height(&app,fitted)>before);
    assert!(!app.world.has::<mirui_kit::sizing::ContentFitIssue>(fitted));
}

#[compose]
fn table_fixture(actions:model!(Actions),table:model!(mirui_kit::table::DataTable))->Entity {
    use mirui_kit::table::data_table;
    ui! {data_table(actions,table,40,false)}
}
#[test]
fn native_table_aligns_cells_and_updates_retained_rows_and_column_widths() {
    use mirui_kit::{table::DataTable,behavior::{collection::SelectionMode,table::{Column,TableRow,Cell}}};
    use mirui::ui::widgets::Text;
    let mut app=support::app_with_size(600,400);
    let actions=app.add_model(Actions::default()).finish();
    let table=app.add_model(DataTable::new(vec![
        Column{id:1,label:"Title".into(),width:240,minimum:100,maximum:400,sortable:true},
        Column{id:2,label:"Size".into(),width:120,minimum:80,maximum:200,sortable:true},
    ],vec![TableRow{id:10,cells:vec![Cell::Text("alpha".into()),Cell::Integer(11)],disabled:false}],SelectionMode::Single).unwrap()).finish();
    let root=app.spawn_root().id();app.compose(root,table_fixture(actions,table.clone()));
    for _ in 0..4 {app.tick();app.render().unwrap();}
    let text_node=|app:&support::TestApp,label:&str|app.world.query::<Text>().iter().find(|(_,text)|text.resolve(None)==label).unwrap().0;
    let rect=|app:&support::TestApp,node|app.world.get::<mirui::ui::ComputedRect>(node).unwrap().0;
    let title=text_node(&app,"Title");let size=text_node(&app,"Size");
    let alpha=text_node(&app,"alpha");let number=text_node(&app,"11");
    assert_eq!(rect(&app,title).x,rect(&app,alpha).x);
    assert_eq!(rect(&app,size).x,rect(&app,number).x);
    let previous=rect(&app,number).x;
    table.resize(1,300);
    table.replace_rows(vec![TableRow{id:10,cells:vec![Cell::Text("updated".into()),Cell::Integer(22)],disabled:false}]).unwrap();
    for _ in 0..4 {app.tick();app.render().unwrap();}
    assert_eq!(text_node(&app,"updated"),alpha,"stable row IDs retain the mounted cell");
    assert_eq!(text_node(&app,"22"),number);
    assert_eq!(rect(&app,number).x,previous+Fixed::from_int(60));
    assert_eq!(rect(&app,size).x,rect(&app,number).x);
}

#[compose]
fn split_fixture(actions:model!(Actions),state:model!(mirui_kit::split::SplitState))->Entity {
    use mirui_kit::split::SplitPane;
    ui! {View (id:"split_viewport",width:500,height:160) {
        SplitPane (width:Dimension::percent(100),height:Dimension::percent(100),state:Some(state.clone()),actions:Some(actions.clone()),key:50u16,overlay:false) {
            @first {View (id:"split_first_content",width:Dimension::percent(100),height:Dimension::percent(100))}
            @second {View (id:"split_second_content",width:Dimension::percent(100),height:Dimension::percent(100))}
        }
    }}
}
#[test]
fn native_split_panes_preserve_slots_bound_geometry_and_accept_keyboard_and_drag() {
    use mirui_kit::split::{SplitState,Axis,SplitDivider,SplitDrag};
    use mirui::surface::{InputEvent,PointerKind};
    let mut app=support::app_with_size(600,400);
    let actions=app.add_model(Actions::default()).finish();
    let state=app.add_model(SplitState::new(Axis::Horizontal,200,100,120)).finish();
    let root=app.spawn_root().id();app.compose(root,split_fixture(actions.clone(),state.clone()));
    for _ in 0..4 {app.tick();app.render().unwrap();}
    let first=app.world.find_by_id("split_first_content").unwrap();
    let second=app.world.find_by_id("split_second_content").unwrap();
    let divider=app.world.query::<SplitDivider>().iter().next().unwrap().0;
    let rect=|app:&support::TestApp,node|app.world.get::<mirui::ui::ComputedRect>(node).unwrap().0;
    assert_eq!(rect(&app,first).w,Fixed::from_int(200));
    assert_eq!(rect(&app,first).w+rect(&app,divider).w+rect(&app,second).w,Fixed::from_int(500));
    support::tap(&mut app,divider);
    let key=|physical|InputEvent::Key{physical:Some(physical),code:0,pressed:true,modifiers:Some(Default::default()),repeat:Some(false)};
    app.backend.events.push_back(key(PhysicalKey::ArrowRight));app.tick();app.render().unwrap();
    assert_eq!(state.sizes().first,208);
    let r=rect(&app,divider);let x=r.x+r.w/2;let y=r.y+r.h/2;
    app.backend.events.push_back(InputEvent::PointerDown{kind:PointerKind::Mouse,id:0,x,y});app.tick();
    for delta in [20,40,60] {
        app.backend.events.push_back(InputEvent::PointerMove{kind:PointerKind::Mouse,id:0,x:x+Fixed::from_int(delta),y});app.tick();
    }
    app.backend.events.push_back(InputEvent::PointerUp{kind:PointerKind::Mouse,id:0,x:x+Fixed::from_int(60),y});app.tick();app.render().unwrap();
    assert!(state.sizes().first>208);
    assert!(!app.world.has::<SplitDrag>(divider));
    app.backend.events.push_back(key(PhysicalKey::End));app.tick();app.render().unwrap();
    assert_eq!(state.sizes().second,120);
    assert_eq!(app.world.find_by_id("split_first_content"),Some(first));
    state.set_disabled(true);app.tick();app.render().unwrap();
    assert!(app.world.has::<InputBlocked>(divider));
    let before=state.sizes().first;app.backend.events.push_back(key(PhysicalKey::Home));app.tick();
    assert_eq!(state.sizes().first,before);
    state.set_disabled(false);actions.set_modal(true);app.tick();app.render().unwrap();
    assert!(app.world.has::<InputBlocked>(divider),"modal layer blocks the background divider");
    app.backend.events.push_back(key(PhysicalKey::Home));app.tick();
    assert_eq!(state.sizes().first,before);
    actions.set_modal(false);app.tick();app.render().unwrap();
    assert!(!app.world.has::<InputBlocked>(divider));
    let viewport=app.world.find_by_id("split_viewport").unwrap();
    mirui::ui::property::apply_to_world::<mirui::ui::property::prop::Width>(&mut app.world,viewport,Dimension::px(200));
    for _ in 0..4 {app.tick();app.render().unwrap();}
    assert!(state.sizes().constrained);
    assert_eq!(rect(&app,first).w+rect(&app,divider).w+rect(&app,second).w,Fixed::from_int(200));
    assert_eq!(app.world.find_by_id("split_second_content"),Some(second));
}
#[test]
fn vertical_split_with_impossible_minima_stays_inside_the_viewport() {
    use mirui_kit::split::{SplitState,Axis,SplitDivider};
    let mut app=support::app_with_size(600,400);
    let actions=app.add_model(Actions::default()).finish();
    let state=app.add_model(SplitState::new(Axis::Vertical,200,200,300)).finish();
    let root=app.spawn_root().id();app.compose(root,split_fixture(actions.clone(),state.clone()));
    for _ in 0..4 {app.tick();app.render().unwrap();}
    let height=|node|app.world.get::<mirui::ui::ComputedRect>(node).unwrap().0.h;
    let first=app.world.find_by_id("split_first_content").unwrap();
    let second=app.world.find_by_id("split_second_content").unwrap();
    let divider=app.world.query::<SplitDivider>().iter().next().unwrap().0;
    assert!(state.sizes().constrained);
    assert_eq!(height(first)+height(divider)+height(second),Fixed::from_int(160));
    assert!(height(first)>Fixed::ZERO && height(second)>Fixed::ZERO);
}

#[test]
fn desktop_defaults_are_compact_without_shrinking_type() {
    use mirui_kit::{button::ButtonSpec,style::Density};
    assert_eq!(Density::default(),Density::Compact);
    assert_eq!(ButtonSpec::labelled("Open").density,Density::Compact);
    assert_eq!(Density::Compact.control_height(),32);
    assert_eq!(Density::Touch.control_height(),48);
    assert_eq!(mirui_kit::widgets::text_size(100,mirui_kit::font::BODY_SIZE),mirui_kit::font::BODY_SIZE);
    let mut app=App::headless(100,100);
    let actions=app.add_model(Actions::default()).finish();
    actions.focus(7,true);
    assert!(!actions.keyboard_focus_on(7),"a remembered selection/focus must not paint a keyboard ring at rest");
    actions.set_focus_visible(true);
    assert!(actions.keyboard_focus_on(7));
    assert!(!actions.keyboard_focus_on(8));
    actions.set_focus_visible(false);
    assert!(!actions.keyboard_focus_on(7));
    assert_eq!(actions.focused(),Some(7),"hiding a ring must not discard focus");
}
#[test]
fn desktop_density_is_explicit_and_does_not_override_text_scale() {
    use mirui_kit::style::Density;
    let mut app=App::headless(100,100);
    let actions=app.add_model(Actions::default()).finish();
    assert_eq!((actions.control_height(),actions.field_height()),(32,36));
    actions.set_scale(150);
    actions.set_density(Density::Comfortable);
    assert_eq!((actions.control_height(),actions.field_height()),(40,44));
    assert_eq!(actions.text_scale(),150);
    actions.set_density(Density::Touch);
    assert_eq!(actions.control_height(),48);
}

#[compose]
fn fitted_wrapped_actions()->Entity {
    ui! {Column (id:"wrapped_root",width:200,row_gap:8) [mirui_kit::sizing::ContentFit] {
        Row (id:"wrapped_actions",width:Dimension::percent(100),wrap:FlexWrap::Wrap,column_gap:8,row_gap:8) {
            View (width:120,height:32)
            View (id:"wrapped_second",width:120,height:32)
        }
        View (id:"after_wrapped_actions",width:200,height:20)
    }}
}
#[test]
fn content_sizing_keeps_native_wrapped_lines_and_following_sections_separate() {
    let mut app=support::app();let root=app.spawn_root().id();app.compose(root,fitted_wrapped_actions());
    for _ in 0..8 {app.tick();app.render().unwrap();}
    let rect=|name:&str| {
        let node=app.world.find_by_id(name).unwrap();
        app.world.get::<mirui::ui::ComputedRect>(node).unwrap().0
    };
    let row=rect("wrapped_actions");let second=rect("wrapped_second");let after=rect("after_wrapped_actions");
    assert_eq!(row.h,Fixed::from_int(72));
    assert_eq!(second.y,row.y+Fixed::from_int(40));
    assert!(after.y>=second.y+second.h+Fixed::from_int(8));
    let before=rect("wrapped_root");
    for _ in 0..20 {app.tick();app.render().unwrap();}
    let root=app.world.find_by_id("wrapped_root").unwrap();
    assert_eq!(app.world.get::<mirui::ui::ComputedRect>(root).unwrap().0,before,"wrapped content drifts on idle frames");
}
