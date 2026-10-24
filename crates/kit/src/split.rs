//! Slotted, resizable panes using native layout, gestures and focus.
//! Minima are logical pixels. Impossible minima share the remaining extent
//! proportionally; they never overflow the viewport. No docking is implied.
use crate::{ActionsHandle, FocusItem, style, theme, widgets::FocusRing};
use crate::behavior::layout::{split, Split};
use mirui::prelude::*;
use mirui::{ecs::World,ui::{Children,ComputedRect,Style,property::{Property,PropertyChange}}};

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Axis { Horizontal, Vertical }
#[model]
pub struct SplitState {
    pub axis: Axis,
    pub first_min: u32,
    pub second_min: u32,
    #[observe]
    pub requested: u32,
    #[observe]
    pub available: u32,
    #[observe]
    pub disabled: bool,
}
impl SplitState {
    pub fn new(axis:Axis,requested:u32,first_min:u32,second_min:u32)->Self {
        Self {axis,requested,first_min,second_min,available:0,disabled:false}
    }
}
#[model]
impl SplitState {
    pub fn sizes(&self)->Split {split(self.available,self.requested,self.first_min,self.second_min)}
    pub fn resize(&mut self,requested:u32) {if !self.disabled {self.requested=requested;}}
    pub fn set_available(&mut self,available:u32) {if self.available!=available {self.available=available;}}
    pub fn set_disabled(&mut self,disabled:bool) {self.disabled=disabled;}
    pub fn step(&mut self,delta:i32) {
        let first=self.sizes().first;
        self.resize(if delta<0 {first.saturating_sub(delta.unsigned_abs())} else {first.saturating_add(delta as u32)});
    }
}
#[component]
#[derive(Clone)]
pub struct SplitRoot {pub state:SplitStateHandle}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Part {First,Divider,Second}
#[component]
pub struct SplitPart {pub part:Part}
#[component]
#[derive(Clone)]
pub struct SplitDivider {pub state:SplitStateHandle,pub actions:ActionsHandle,pub overlay:bool}
#[component]
pub struct SplitDrag {pointer:Fixed,first:u32}

// Native slotted compositions keep both application-owned subtrees mounted.
ui!(compose SplitPane (state:SplitStateHandle,actions:ActionsHandle,key:u16,overlay:bool) {
    View (width:Dimension::percent(100),height:Dimension::percent(100)) [SplitRoot{state:state.clone()}] {
        View [SplitPart{part:Part::First}] {@@first}
        View (bg_color:ColorToken::Outline,border_width:style::FOCUS_WIDTH,border_color:theme::CLEAR,border_radius:style::RADIUS) [
            SplitPart{part:Part::Divider},SplitDivider{state:state.clone(),actions:actions.clone(),overlay},
            Focusable,FocusItem{overlay,order:key},FocusRing{actions:actions.clone()}
        ]
        View [SplitPart{part:Part::Second}] {@@second}
    }
});

