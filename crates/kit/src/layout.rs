//! Slotted structural compositions built with Mirui's native composition API.
//! These arrange application-owned content, never mount a second input root.
//! ToolbarLayout is geometry, not a command/overflow or keyboard service.
use mirui::prelude::*;
use crate::style;

// Mirui's Auto container axis does not infer children's extents. Supply slot
// extents explicitly rather than let content paint over adjacent sections.
ui!(compose Panel (header_height:i32,footer_height:i32) {
    Column (width:Dimension::percent(100),height:Dimension::percent(100),row_gap:style::GAP,padding:Padding::all(style::INSET),bg_color:ColorToken::Surface,border_color:ColorToken::Outline,border_width:1,border_radius:style::RADIUS) {
        Column (width:Dimension::percent(100),height:header_height,row_gap:style::GAP) {@@header}
        Column (width:Dimension::percent(100),grow:1.0,row_gap:style::GAP) {@@body}
        Row (width:Dimension::percent(100),height:footer_height,column_gap:style::GAP,align:AlignItems::Center) {@@footer}
    }
});
ui!(compose ToolbarLayout {
    Row (width:Dimension::percent(100),height:Dimension::percent(100),min_height:40,padding:Padding::all(style::GAP),column_gap:style::GAP,align:AlignItems::Center,bg_color:ColorToken::SurfaceVariant) {
        Row (column_gap:style::GAP,align:AlignItems::Center) {@@leading}
        Row (grow:1.0,column_gap:style::GAP,align:AlignItems::Center) {@@center}
        Row (column_gap:style::GAP,align:AlignItems::Center) {@@trailing}
    }
});
ui!(compose AppFrame (sidebar_width:i32,header_height:i32,toolbar_height:i32,status_height:i32) {
    Column (width:Dimension::percent(100),height:Dimension::percent(100),bg_color:ColorToken::Surface) {
        Column (width:Dimension::percent(100),height:header_height) {@@header}
        Row (width:Dimension::percent(100),grow:1.0) {
            Column (width:sidebar_width,height:Dimension::percent(100),bg_color:ColorToken::SurfaceVariant) {@@sidebar}
            Column (grow:1.0,height:Dimension::percent(100)) {
                Column (width:Dimension::percent(100),height:toolbar_height) {@@toolbar}
                Column (width:Dimension::percent(100),grow:1.0) {@@content}
                Column (width:Dimension::percent(100),height:status_height) {@@status}
            }
        }
        // Overlay content is a sibling of the potentially blocked background.
        View (position:Position::Absolute,left:0,top:0,width:Dimension::percent(100),height:Dimension::percent(100)) {@@overlays}
    }
});

pub fn install<B:Surface,F:RendererFactory<B>>(app:&mut App<B,F>) {
    app.with_widget(Panel::__view());
    app.with_widget(ToolbarLayout::__view());
    app.with_widget(AppFrame::__view());
    app.with_widget(DesktopFrame::__view());
    app.with_widget(fill_slot::view());
    app.with_widget(chrome_region::view());
}

/// Fill the native Auto host created by a named slot. Use only for slots whose
/// parent has definite geometry, not intrinsic toolbar action groups.
#[component]
pub struct FillSlot;
#[view(component=FillSlot,name="KitFillSlot",systems=[fill_slots::system()])]
pub fn fill_slot() {}
#[system(order=PreRender)]
fn fill_slots(nodes:query!(Entity,FillSlot),commands:commands!()) {
    for (node,_) in nodes {commands.set::<FillSlotBounds>(node,());}
}
struct FillSlotBounds;
impl mirui::ui::property::Property for FillSlotBounds {
    type Value=();
    fn apply(world:&mut mirui::ecs::World,node:Entity,_:())->mirui::ui::property::PropertyChange {
        use mirui::ui::{Children,Style,dirty::Dirty,property::PropertyChange};
        let hosts=world.get::<Children>(node).map(|c|c.0.clone()).unwrap_or_default();
        let mut changed=false;
        for host in hosts {
            let mut style=world.get::<Style>(host).cloned().unwrap_or_default();
            if style.layout.width!=Dimension::percent(100) || style.layout.height!=Dimension::percent(100) {
                style.layout.width=Dimension::percent(100);style.layout.height=Dimension::percent(100);
                world.insert(host,style);world.insert(host,Dirty);changed=true;
            }
        }
        if changed {PropertyChange::Layout} else {PropertyChange::Unchanged}
    }
}

