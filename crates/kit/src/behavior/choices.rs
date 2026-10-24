//! Shared option-list policy for listboxes and picker result collections.
use alloc::{collections::BTreeSet,string::String,vec::Vec};
use super::{collection::{ItemId,Selection,SelectionMode,Modifiers},group::Move};
#[derive(Clone,Debug)]
pub struct OptionItem {pub id:ItemId,pub label:String,pub disabled:bool}
#[derive(Clone,Debug)]
pub struct Choices {items:Vec<OptionItem>,filter:String,pub selection:Selection}
impl Choices {
    pub fn new(items:Vec<OptionItem>,mode:SelectionMode)->Result<Self,ItemId> {
        Self::validate(&items)?;
        let mut state=Self{items,filter:String::new(),selection:Selection::new(mode)};
        state.reconcile();Ok(state)
    }
    fn validate(items:&[OptionItem])->Result<(),ItemId> {
        let mut seen=BTreeSet::new();for item in items {if !seen.insert(item.id) {return Err(item.id)}}Ok(())
    }
    pub fn rows(&self)->Vec<OptionItem> {
        let query=self.filter.to_lowercase();
        self.items.iter().filter(|i|i.label.to_lowercase().contains(&query)).cloned().collect()
    }
    pub fn available(&self)->Vec<ItemId> {self.rows().iter().filter(|i|!i.disabled).map(|i|i.id).collect()}
    pub fn filter(&mut self,query:String) {self.filter=query;self.reconcile();}
    fn reconcile(&mut self) {
        let all:Vec<_>=self.items.iter().filter(|i|!i.disabled).map(|i|i.id).collect();
        self.selection.reconcile(&all);
        let visible=self.available();
        if !self.selection.focus().is_some_and(|id|visible.contains(&id)) {
            if let Some(id)=visible.first() {self.selection.focus_only(*id,&visible);}
        }
    }
    pub fn choose(&mut self,id:ItemId,modifiers:Modifiers) {self.selection.choose(id,&self.available(),modifiers);}
    pub fn navigate(&mut self,movement:Move,modifiers:Modifiers) {
        let ids=self.available();if ids.is_empty() {return}
        let i=self.selection.focus().and_then(|id|ids.iter().position(|v|*v==id));
        let next=match movement {
            Move::First=>0,Move::Last=>ids.len()-1,
            Move::Previous=>i.unwrap_or(0).saturating_sub(1),
            Move::Next=>i.map(|i|(i+1).min(ids.len()-1)).unwrap_or(0),
        };
        if modifiers.shift {self.selection.choose(ids[next],&ids,modifiers);} else {self.selection.focus_only(ids[next],&ids);}
    }
    pub fn select_all(&mut self) {self.selection.select_all(&self.available());}
    pub fn replace(&mut self,items:Vec<OptionItem>)->Result<(),ItemId> {Self::validate(&items)?;self.items=items;self.reconcile();Ok(())}
    pub fn typeahead(&mut self,prefix:&str) {
        if prefix.is_empty() {return}
        let ids=self.available();let query=prefix.to_lowercase();
        let start=self.selection.focus().and_then(|id|ids.iter().position(|v|*v==id)).map(|i|i+1).unwrap_or(0);
        for offset in 0..ids.len() {
            let id=ids[(start+offset)%ids.len()];
            if self.items.iter().any(|i|i.id==id&&i.label.to_lowercase().starts_with(&query)) {self.selection.focus_only(id,&ids);break;}
        }
    }
}
#[cfg(test)]mod tests {
    use super::*;use alloc::vec;
    fn options()->Vec<OptionItem> {vec![OptionItem{id:9,label:"Apricot".into(),disabled:false},OptionItem{id:2,label:"Apple".into(),disabled:true},OptionItem{id:5,label:"Banana".into(),disabled:false}]}
    #[test]fn filter_keeps_selected_ids_and_recovers_visible_focus() {let mut c=Choices::new(options(),SelectionMode::Multiple).unwrap();c.choose(9,Modifiers::default());c.filter("ban".into());assert!(c.selection.contains(9));assert_eq!(c.selection.focus(),Some(5));}
    #[test]fn navigation_skips_unavailable_and_clamps() {let mut c=Choices::new(options(),SelectionMode::Single).unwrap();c.navigate(Move::Next,Modifiers::default());assert_eq!(c.selection.focus(),Some(5));c.navigate(Move::Next,Modifiers::default());assert_eq!(c.selection.focus(),Some(5));assert!(c.selection.selected().is_empty());}
    #[test]fn failed_replace_is_atomic() {let mut c=Choices::new(options(),SelectionMode::Single).unwrap();let mut bad=options();bad[2].id=9;assert_eq!(c.replace(bad),Err(9));assert_eq!(c.rows().len(),3);}
    #[test]fn typeahead_wraps_and_ignores_disabled() {let mut c=Choices::new(options(),SelectionMode::Single).unwrap();c.typeahead("ban");assert_eq!(c.selection.focus(),Some(5));c.typeahead("ap");assert_eq!(c.selection.focus(),Some(9));}
    #[test]fn select_all_uses_only_visible_enabled_items() {let mut c=Choices::new(options(),SelectionMode::Multiple).unwrap();c.filter("ap".into());c.select_all();assert_eq!(c.selection.selected().iter().copied().collect::<Vec<_>>(),[9]);}
}
