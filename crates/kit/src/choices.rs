//! Shared listbox view. Picker popups compose the same model at an overlay root;
//! they must not nest a popup under an input-blocked background subtree.
use alloc::{string::String,vec::Vec};
use mirui::prelude::*;
use mirui::ui::widgets::{Button,Text};
use crate::{Actions,FocusItem,font,style,theme,widgets::{FocusRing,text_size}};
use crate::behavior::{choices::{Choices,OptionItem},collection::{ItemId,Modifiers,SelectionMode},group::Move};
#[model]
pub struct ChoiceList {state:Choices,#[observe]pub revision:u64}
impl ChoiceList {pub fn new(items:Vec<OptionItem>,mode:SelectionMode)->Result<Self,ItemId> {Ok(Self{state:Choices::new(items,mode)?,revision:0})}}
#[model]
impl ChoiceList {
    pub fn rows(&self)->Vec<OptionItem> {self.state.rows()}
    pub fn selected(&self,id:ItemId)->bool {self.state.selection.contains(id)}
    pub fn focused(&self,id:ItemId)->bool {self.state.selection.focus()==Some(id)}
    pub fn selected_ids(&self)->Vec<ItemId> {self.state.selection.selected().iter().copied().collect()}
    pub fn filter(&mut self,query:String) {self.state.filter(query);self.revision=self.revision.wrapping_add(1);}
    pub fn choose(&mut self,id:ItemId,modifiers:Modifiers) {self.state.choose(id,modifiers);self.revision=self.revision.wrapping_add(1);}
    pub fn navigate(&mut self,movement:Move,modifiers:Modifiers) {self.state.navigate(movement,modifiers);self.revision=self.revision.wrapping_add(1);}
    pub fn typeahead(&mut self,prefix:&str) {self.state.typeahead(prefix);self.revision=self.revision.wrapping_add(1);}
    pub fn toggle_focused(&mut self) {if let Some(id)=self.state.selection.focus() {self.state.choose(id,Modifiers{ctrl:true,shift:false});self.revision=self.revision.wrapping_add(1);}}
    pub fn select_all(&mut self) {self.state.select_all();self.revision=self.revision.wrapping_add(1);}
    pub fn replace(&mut self,items:Vec<OptionItem>)->Result<(),ItemId> {self.state.replace(items)?;self.revision=self.revision.wrapping_add(1);Ok(())}
}
#[compose]
pub fn list_box(actions:model!(Actions),list:model!(ChoiceList),key:u16,overlay:bool)->Entity {
    ui! {
        Column (width:Dimension::percent(100),row_gap:2,border_radius:style::RADIUS,border_width:2,border_color:theme::CLEAR) [Focusable,FocusItem{overlay,order:key},FocusRing{actions:actions.clone()}] {
            walk ${ {let _=list.revision();list.rows()} } with option by option.id {
                option_row (actions,list,option,overlay)
            }
            Text ("No matching options",visible:${ {let _=list.revision();list.rows().is_empty()} },font:font::MEDIUM,font_size:font::BODY_SIZE,text_color:ColorToken::OnSurfaceVariant)
        } on Key(|event:event!(Key)| {
            if !event.pressed || actions.modal()!=overlay {return false;}
            let modifiers=Modifiers{ctrl:event.modifiers.is_some_and(|m|m.control),shift:event.modifiers.is_some_and(|m|m.shift)};
            let movement=match event.physical {Some(PhysicalKey::ArrowUp)=>Some(Move::Previous),Some(PhysicalKey::ArrowDown)=>Some(Move::Next),Some(PhysicalKey::Home)=>Some(Move::First),Some(PhysicalKey::End)=>Some(Move::Last),_=>None};
            if let Some(movement)=movement {list.navigate(movement,modifiers);return true;}
            if event.physical==Some(PhysicalKey::Space) && event.repeat!=Some(true) {list.toggle_focused();return true;}
            if event.physical==Some(PhysicalKey::KeyA) && modifiers.ctrl {list.select_all();return true;}
            false
        }) on FocusChanged(|event:event!(FocusChanged)| {actions.focus(key,event.focused);})
    }
}
#[compose]
fn option_row(actions:model!(Actions),list:model!(ChoiceList),option:OptionItem,overlay:bool)->Entity {
    ui! {
        Button (width:Dimension::percent(100),height:${text_size(actions.text_scale(),40)},padding:Padding::all(8),border_radius:style::RADIUS,border_width:2,border_color:${ {let _=list.revision();if list.focused(option.id) {theme::FOCUS} else {theme::CLEAR}} },normal_color:${ {let _=list.revision();if list.selected(option.id) {theme::SELECTION} else {theme::CLEAR}} },pressed_color:theme::PRESSED) {
            Text (text:option.label.clone(),font:font::MEDIUM,font_size:${text_size(actions.text_scale(),font::BODY_SIZE)},text_color:if option.disabled {ColorToken::OnSurfaceVariant} else {ColorToken::OnSurface})
        } on Tap {if actions.modal()==overlay {list.choose(option.id,Modifiers::default());}}
    }
}
