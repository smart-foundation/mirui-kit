//! Isolated native specimens. These mount shared kit components, not replicas.
//! A capture is evidence for review, never automatic visual/UX approval.
use alloc::{string::ToString, vec};
use mirui::prelude::*;
use mirui::ui::widgets::Text;
use mirui_kit::{font, widgets::{number,toggle,text_size}, controls::{checkbox,choice_group,tree_view}, forms::{form_field,number_field}, choices::list_box, calendar::calendar, table::data_table, button::{button,ButtonSpec}, feedback::{alert,badge,empty_state,progress,separator,Severity}, split::SplitPane};
use crate::model::Studio;

#[derive(Clone,Copy,Debug)]
pub enum Specimen {Button,Field,Number,Checkbox,Switch,Slider,Group,List,Tree,Table,Calendar,Split,Feedback}
impl Specimen {
    pub const ALL:[Self;13]=[Self::Button,Self::Field,Self::Number,Self::Checkbox,Self::Switch,Self::Slider,Self::Group,Self::List,Self::Tree,Self::Table,Self::Calendar,Self::Split,Self::Feedback];
    pub fn name(self)->&'static str {match self {Self::Button=>"button",Self::Field=>"field",Self::Number=>"numeric-entry",Self::Checkbox=>"checkbox",Self::Switch=>"switch",Self::Slider=>"slider",Self::Group=>"choice-group",Self::List=>"listbox",Self::Tree=>"tree",Self::Table=>"table",Self::Calendar=>"calendar",Self::Split=>"split",Self::Feedback=>"feedback"}}
    pub fn variants(self)->&'static [&'static str] {match self {Self::Button=>&["rest","disabled","pending","checked","hover","pressed","focused"],Self::Field|Self::Number=>&["rest","invalid"],Self::Checkbox=>&["rest","checked","disabled"],Self::List|Self::Table=>&["rest","selected","empty"],Self::Calendar=>&["rest","range","disabled"],Self::Tree=>&["rest","expanded"],Self::Switch|Self::Slider|Self::Group=>&["rest","selected"],Self::Split=>&["rest","constrained"],Self::Feedback=>&["rest","error","pending"]}}
}
#[model]
pub struct Fixture {
    pub state:model!(owned Studio),
    pub button:model!(owned mirui_kit::button::ButtonState),
    pub progress:model!(owned mirui_kit::feedback::Progress),
    pub validation:model!(owned mirui_kit::forms::Validation),
}
impl Default for Fixture {
    fn default()->Self {Self {state:Studio::default().into(),button:mirui_kit::button::ButtonState::default().into(),progress:mirui_kit::feedback::Progress::default().into(),validation:mirui_kit::forms::Validation::default().into()}}
}
/// Prepare a real state without inventing user events. Interaction tests must
/// separately establish that pointer/keyboard flows can reach that state.
pub fn prepare(fixture:&FixtureHandle,kind:Specimen,variant:&str,scale:u16) {
    let state=fixture.state();state.initialize();state.metadata().set(false);state.actions().set_scale(scale);
    fixture.progress().set(4250);
    fixture.validation().describe("Persistent help text should wrap at the available width without colliding with adjacent controls.".into());
    match (kind,variant) {
        (Specimen::Button,"disabled")=>fixture.button().disable(true),
        (Specimen::Button,"pending")=>fixture.button().set_pending(true),
        (Specimen::Button,"checked")=>fixture.button().check(true),
        (Specimen::Field,"invalid")=>fixture.validation().set_error("This value is unavailable. Choose a different name; your draft has been retained.".into()),
        (Specimen::Number,"invalid")=>{state.demo_numeric().document().set_text("invalid").expect("fixture capacity");state.demo_numeric().commit();},
        (Specimen::Checkbox,"checked")=>state.demo_check().toggle(),
        (Specimen::Checkbox,"disabled")=>{mirui::core::model::ModelHandle::update(&state.demo_check(),|data|data.disabled=true);},
        (Specimen::Switch,"selected")=>state.metadata().set(true),
        (Specimen::Slider,"selected")=>state.quality().set(Fixed::from_int(85)),
        (Specimen::Group,"selected")=>{let _=state.demo_choices().activate(2);},
        (Specimen::List,"selected")=>state.demo_options().select_all(),
        (Specimen::List,"empty")=>{state.demo_options().replace(vec![]).expect("empty options");},
        (Specimen::Table,"selected")=>state.demo_table().choose(1),
        (Specimen::Table,"empty")=>{state.demo_table().replace_rows(vec![]).expect("empty rows");},
        (Specimen::Tree,"expanded")=>{state.demo_hierarchy().toggle(1);state.demo_hierarchy().toggle(2);},
        (Specimen::Calendar,"disabled")=>state.demo_calendar().set_disabled(true),
        (Specimen::Calendar,"range")=>{let _=state.demo_calendar().choose_focused();state.demo_calendar().navigate(mirui_kit::behavior::calendar::CalendarMove::Days(5));let _=state.demo_calendar().choose_focused();},
        (Specimen::Split,"constrained")=>{mirui::core::model::ModelHandle::update(&state.demo_split(),|data|{data.first_min=480;data.second_min=480;});},
        (Specimen::Feedback,"pending")=>fixture.progress().pending(),
        _=>{},
    }
}
#[compose]
pub fn specimen(fixture:model!(Fixture),state:model!(Studio),kind:Specimen,variant:&'static str)->Entity {
    ui! {Column (width:Dimension::percent(100),height:Dimension::percent(100),padding:Padding::all(24),row_gap:16,bg_color:ColorToken::Surface) {
        Text (text:alloc::format!("{} / {}",kind.name(),variant),width:Dimension::percent(100),font:font::SEMIBOLD,font_size:${text_size(state.actions().text_scale(),font::SECTION_SIZE)},text_color:ColorToken::OnSurface)
        Column (id:"specimen_bounds",width:Dimension::percent(100),row_gap:12) [mirui_kit::sizing::ContentFit] {
            match kind {
                Specimen::Button=>{button(state.actions(),fixture.button(),10,ButtonSpec{toggle:true,..ButtonSpec::labelled("A real shared action")},false)}
                Specimen::Field=>{form_field(state.actions(),state.preset(),fixture.validation(),10,"Preset name".to_string(),"Name this preset",false,0)}
                Specimen::Number=>{number_field(state.actions(),state.demo_numeric(),10,"Quality".to_string(),false)}
                Specimen::Checkbox=>{checkbox(state.actions(),state.demo_check(),10,"Include metadata".to_string(),false)}
                Specimen::Switch=>{toggle(state.actions(),state.metadata(),10,"Include metadata")}
                Specimen::Slider=>{number(state.actions(),state.quality(),10)}
                Specimen::Group=>{choice_group(state.actions(),state.demo_choices(),10,false)}
                Specimen::List=>{list_box(state.actions(),state.demo_options(),10,false)}
                Specimen::Tree=>{tree_view(state.actions(),state.demo_hierarchy(),10,false)}
                Specimen::Table=>{data_table(state.actions(),state.demo_table(),10,false)}
                Specimen::Calendar=>{calendar(state.actions(),state.demo_calendar(),10,false)}
                Specimen::Split=>{View (width:Dimension::percent(100),height:220) {SplitPane (width:Dimension::percent(100),height:Dimension::percent(100),state:Some(state.demo_split()),actions:Some(state.actions()),key:10u16,overlay:false) {
                    @first {Text ("First pane",font:font::MEDIUM,font_size:${text_size(state.actions().text_scale(),font::BODY_SIZE)},text_color:ColorToken::OnSurface)}
                    @second {Text ("Second pane",font:font::MEDIUM,font_size:${text_size(state.actions().text_scale(),font::BODY_SIZE)},text_color:ColorToken::OnSurface)}
                }}}
                Specimen::Feedback=>{
                    alert(state.actions(),if variant=="error" {Severity::Error} else {Severity::Information},"Operation status".to_string(),"A long, real description must wrap rather than paint outside its container, overlap another row, or disappear behind a neighboring control.".to_string())
                    badge(state.actions(),"Session only".to_string())
                    progress(state.actions(),fixture.progress(),"Transfer progress".to_string())
                    separator()
                    empty_state(state.actions(),"Nothing selected".to_string(),"Select an item to inspect its properties.".to_string())
                }
            }
        }
    }}
}

pub fn install<B:Surface,F:RendererFactory<B>>(app:&mut App<B,F>,kind:Specimen,variant:&'static str,scale:u16,theme:&str)->FixtureHandle {
    mirui_kit::install(app);
    let _=app.set_theme(theme);
    let fixture=app.add_model(Fixture::default()).finish();
    prepare(&fixture,kind,variant,scale);
    if variant=="focused" {mirui_kit::bind_actions(app,fixture.state().actions());}
    let root=app.spawn_root().id();app.compose(root,specimen(fixture.clone(),fixture.state(),kind,variant));
    fixture
}

/// The target carries the real shared focus contract, not a screenshot offset.
pub fn primary_control(world:&mirui::ecs::World)->Option<Entity> {
    world.query::<mirui_kit::FocusItem>().iter().find(|(_,item)|item.order==10).map(|(entity,_)|entity)
}
