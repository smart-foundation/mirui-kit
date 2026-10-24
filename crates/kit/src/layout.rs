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
}
