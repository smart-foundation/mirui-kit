//! Deterministic isolated native component evidence, independent of route scroll.
mod capture_png;
mod capture_surface;
use mirui::{prelude::*,surface::{InputEvent,PointerKind}};
use capture_surface::CaptureSurface;
use studio::specimens::{self,Specimen};
use std::path::PathBuf;
fn main() {
    let mut output=PathBuf::from("artifacts/studio/components");
    let mut theme="dark".to_string();let mut scale=100u16;let mut width=640u16;
    let mut args=std::env::args().skip(1);
    while let Some(arg)=args.next() {
        let value=args.next().expect("each option requires a value");
        match arg.as_str() {"--out"=>output=value.into(),"--theme"=>theme=value,"--scale"=>scale=value.parse().expect("integer scale"),"--width"=>width=value.parse().expect("integer width"),_=>panic!("unknown option {arg}")}
    }
    assert!(matches!(theme.as_str(),"dark"|"light"));assert!(matches!(scale,100|150));assert!((480..=1600).contains(&width));
    let height=900u16;std::fs::create_dir_all(&output).expect("capture directory");
    let mut cases=Vec::new();
    for kind in Specimen::ALL {
        for &variant in kind.variants() {
            let backend=CaptureSurface::new(width,height);
            let mut app=App::new(backend);app.with_default_widgets().with_default_systems();
            let _fixture=specimens::install(&mut app,kind,variant,scale,&theme);
            for _ in 0..4 {app.tick();app.render().expect("native specimen frame");}
            if matches!(variant,"hover"|"pressed"|"focused") {
                let target=specimens::primary_control(&app.world).expect("interactive specimen target");
                let target_rect=app.world.get::<mirui::ui::ComputedRect>(target).expect("target geometry").0;
                assert!(target_rect.w>Fixed::ZERO && target_rect.h>Fixed::ZERO);
                let x=target_rect.x+target_rect.w/2;let y=target_rect.y+target_rect.h/2;
                let event=match variant {
                    "hover"=>InputEvent::PointerMove{kind:PointerKind::Mouse,id:0,x,y},
                    "pressed"=>InputEvent::PointerDown{kind:PointerKind::Mouse,id:0,x,y},
                    _=>InputEvent::Key{physical:Some(PhysicalKey::Tab),code:9,pressed:true,modifiers:Some(Default::default()),repeat:Some(false)},
                };
                app.backend.events.push_back(event);app.tick();app.render().expect("interaction frame");
                if variant=="focused" {assert_eq!(app.world.resource::<mirui::input::event::focus::FocusState>().expect("native focus").focused,Some(target));}
            }
            let node=app.world.find_by_id("specimen_bounds").expect("specimen landmark");
            let rect=app.world.get::<mirui::ui::ComputedRect>(node).expect("native geometry").0;
            assert!(rect.w>Fixed::ZERO && rect.h>Fixed::ZERO,"{} / {variant}: collapsed specimen",kind.name());
            assert!(rect.x>=Fixed::ZERO && rect.y>=Fixed::ZERO && rect.x+rect.w<=Fixed::from_int(width.into()) && rect.y+rect.h<=Fixed::from_int(height.into()),"{} / {variant}: specimen exceeds capture viewport: {rect:?}",kind.name());
            let name=format!("{}--{}--{}--{}--{}.png",kind.name(),variant,theme,scale,width);
            let image=app.snapshot().expect("native snapshot").expect("rendered frame");
            capture_png::write_png(&output.join(&name),&image.read_pixels().expect("CPU readback"));
            cases.push(format!("{{\"file\":\"{name}\",\"component\":\"{}\",\"state\":\"{variant}\",\"bounds\":[{},{},{},{}]}}",kind.name(),rect.x.to_int(),rect.y.to_int(),rect.w.to_int(),rect.h.to_int()));
        }
    }
    let index=format!("{{\"schema\":1,\"renderer\":\"mirui-0.51.0\",\"theme\":\"{theme}\",\"scale\":{scale},\"viewport\":[{width},{height}],\"captures\":[{}]}}\n",cases.join(","));
    std::fs::write(output.join("index.json"),index).expect("capture index");
    println!("{} native specimens saved to {} (not visual approval)",cases.len(),output.display());
}
