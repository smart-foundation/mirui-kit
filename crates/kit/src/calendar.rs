//! Shared single-date and date-range calendar, with one grid Tab stop and
//! separate previous/next-month stops. Formatting is intentionally ISO/Gregorian;
//! locale adapters and popup DateField ownership are separate contracts.
use alloc::{format,string::{String,ToString},vec::Vec};
use mirui::prelude::*;
use mirui::ui::widgets::{Button,Text};
use crate::{Actions,FocusItem,font,style,theme,widgets::{FocusRing,text_size}};
use crate::behavior::{forms::Date,calendar::{CalendarState,CalendarMode,WeekStart,CalendarMove,CalendarError}};
#[model]
pub struct Calendar {state:CalendarState,#[observe]pub revision:u64}
impl Calendar {
    pub fn new(focused:Date,minimum:Date,maximum:Date,mode:CalendarMode,week_start:WeekStart)->Result<Self,CalendarError> {Ok(Self{state:CalendarState::new(focused,minimum,maximum,mode,week_start)?,revision:0})}
}
#[model]
impl Calendar {
    pub fn selection(&self)->Option<(Date,Date)> {self.state.selection()}
    pub fn selection_label(&self)->String {
        if let Some((start,end))=self.state.selection() {
            let (y,m,d)=start.parts();let (ey,em,ed)=end.parts();
            if start==end {format!("{y:04}-{m:02}-{d:02}")} else {format!("{y:04}-{m:02}-{d:02} to {ey:04}-{em:02}-{ed:02}")}
        } else if self.state.range_pending() {"Choose an end date".into()} else {"No dates selected".into()}
    }
    pub fn focused_date(&self)->Date {self.state.focused}
    pub fn cell(&self,index:u8)->Option<Date> {self.state.cell(index)}
    pub fn cell_label(&self,index:u8)->String {self.state.cell(index).map(|date|date.parts().2.to_string()).unwrap_or_default()}
    pub fn month_label(&self)->String {let (year,month,_)=self.state.focused.parts();format!("{year:04}-{month:02}")}
    pub fn weekday_label(&self,index:u8)->String {let i=(index as usize+if self.state.week_start==WeekStart::Sunday {6} else {0})%7;["Mon","Tue","Wed","Thu","Fri","Sat","Sun"][i].to_string()}
    pub fn disabled(&self)->bool {self.state.disabled}
    pub fn available(&self,index:u8)->bool {self.cell(index).is_some_and(|date|self.state.available(date))}
    pub fn selected(&self,index:u8)->bool {self.cell(index).is_some_and(|date|self.state.selected(date))}
    pub fn focused(&self,index:u8)->bool {self.cell(index)==Some(self.state.focused)}
    pub fn can_page(&self,delta:i32)->bool {if self.state.disabled {return false;}self.state.focused.add_months(delta).is_some_and(|date| {let next=date.clamp(self.state.minimum,self.state.maximum);(next.parts().0,next.parts().1)!=(self.state.focused.parts().0,self.state.focused.parts().1)})}
    pub fn set_disabled(&mut self,disabled:bool) {self.state.disabled=disabled;self.revision=self.revision.wrapping_add(1);}
    pub fn choose_cell(&mut self,index:u8)->Result<(),CalendarError> {let date=self.state.cell(index).ok_or(CalendarError::Unavailable)?;self.state.choose(date)?;self.revision=self.revision.wrapping_add(1);Ok(())}
    pub fn choose_focused(&mut self)->Result<(),CalendarError> {self.state.choose(self.state.focused)?;self.revision=self.revision.wrapping_add(1);Ok(())}
    pub fn navigate(&mut self,movement:CalendarMove) {self.state.move_focus(movement);self.revision=self.revision.wrapping_add(1);}
    pub fn clear(&mut self) {self.state.clear();self.revision=self.revision.wrapping_add(1);}
}
#[derive(Clone,Copy)]
pub enum CalendarPart {Root,Cell(u8),Month(i32)}
#[component]
pub struct CalendarNode {pub calendar:model!(Calendar),pub part:CalendarPart}
#[view(component=CalendarNode,name="KitCalendarAvailability",systems=[sync_availability::system()])]
pub fn availability() {}
#[system]
fn sync_availability(nodes:query!(Entity,CalendarNode,optional crate::availability::InputBlocked),commands:commands!()) {
    for (entity,node,blocked) in nodes {
        let enabled=match node.part {CalendarPart::Root=>!node.calendar.disabled(),CalendarPart::Cell(i)=>node.calendar.available(i),CalendarPart::Month(delta)=>node.calendar.can_page(delta)};
        if enabled==blocked.is_some() {commands.set::<crate::availability::InputAvailability>(entity,enabled);}
    }
}
fn height(scale:u16)->i32 {text_size(scale,40) as i32*7+text_size(scale,24) as i32+5*4+4*style::GAP+4}
#[compose]
pub fn calendar(actions:model!(Actions),calendar:model!(Calendar),key:u16,overlay:bool)->Entity {
    assert!(key<=u16::MAX-2,"calendar needs three consecutive focus keys");
    ui! {
        Column (width:${text_size(actions.text_scale(),320)},height:${height(actions.text_scale())},padding:Padding::all(style::GAP),row_gap:style::GAP,border_width:2,border_radius:style::RADIUS,border_color:ColorToken::Outline,bg_color:ColorToken::Surface) [CalendarNode{calendar:calendar.clone(),part:CalendarPart::Root}] {
            Row (width:Dimension::percent(100),height:${text_size(actions.text_scale(),40)},align:AlignItems::Center,column_gap:style::GAP) {
                month_button (actions,calendar,key,overlay,-1,"Previous")
                Text (text:${ {let _=calendar.revision();calendar.month_label()} },grow:1.0,font:font::SEMIBOLD,font_size:${text_size(actions.text_scale(),font::BODY_SIZE)},text_color:ColorToken::OnSurface)
                month_button (actions,calendar,key+1,overlay,1,"Next")
            }
            Row (width:Dimension::percent(100),height:${text_size(actions.text_scale(),24)},justify:JustifyContent::SpaceBetween) {
                walk (0..7).collect::<Vec<u8>>() with day {
                    Text (text:calendar.weekday_label(day),width:${text_size(actions.text_scale(),40)},font:font::MEDIUM,font_size:${text_size(actions.text_scale(),font::CAPTION_SIZE)},text_color:ColorToken::OnSurfaceVariant)
                }
            }
            Column (width:Dimension::percent(100),grow:1.0,row_gap:4,border_width:0,border_color:theme::CLEAR,border_radius:style::RADIUS) [Focusable,FocusItem{overlay,order:key+2},FocusRing{actions:actions.clone()}] {
                walk (0..6).collect::<Vec<u8>>() with week {calendar_week(actions,calendar,key+2,overlay,week)}
            } on FocusChanged(|event:event!(FocusChanged)| {actions.focus(key+2,event.focused);}) on Key(|event:event!(Key)| {
                if !event.pressed || actions.modal()!=overlay || calendar.disabled() {return false;}
                let year=event.modifiers.is_some_and(|m|m.shift);
                let movement=match event.physical {
                    Some(PhysicalKey::ArrowLeft)=>Some(CalendarMove::Days(-1)),Some(PhysicalKey::ArrowRight)=>Some(CalendarMove::Days(1)),
                    Some(PhysicalKey::ArrowUp)=>Some(CalendarMove::Days(-7)),Some(PhysicalKey::ArrowDown)=>Some(CalendarMove::Days(7)),
                    Some(PhysicalKey::Home)=>Some(CalendarMove::WeekStart),Some(PhysicalKey::End)=>Some(CalendarMove::WeekEnd),
                    Some(PhysicalKey::PageUp)=>Some(CalendarMove::Months(if year {-12} else {-1})),Some(PhysicalKey::PageDown)=>Some(CalendarMove::Months(if year {12} else {1})),_=>None};
                if let Some(movement)=movement {calendar.navigate(movement);return true;}
                if (event.code==13 || event.physical==Some(PhysicalKey::Space)) && event.repeat!=Some(true) {let _=calendar.choose_focused();return true;}false
            })
        }
    }
}
#[compose]
fn month_button(actions:model!(Actions),calendar:model!(Calendar),key:u16,overlay:bool,delta:i32,label:&'static str)->Entity {
    ui! {Button(text:label,width:${text_size(actions.text_scale(),80)},height:${text_size(actions.text_scale(),40)},font:font::MEDIUM,font_size:${text_size(actions.text_scale(),font::BODY_SIZE)},text_color:ColorToken::OnSurface,normal_color:ColorToken::SurfaceVariant,pressed_color:theme::PRESSED,border_radius:style::RADIUS) [Focusable,FocusItem{overlay,order:key},FocusRing{actions:actions.clone()},CalendarNode{calendar:calendar.clone(),part:CalendarPart::Month(delta)}]
        on Tap {if actions.modal()==overlay {calendar.navigate(CalendarMove::Months(delta));}}
        on Key(|event:event!(Key)| {if event.pressed && event.repeat!=Some(true) && (event.code==13 || event.physical==Some(PhysicalKey::Space)) && actions.modal()==overlay {calendar.navigate(CalendarMove::Months(delta));return true;}false})
        on FocusChanged(|event:event!(FocusChanged)| {actions.focus(key,event.focused);})}
}
#[compose]
fn calendar_week(actions:model!(Actions),calendar:model!(Calendar),grid_key:u16,overlay:bool,week:u8)->Entity {
    ui! {Row(width:Dimension::percent(100),height:${text_size(actions.text_scale(),40)},justify:JustifyContent::SpaceBetween) {
        walk (0..7).collect::<Vec<u8>>() with day {calendar_cell(actions,calendar,grid_key,overlay,week*7+day)}
    }}
}
#[compose]
fn calendar_cell(actions:model!(Actions),calendar:model!(Calendar),grid_key:u16,overlay:bool,index:u8)->Entity {
    ui! {View(width:${text_size(actions.text_scale(),40)},height:${text_size(actions.text_scale(),40)}) {
        Button(text:${ {let _=calendar.revision();calendar.cell_label(index)} },width:Dimension::percent(100),height:Dimension::percent(100),visible:${ {let _=calendar.revision();calendar.cell(index).is_some()} },font:font::MEDIUM,font_size:${text_size(actions.text_scale(),font::BODY_SIZE)},text_color:ColorToken::OnSurface,border_radius:style::RADIUS,border_width:2,
            border_color:${ {let _=calendar.revision();if calendar.focused(index) && actions.focused()==Some(grid_key) {theme::FOCUS} else {theme::CLEAR}} },
            normal_color:${ {let _=calendar.revision();if calendar.selected(index) {theme::SELECTION} else {theme::CLEAR}} },pressed_color:theme::PRESSED)
            [CalendarNode{calendar:calendar.clone(),part:CalendarPart::Cell(index)}]
            on Tap {if actions.modal()==overlay {let _=calendar.choose_cell(index);}}
    }}
}