#[model]
pub struct DesktopChrome {
    #[observe]
    pub sidebar_visible:bool,
    #[observe]
    pub sidebar_width:i32,
}
impl Default for DesktopChrome {
    fn default()->Self {Self {sidebar_visible:true,sidebar_width:224}}
}
#[model]
impl DesktopChrome {
    pub fn toggle_sidebar(&mut self) {self.sidebar_visible=!self.sidebar_visible;}
    pub fn show_sidebar(&mut self,visible:bool) {self.sidebar_visible=visible;}
    pub fn resize_sidebar(&mut self,width:i32) {self.sidebar_width=width.clamp(144,480);}
}
/// Compact desktop chrome, not a second window manager. The host supplies real
/// caption controls; no decorative traffic lights pretend to close a window.
/// Controls and toggle stay mounted outside scrolling navigation. Model-backed
/// visibility preserves the sidebar subtree and its selection when collapsed.
/// Caller supplies a definite viewport and an initialized chrome model.
ui!(compose DesktopFrame (chrome:Option<DesktopChromeHandle>,caption_width:i32,controls_width:i32,toggle_width:i32,titlebar_height:i32,toolbar_height:i32,status_height:i32) {
    Column (width:Dimension::percent(100),height:Dimension::percent(100),bg_color:ColorToken::Surface) {
        Row (width:Dimension::percent(100),height:titlebar_height.max(32),align:AlignItems::Center,bg_color:ColorToken::SurfaceVariant) {
            Row (width:caption_width.max(0),height:Dimension::percent(100),align:AlignItems::Center,column_gap:style::GAP,padding:Padding::all(style::GAP)) [ChromeRegion{chrome:chrome.clone().expect("DesktopFrame requires chrome"),part:ChromePart::Caption(caption_width.max(controls_width.saturating_add(toggle_width).saturating_add(3*style::GAP)))}] {
                View (width:controls_width.max(0),height:Dimension::percent(100)) [FillSlot] {@@window_controls}
                View (width:toggle_width.max(0),height:Dimension::percent(100)) [FillSlot] {@@sidebar_toggle}
            }
            View (grow:1.0,height:Dimension::percent(100)) [FillSlot] {@@titlebar}
        }
        Row (width:Dimension::percent(100),grow:1.0) {
            Column (width:0,height:Dimension::percent(100),bg_color:ColorToken::SurfaceVariant) [FillSlot,ChromeRegion{chrome:chrome.clone().expect("DesktopFrame requires chrome"),part:ChromePart::Sidebar}] {@@sidebar}
            Column (grow:1.0,height:Dimension::percent(100)) {
                View (width:Dimension::percent(100),height:toolbar_height.max(0)) [FillSlot] {@@toolbar}
                View (width:Dimension::percent(100),grow:1.0) [FillSlot] {@@content}
                View (width:Dimension::percent(100),height:status_height.max(0)) [FillSlot] {@@status}
            }
        }
        View (position:Position::Absolute,left:0,top:0,width:Dimension::percent(100),height:Dimension::percent(100)) [FillSlot] {@@overlays}
    }
});

#[derive(Clone,Copy)]
pub enum ChromePart {Caption(i32),Sidebar}
#[component]
pub struct ChromeRegion {pub chrome:DesktopChromeHandle,pub part:ChromePart}
#[view(component=ChromeRegion,name="KitChromeRegion",systems=[sync_chrome::system()])]
pub fn chrome_region() {}
#[system]
fn sync_chrome(nodes:query!(Entity,ChromeRegion,mirui::ui::Style,optional mirui::ui::Hidden),commands:commands!()) {
    use mirui::ui::property::prop;
    for (entity,region,current,hidden) in nodes {
        let shown=region.chrome.sidebar_visible();
        let width=match region.part {
            ChromePart::Caption(minimum)=>if shown {region.chrome.sidebar_width().max(minimum)} else {minimum},
            ChromePart::Sidebar=>if shown {region.chrome.sidebar_width()} else {0},
        }.clamp(0,65535);
        if current.layout.width!=Dimension::px(width) {commands.set::<prop::Width>(entity,Dimension::px(width));}
        if matches!(region.part,ChromePart::Sidebar) && shown==hidden.is_some() {commands.set::<prop::Visible>(entity,shown);}
    }
}
