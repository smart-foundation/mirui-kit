//! Feed real input through Mirui's normal surface and event runtime.
use std::collections::VecDeque;
use mirui::{app::App,render::{RenderError,SwRendererFactory,texture::{Texture,ColorFormat}},surface::{Surface,FramebufferAccess,DisplayInfo,InputEvent,framebuf::FramebufSurface},types::PhysicalRect};
pub struct ScriptedSurface {
    framebuffer:FramebufSurface<fn(&[u8],PhysicalRect)>,
    pub events:VecDeque<InputEvent>,
}
impl mirui::core::cache::InspectCaches for ScriptedSurface {}
impl Surface for ScriptedSurface {
    fn display_info(&self)->DisplayInfo {self.framebuffer.display_info()}
    fn flush(&mut self,area:PhysicalRect)->Result<(),RenderError> {self.framebuffer.flush(area)}
    fn poll_event(&mut self)->Option<InputEvent> {self.events.pop_front()}
}
impl FramebufferAccess for ScriptedSurface {
    fn framebuffer(&mut self)->Texture<'_> {self.framebuffer.framebuffer()}
}
pub type TestApp=App<ScriptedSurface,SwRendererFactory>;
pub fn app()->TestApp {
    let mut app=App::new(ScriptedSurface{framebuffer:FramebufSurface::with_format(240,200,ColorFormat::RGBA8888,|_,_|{}),events:VecDeque::new()});
    app.with_default_widgets().with_default_systems();mirui_kit::install(&mut app);app
}
pub fn tap(app:&mut TestApp,entity:mirui::ecs::Entity) {
    use mirui::{types::Fixed,surface::PointerKind};
    let rect=app.world.get::<mirui::ui::ComputedRect>(entity).unwrap().0;
    assert!(rect.w>Fixed::ZERO && rect.h>Fixed::ZERO,"input target needs real geometry");
    let (x,y)=(rect.x+rect.w/2,rect.y+rect.h/2);
    app.backend.events.extend([InputEvent::PointerDown{kind:PointerKind::Mouse,id:0,x,y},InputEvent::PointerUp{kind:PointerKind::Mouse,id:0,x,y}]);app.tick();
}
