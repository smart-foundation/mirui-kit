//! Script real pointer/key input through Mirui; never paint a pretend hover.
use std::collections::VecDeque;
use mirui::{surface::{Surface,FramebufferAccess,DisplayInfo,InputEvent,framebuf::FramebufSurface},render::{RenderError,texture::{Texture,ColorFormat}},types::PhysicalRect};
pub struct CaptureSurface {framebuffer:FramebufSurface<fn(&[u8],PhysicalRect)>,pub events:VecDeque<InputEvent>}
impl CaptureSurface {
    pub fn new(width:u16,height:u16)->Self {Self{framebuffer:FramebufSurface::with_format(width,height,ColorFormat::RGBA8888,|_,_|{}),events:VecDeque::new()}}
}
impl mirui::core::cache::InspectCaches for CaptureSurface {}
impl Surface for CaptureSurface {
    fn display_info(&self)->DisplayInfo {self.framebuffer.display_info()}
    fn flush(&mut self,area:PhysicalRect)->Result<(),RenderError> {self.framebuffer.flush(area)}
    fn poll_event(&mut self)->Option<InputEvent> {self.events.pop_front()}
}
impl FramebufferAccess for CaptureSurface {fn framebuffer(&mut self)->Texture<'_> {self.framebuffer.framebuffer()}}
