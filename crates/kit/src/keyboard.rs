//! Native focus requests evaluate the live Mirui tree, including retained
//! conditional branches. Hidden is not the only source of native visibility.
use crate::{Actions, ActionsHandle, FocusItem};
use alloc::vec::Vec;
use mirui::input::event::focus::FocusState;
use mirui::prelude::*;
use mirui::ui::{Parent, UserState, dirty::Dirty,property::{Property,PropertyChange}};

/// Native property application runs after callback query borrows are released.
/// This uses Mirui's supported property extension rather than caching visibility
/// or accessing private conditional-branch components.
struct FocusRequest;
#[derive(Clone)]
pub struct Request {actions:ActionsHandle,reverse:Option<bool>}
fn available(world:&mirui::ecs::World,mut node:Entity)->bool {
    for _ in 0..64 {
        if !world.is_alive(node) || mirui::ui::branch::is_effectively_hidden(world,node)
            || matches!(world.get::<UserState>(node),Some(UserState::Disabled)) {return false;}
        if let Some(parent)=world.get::<Parent>(node) {node=parent.0;} else {return true;}
    }
    false
}
impl Property for FocusRequest {
    type Value=Request;
    fn apply(world:&mut mirui::ecs::World,_anchor:Entity,request:Request)->PropertyChange {
        let actions=request.actions;
        let layer=actions.modal();
        if request.reverse.is_none() && actions.applied_layer()==Some(layer) {return PropertyChange::Unchanged;}
        let mut candidates:Vec<_>=world.query::<FocusItem>().iter()
            .filter(|(entity,item)|item.overlay==layer && available(world,*entity))
            .map(|(entity,item)|(item.order,entity)).collect();
        candidates.sort_by_key(|(order,entity)|(*order,entity.id));
        let Some(previous)=world.resource::<FocusState>().map(|focus|focus.focused) else {return PropertyChange::Unchanged;};
        let next=if let Some(reverse)=request.reverse {
            if candidates.is_empty() {None} else {
                let i=candidates.iter().position(|(_,entity)|Some(*entity)==previous);
                let index=match(i,reverse) {(Some(i),true)=>(i+candidates.len()-1)%candidates.len(),(Some(i),false)=>(i+1)%candidates.len(),(None,true)=>candidates.len()-1,(None,false)=>0};
                Some(candidates[index].1)
            }
        } else {
            let restore=if !layer {actions.saved_focus().filter(|entity|candidates.iter().any(|(_,candidate)|candidate==entity))} else {None};
            actions.accept_layer(previous);
            restore.or_else(||candidates.first().map(|(_,entity)|*entity))
        };
        if previous==next {return PropertyChange::Unchanged;}
        world.resource_mut::<FocusState>().expect("checked focus resource").focused=next;
        for entity in [previous,next].into_iter().flatten() {
            if world.is_alive(entity) {world.insert(entity,Dirty);}
        }
        PropertyChange::Visual
    }
}
pub fn install<B:Surface,F:RendererFactory<B>>(app:&mut App<B,F>,actions:ActionsHandle) {
    app.on_event(Key::EVENT,tab::callback(actions.clone()));
    app.add_system(focus_layer::system(actions));
}
#[event]
fn tab(event:event!(Key),actions:model!(Actions),targets:query!(Entity,FocusItem),commands:commands!())->bool {
    if !event.pressed || event.physical!=Some(PhysicalKey::Tab) {return false;}
    for (entity,_) in targets {
        commands.set::<FocusRequest>(entity,Request{actions:actions.clone(),reverse:Some(event.modifiers.is_some_and(|m|m.shift))});break;
    }
    true
}
#[system]
fn focus_layer(actions:model!(Actions),targets:query!(Entity,FocusItem),commands:commands!()) {
    if actions.applied_layer()==Some(actions.modal()) {return;}
    for (entity,_) in targets {
        commands.set::<FocusRequest>(entity,Request{actions:actions.clone(),reverse:None});break;
    }
}
