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

#[test]
fn showcase_navigation_has_one_compact_selection_surface() {
    for scale in [100,150] {
        let mut app=App::headless(1400,900);
        app.with_default_widgets().with_default_systems();
        let state=studio::install(&mut app);
        state.actions().set_scale(scale);
        for _ in 0..6 {app.tick();app.render().unwrap();}
        let mut count=0;
        for (entity,_) in app.world.query::<studio::ui::NavigationItem>().iter() {
            let rect=app.world.get::<mirui::ui::ComputedRect>(entity).unwrap().0;
            assert_eq!(rect.h,Fixed::from_int(mirui_kit::widgets::text_size(scale,32) as i32),"navigation backing must match its control, not add a padded second selection");
            assert!(rect.w>Fixed::ZERO);
            count+=1;
        }
        assert_eq!(count,6);
    }
}

#[test]
fn preview_card_hugs_content_and_task_actions_do_not_fill_the_form() {
    use studio::commands::Command;
    for scale in [100,150] {
        let mut app=App::headless(1400,900);
        app.with_default_widgets().with_default_systems();
        let state=studio::install(&mut app);
        state.actions().set_scale(scale);
        state.actions().activate(Command::Controls as u16,false).unwrap();
        for _ in 0..8 {app.tick();app.render().unwrap();}
        let card=app.world.query::<studio::ui::PreviewCard>().iter().next().expect("preview card").0;
        let rect=app.world.get::<mirui::ui::ComputedRect>(card).unwrap().0;
        assert!(rect.h>Fixed::from_int(80),"preview content collapsed");
        assert!(rect.h<Fixed::from_int(300),"short preview stretched into an empty panel");
        let strip=app.world.query::<studio::ui::TaskActions>().iter().next().expect("task action strip").0;
        let strip=app.world.get::<mirui::ui::ComputedRect>(strip).unwrap().0;
        for command in [Command::Preview,Command::SavePreset] {
            let action=app.world.query::<mirui_kit::FocusItem>().iter().find(|(_,item)|item.order==command as u16).expect("task action").0;
            let action=app.world.get::<mirui::ui::ComputedRect>(action).unwrap().0;
            assert!(action.x>=strip.x && action.x+action.w<=strip.x+strip.w,"action overflows the available form width");
            assert!(action.y>=strip.y && action.y+action.h<=strip.y+strip.h,"wrapped action overflows its strip height");
            assert!(action.w>Fixed::from_int(100));
            assert!(action.w<Fixed::from_int(400),"task action stretched across its form");
            assert_eq!(action.h,Fixed::from_int(mirui_kit::widgets::text_size(scale,32) as i32));
        }
    }
}