#[view(component=SplitRoot,name="KitSplitRoot",systems=[measure::system()])]
pub fn split_root() {}
#[system(order=PreRender)]
fn measure(roots:query!(Entity,SplitRoot,ComputedRect),commands:commands!()) {
    for (entity,_,_) in roots {commands.set::<ApplyLayout>(entity,());}
}
struct ApplyLayout;
impl Property for ApplyLayout {
    type Value=();
    fn apply(world:&mut World,entity:Entity,_:())->PropertyChange {
        let Some(root)=world.get::<SplitRoot>(entity).cloned() else {return PropertyChange::Unchanged;};
        let Some(rect)=world.get::<ComputedRect>(entity).map(|r|r.0) else {return PropertyChange::Unchanged;};
        let horizontal=root.state.axis()==Axis::Horizontal;
        let extent=(if horizontal {rect.w} else {rect.h}).max(Fixed::ZERO);
        let divider=Fixed::from_int(style::GAP).min(extent);
        let available=(extent-divider).to_int().max(0) as u32;
        root.state.set_available(available);
        let first=Fixed::from_int(root.state.sizes().first as i32);
        let second=(extent-divider-first).max(Fixed::ZERO);
        let children=world.get::<Children>(entity).map(|c|c.0.clone()).unwrap_or_default();
        let mut changed=false;
        if let Some(style)=world.get_mut::<Style>(entity) {
            let direction=if horizontal {FlexDirection::Row} else {FlexDirection::Column};
            if style.layout.direction!=direction {style.layout.direction=direction;changed=true;}
        }
        for child in children {
            let Some(part)=world.get::<SplitPart>(child).map(|p|p.part) else {continue;};
            let size=match part {Part::First=>first,Part::Divider=>divider,Part::Second=>second};
            if let Some(style)=world.get_mut::<Style>(child) {
                let (width,height)=if horizontal {(Dimension::Px(size),Dimension::percent(100))} else {(Dimension::percent(100),Dimension::Px(size))};
                if style.layout.width!=width || style.layout.height!=height {
                    style.layout.width=width;style.layout.height=height;changed=true;
                }
            }
            if changed {world.insert(child,mirui::ui::dirty::Dirty);}
        }
        if changed {world.insert(entity,mirui::ui::dirty::Dirty);PropertyChange::Layout} else {PropertyChange::Unchanged}
    }
}
#[view(component=SplitDivider,name="KitSplitDivider",attach=attach::callback(),input=(start::callback(),drag::callback(),end::callback(),cancel::callback(),key::callback(),focused::callback()),systems=[availability::system()])]
pub fn split_divider() {}
#[event]
fn attach(entity:entity!(),commands:commands!()) {commands.insert(entity,HitTarget);}
#[system]
fn availability(nodes:query!(Entity,SplitDivider,optional crate::availability::InputBlocked),commands:commands!()) {
    for (entity,node,blocked) in nodes {
        let enabled=!node.state.disabled();
        if enabled==blocked.is_some() {commands.set::<crate::availability::InputAvailability>(entity,enabled);}
    }
}
#[event]
fn start(entity:entity!(),event:event!(DragStart),nodes:query!(SplitDivider),commands:commands!())->bool {
    let Some(node)=nodes.get(entity) else {return false;};
    if node.state.disabled() || node.actions.modal()!=node.overlay {return false;}
    commands.insert(entity,SplitDrag{pointer:if node.state.axis()==Axis::Horizontal {event.x} else {event.y},first:node.state.sizes().first});true
}
#[event]
fn drag(entity:entity!(),event:event!(DragMove),nodes:query!(SplitDivider,SplitDrag))->bool {
    let Some((node,start))=nodes.get(entity) else {return false;};
    if node.state.disabled() || node.actions.modal()!=node.overlay {return true;}
    let pointer=if node.state.axis()==Axis::Horizontal {event.x} else {event.y};
    let delta=(pointer-start.pointer).to_int() as i64;
    node.state.resize((start.first as i64+delta).clamp(0,u32::MAX as i64) as u32);true
}
#[event]
fn end(entity:entity!(),_event:event!(DragEnd),commands:commands!())->bool {commands.remove::<SplitDrag>(entity);true}
#[event]
fn cancel(entity:entity!(),_event:event!(DragCancel),commands:commands!())->bool {commands.remove::<SplitDrag>(entity);true}
#[event]
fn key(entity:entity!(),event:event!(Key),nodes:query!(SplitDivider))->bool {
    let Some(node)=nodes.get(entity) else {return false;};
    if !event.pressed || node.state.disabled() || node.actions.modal()!=node.overlay {return false;}
    let step=if event.modifiers.is_some_and(|m|m.shift) {32} else {8};
    let horizontal=node.state.axis()==Axis::Horizontal;
    match event.physical {
        Some(PhysicalKey::ArrowLeft) if horizontal=>node.state.step(-step),
        Some(PhysicalKey::ArrowRight) if horizontal=>node.state.step(step),
        Some(PhysicalKey::ArrowUp) if !horizontal=>node.state.step(-step),
        Some(PhysicalKey::ArrowDown) if !horizontal=>node.state.step(step),
        Some(PhysicalKey::Home)=>node.state.resize(0),
        Some(PhysicalKey::End)=>node.state.resize(u32::MAX),
        _=>return false,
    } true
}
pub fn install<B:Surface,F:RendererFactory<B>>(app:&mut App<B,F>) {
    app.with_widget(SplitPane::__view());
    app.with_widget(split_root::view());app.with_widget(split_divider::view());
}

#[event]
fn focused(entity:entity!(),event:event!(FocusChanged),nodes:query!(SplitDivider,FocusItem)) {
    if let Some((node,item))=nodes.get(entity) {node.actions.focus(item.order,event.focused);}
}
