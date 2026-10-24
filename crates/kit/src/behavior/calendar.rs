//! Gregorian calendar selection. Host date formatting/time zones stay outside
//! this date-only policy; no clock or platform I/O is assumed.
use super::forms::Date;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum CalendarMode {Single,Range}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum WeekStart {Monday,Sunday}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum CalendarMove {Days(i32),Months(i32),WeekStart,WeekEnd}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum CalendarError {InvalidBounds,Unavailable}
#[derive(Clone,Debug)]
pub struct CalendarState {
    pub focused:Date,
    pub minimum:Date,
    pub maximum:Date,
    pub mode:CalendarMode,
    pub week_start:WeekStart,
    pub disabled:bool,
    start:Option<Date>,
    end:Option<Date>,
    pending_end:bool,
}
impl CalendarState {
    pub fn new(focused:Date,minimum:Date,maximum:Date,mode:CalendarMode,week_start:WeekStart)->Result<Self,CalendarError> {
        if minimum>maximum || focused<minimum || focused>maximum {return Err(CalendarError::InvalidBounds);}
        Ok(Self{focused,minimum,maximum,mode,week_start,disabled:false,start:None,end:None,pending_end:false})
    }
    /// Monday is zero. Year one started on Monday in the proleptic Gregorian
    /// calendar. Wide arithmetic avoids overflow across the supported range.
    pub fn weekday(date:Date)->u8 {
        let (year,month,day)=date.parts();let y=year as u32-1;
        let mut days=365*y+y/4-y/100+y/400+day as u32-1;
        for m in 1..month {days+=Date::days_in_month(year,m) as u32;}
        (days%7) as u8
    }
    fn week_offset(&self,date:Date)->u8 {(Self::weekday(date)+if self.week_start==WeekStart::Sunday {1} else {0})%7}
    pub fn cell(&self,index:u8)->Option<Date> {
        if index>=42 {return None;}
        let (y,m,_)=self.focused.parts();let first=Date::new(y,m,1)?;
        let day=index as i32-self.week_offset(first) as i32+1;
        if day<1 {return None;}
        Date::new(y,m,u8::try_from(day).ok()?)
    }
    pub fn available(&self,date:Date)->bool {!self.disabled && date>=self.minimum && date<=self.maximum}
    pub fn selection(&self)->Option<(Date,Date)> {Some((self.start?,self.end?))}
    pub fn range_pending(&self)->bool {self.pending_end}
    pub fn selected(&self,date:Date)->bool {
        self.start.is_some_and(|start|if let Some(end)=self.end {date>=start && date<=end} else {date==start})
    }
    pub fn choose(&mut self,date:Date)->Result<(),CalendarError> {
        if !self.available(date) {return Err(CalendarError::Unavailable);}
        self.focused=date;
        if self.mode==CalendarMode::Single {self.start=Some(date);self.end=Some(date);self.pending_end=false;}
        else if self.pending_end {
            let start=self.start.expect("pending range has an anchor");
            self.start=Some(start.min(date));self.end=Some(start.max(date));self.pending_end=false;
        } else {self.start=Some(date);self.end=None;self.pending_end=true;}
        Ok(())
    }
    pub fn move_focus(&mut self,movement:CalendarMove) {
        if self.disabled {return;}
        let candidate=match movement {
            CalendarMove::Days(n)=>self.focused.add_days(n),
            CalendarMove::Months(n)=>self.focused.add_months(n),
            CalendarMove::WeekStart=>self.focused.add_days(-(self.week_offset(self.focused) as i32)),
            CalendarMove::WeekEnd=>self.focused.add_days(6-self.week_offset(self.focused) as i32),
        };
        if let Some(date)=candidate {self.focused=date.clamp(self.minimum,self.maximum);}
    }
    pub fn clear(&mut self) {if !self.disabled {self.start=None;self.end=None;self.pending_end=false;}}
}
#[cfg(test)]mod tests {
    use super::*;
    fn date(y:u16,m:u8,d:u8)->Date {Date::new(y,m,d).unwrap()}
    fn calendar(mode:CalendarMode)->CalendarState {CalendarState::new(date(2024,2,29),date(2024,1,1),date(2025,12,31),mode,WeekStart::Monday).unwrap()}
    #[test]fn weekday_and_month_cells_follow_requested_week_start() {let mut c=calendar(CalendarMode::Single);assert_eq!(CalendarState::weekday(date(1,1,1)),0);assert_eq!(CalendarState::weekday(date(2026,10,24)),5);assert_eq!(c.cell(3),Some(date(2024,2,1)));assert_eq!(c.cell(2),None);c.week_start=WeekStart::Sunday;assert_eq!(c.cell(4),Some(date(2024,2,1)));assert_eq!(c.cell(41),None);}
    #[test]fn leap_month_clamps_without_changing_selection() {let mut c=calendar(CalendarMode::Single);c.choose(c.focused).unwrap();c.move_focus(CalendarMove::Months(12));assert_eq!(c.focused,date(2025,2,28));assert_eq!(c.selection(),Some((date(2024,2,29),date(2024,2,29))));}
    #[test]fn range_can_be_selected_in_reverse_and_restarted() {let mut c=calendar(CalendarMode::Range);c.choose(date(2024,3,8)).unwrap();assert!(c.range_pending());assert_eq!(c.selection(),None);c.choose(date(2024,3,2)).unwrap();assert_eq!(c.selection(),Some((date(2024,3,2),date(2024,3,8))));assert!(c.selected(date(2024,3,5)));c.choose(date(2024,4,1)).unwrap();assert!(c.range_pending());assert!(!c.selected(date(2024,3,5)));}
    #[test]fn bounds_and_disabled_state_reject_pointer_and_keyboard_changes() {let mut c=calendar(CalendarMode::Single);c.move_focus(CalendarMove::Days(-200));assert_eq!(c.focused,date(2024,1,1));assert_eq!(c.choose(date(2023,1,1)),Err(CalendarError::Unavailable));c.disabled=true;c.move_focus(CalendarMove::Days(1));assert_eq!(c.focused,date(2024,1,1));assert_eq!(c.choose(c.focused),Err(CalendarError::Unavailable));}
    #[test]fn week_edges_and_year_boundaries_are_safe() {let mut c=calendar(CalendarMode::Single);c.move_focus(CalendarMove::WeekStart);assert_eq!(c.focused,date(2024,2,26));c.move_focus(CalendarMove::WeekEnd);assert_eq!(c.focused,date(2024,3,3));let mut edge=CalendarState::new(date(9999,12,31),date(1,1,1),date(9999,12,31),CalendarMode::Single,WeekStart::Monday).unwrap();edge.move_focus(CalendarMove::Days(1));assert_eq!(edge.focused,date(9999,12,31));}
}
