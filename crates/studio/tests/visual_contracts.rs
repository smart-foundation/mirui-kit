//! Real native geometry contracts for defects found in screenshot review.
use mirui::prelude::*;
use studio::specimens::{self,Specimen};
#[test]
fn numeric_steps_align_with_the_editable_field_at_both_text_scales() {
    for theme in ["dark","light"] {for scale in [100,150] {
        let mut app=App::headless(640,900);app.with_default_widgets().with_default_systems();
        let _fixture=specimens::install(&mut app,Specimen::Number,"rest",scale,theme);
        for _ in 0..4 {app.tick();app.render().unwrap();}
        let frame=app.world.query::<mirui_kit::field::FieldFrame>().iter().next().expect("numeric field frame").0;
        let rect=app.world.get::<mirui::ui::ComputedRect>(frame).unwrap().0;
        for key in [11,12] {
            let step=app.world.query::<mirui_kit::FocusItem>().iter().find(|(_,item)|item.order==key).expect("step button").0;
            let step=app.world.get::<mirui::ui::ComputedRect>(step).unwrap().0;
            assert_eq!((step.y,step.h),(rect.y,rect.h),"{theme}/{scale}: input and step edges differ");
            assert_eq!(step.w,Fixed::from_int(mirui_kit::widgets::text_size(scale,mirui_kit::style::FIELD_HEIGHT).into()));
        }
    }}
}
#[test]
fn a_short_badge_hugs_its_content_instead_of_stretching_into_a_banner() {
    for scale in [100,150] {
        let mut app=App::headless(640,900);app.with_default_widgets().with_default_systems();
        let _fixture=specimens::install(&mut app,Specimen::Feedback,"rest",scale,"light");
        for _ in 0..4 {app.tick();app.render().unwrap();}
        let badge=app.world.query::<mirui_kit::feedback::BadgeSurface>().iter().next().expect("badge surface").0;
        let rect=app.world.get::<mirui::ui::ComputedRect>(badge).unwrap().0;
        let bounds=app.world.find_by_id("specimen_bounds").unwrap();
        let bounds=app.world.get::<mirui::ui::ComputedRect>(bounds).unwrap().0;
        assert!(rect.w>Fixed::ZERO && rect.h>Fixed::ZERO);
        assert!(rect.w<bounds.w/2,"short badge unexpectedly filled the content column");
    }
}
