//! Shared button, icon-button and toggle composition with native availability.
use alloc::string::String;
use mirui::prelude::*;
use mirui::render::path::Path;
use mirui::ui::widgets::{Button,Icon,Text};
use crate::{Actions,FocusItem,availability::{InputAvailability,InputBlocked},font,style::{self,Density,Intent},theme,widgets::{FocusRing,text_size}};

#[model]
#[derive(Default)]
pub struct ButtonState {
    #[observe] pub disabled:bool,
    #[observe] pub pending:bool,
    #[observe] pub checked:bool,
}
#[model]
impl ButtonState {
    pub fn disable(&mut self,disabled:bool) {self.disabled=disabled;}
    pub fn set_pending(&mut self,pending:bool) {self.pending=pending;}
    pub fn check(&mut self,checked:bool) {self.checked=checked;}
    pub fn toggle(&mut self) {if !self.disabled && !self.pending {self.checked=!self.checked;}}
}
#[derive(Clone)]
pub struct ButtonSpec {
    pub label:String,
    pub icon:Option<&'static Path>,
    pub icon_only:bool,
    pub intent:Intent,
    pub density:Density,
    pub full_width:bool,
    pub toggle:bool,
}
impl ButtonSpec {
    pub fn labelled(label:impl Into<String>)->Self {Self{label:label.into(),icon:None,icon_only:false,intent:Intent::Neutral,density:Density::Comfortable,full_width:false,toggle:false}}
    pub fn icon(label:impl Into<String>,icon:&'static Path)->Self {Self{icon:Some(icon),icon_only:true,..Self::labelled(label)}}
}
/// Public label metadata is retained even when an icon-only control hides its
/// visible label. Native accessibility publication remains a host adapter task.
#[component]
pub struct ControlName {pub label:String}
#[component]
pub struct ButtonAvailability {pub state:model!(ButtonState)}
#[view(component=ButtonAvailability,name="KitButtonAvailability",systems=[sync_availability::system()])]
pub fn availability() {}
#[system]
fn sync_availability(rows:query!(Entity,ButtonAvailability,optional InputBlocked),commands:commands!()) {
    for (entity,control,blocked) in rows {
        let enabled=!control.state.disabled() && !control.state.pending();
        if enabled==blocked.is_some() {commands.set::<InputAvailability>(entity,enabled);}
    }
}
#[compose]
pub fn button(actions:model!(Actions),state:model!(ButtonState),key:u16,spec:ButtonSpec,overlay:bool)->Entity {
    assert!(!spec.label.trim().is_empty(),"buttons require an accessible name");
    assert!(!spec.icon_only || spec.icon.is_some(),"icon buttons require an icon");
    ui! {
        Button (
            font:font::MEDIUM,font_size:${text_size(actions.text_scale(),font::BODY_SIZE)},
            text_color:spec.intent.foreground(),
            width:if spec.full_width {Dimension::percent(100)} else {Dimension::Auto},
            height:${text_size(actions.text_scale(),spec.density.control_height() as u16)},
            min_width:spec.density.control_height(),
            padding:Padding{left:Dimension::px(if spec.icon.is_some() && !spec.icon_only {spec.density.inset()+18+style::GAP} else {spec.density.inset()}),right:Dimension::px(spec.density.inset()),top:Dimension::px(4),bottom:Dimension::px(4)},
            border_width:style::FOCUS_WIDTH,border_color:${if state.checked() {theme::FOCUS} else {ColorToken::Outline}},
            border_radius:style::RADIUS,normal_color:spec.intent.background(),pressed_color:theme::PRESSED
        ) [Text::label(if spec.icon_only {String::new()} else {spec.label.clone()}),Focusable,FocusItem{overlay,order:key},FocusRing{actions:actions.clone()},ButtonAvailability{state:state.clone()},ControlName{label:spec.label.clone()}] {
            if spec.icon.is_some() {
                if spec.icon_only {
                    Icon (path:spec.icon.expect("icon exists").clone(),position:Position::Absolute,
                        left:@width {(width-Fixed::from_int(18))/2},
                        top:${(text_size(actions.text_scale(),spec.density.control_height() as u16) as i32-18)/2},
                        width:18,height:18,color:spec.intent.foreground())
                } else {
                    Icon (path:spec.icon.expect("icon exists").clone(),position:Position::Absolute,
                        left:spec.density.inset(),
                        top:${(text_size(actions.text_scale(),spec.density.control_height() as u16) as i32-18)/2},
                        width:18,height:18,color:spec.intent.foreground())
                }
            }
        } on Tap {
            if actions.modal()==overlay && !state.disabled() && !state.pending() {
                if spec.toggle {state.toggle();} actions.activate(key,overlay);
            }
        } on Key(|event:event!(Key)| {
            if event.pressed && event.repeat!=Some(true) && (event.code==13 || event.physical==Some(PhysicalKey::Space)) {
                if actions.modal()==overlay && !state.disabled() && !state.pending() {
                    if spec.toggle {state.toggle();} actions.activate(key,overlay);
                }
                return true;
            }
            false
        }) on FocusChanged(|event:event!(FocusChanged)| {actions.focus(key,event.focused);})
    }
}
