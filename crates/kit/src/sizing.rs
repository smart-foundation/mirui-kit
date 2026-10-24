//! Opt-in height-to-content for Mirui container compositions. Mirui measures
//! text intrinsically, but Auto containers do not aggregate their descendants.
//! One ContentFit on a content root sizes its unconstrained vertical descendants
//! using native layout, font geometry, visibility and property invalidation.
//! Explicit heights and flex-fill axes remain native-owned. Percentage heights
//! need a definite parent, except transparent 100% composition wrappers.
use alloc::vec::Vec;
use mirui::{ecs::{Entity,World},prelude::*,types::Fixed64,ui::{Children,Parent,Style,ComputedRect,property::{Property,PropertyChange}}};
#[component]
pub struct ContentFit;
#[component]
struct FittedHeight {last:Fixed}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum FitError {DepthLimit,NodeBudget,IndefinitePercentage,MissingGeometry,Overflow}
#[component]
pub struct ContentFitIssue {pub error:FitError}
struct Measure;
#[view(component=ContentFit,name="KitContentFit",systems=[measure::system()])]
pub fn content_fit() {}
#[system(order=PreRender)]
fn measure(roots:query!(Entity,ContentFit),commands:commands!()) {
    for (entity,_) in roots {commands.set::<Measure>(entity,());}
}
fn pixels(value:Dimension)->Result<Fixed,FitError> {
    match value {Dimension::Px(value)=>Ok(value.max(Fixed::ZERO)),Dimension::Auto|Dimension::Content=>Ok(Fixed::ZERO),Dimension::Percent(_)=>Err(FitError::IndefinitePercentage)}
}
fn natural(world:&mut World,node:Entity,parent_direction:FlexDirection,parent_height:Option<Fixed>,depth:usize,budget:&mut usize,changed:&mut bool)->Result<Fixed,FitError> {
    if depth>=64 {return Err(FitError::DepthLimit);}
    if *budget==0 {return Err(FitError::NodeBudget);}*budget-=1;
    if mirui::ui::branch::is_effectively_hidden(world,node) {return Ok(Fixed::ZERO);}
    let Some(style)=world.get::<Style>(node).cloned() else {return Ok(Fixed::ZERO);};
    let fitted=world.get::<FittedHeight>(node).map(|old|old.last);
    let root=world.has::<ContentFit>(node);
    let owned=fitted.is_some_and(|old|style.layout.height==Dimension::Px(old));
    if fitted.is_some() && !owned {world.remove::<FittedHeight>(node);}
    let explicit=if root||owned {None} else {match style.layout.height {
        Dimension::Px(height)=>Some(height.max(Fixed::ZERO)),
        Dimension::Percent(percent)=>if let Some(height)=parent_height {style.layout.height.resolve(height)} else if Dimension::Percent(percent)==Dimension::percent(100) {None} else {return Err(FitError::IndefinitePercentage);},
        Dimension::Auto|Dimension::Content=>None,
    }};
    let children:Vec<_>=world.get::<Children>(node).map(|children|children.0.clone()).unwrap_or_default();
    let mut extent=Fixed64::from(Fixed::ZERO);let mut count=0usize;
    let gap=pixels(style.layout.row_gap)?;
    for child in children {
        if mirui::ui::branch::is_effectively_hidden(world,child) || world.get::<Style>(child).is_some_and(|style|style.layout.position==Position::Absolute) {continue;}
        let height=natural(world,child,style.layout.direction,explicit,depth+1,budget,changed)?;
        if style.layout.direction==FlexDirection::Column {
            if count>0 {extent+=Fixed64::from(gap);}extent+=Fixed64::from(height);
        } else {extent=extent.max(Fixed64::from(height));}
        count+=1;
    }
    if let Some(height)=explicit {return Ok(height);}
    if count==0 {
        // Keep native text/input intrinsic measurement live; do not freeze a
        // leaf's old line height when text or typography changes.
        return world.get::<ComputedRect>(node).map(|rect|rect.0.h).ok_or(FitError::MissingGeometry);
    }
    extent+=Fixed64::from(pixels(style.layout.padding.top)?);
    extent+=Fixed64::from(pixels(style.layout.padding.bottom)?);
    if extent>Fixed64::from(Fixed::MAX) {return Err(FitError::Overflow);}
    let mut height=extent.to_fixed();
    let bound_height=parent_height.unwrap_or(Fixed::ZERO);
    if let Some(min)=style.layout.min_height.resolve(bound_height) {height=height.max(min);}
    if let Some(max)=style.layout.max_height.resolve(bound_height) {height=height.min(max);}
    // A row cross-axis naturally stretches to its measured line height. A
    // column's grow child still fills its native allocated remainder.
    let fit_axis=root || (parent_direction==FlexDirection::Column && style.layout.grow<=Fixed::ZERO);
    if fit_axis && style.layout.height!=Dimension::Px(height) {
        world.get_mut::<Style>(node).expect("measured style").layout.height=Dimension::Px(height);
        world.insert(node,FittedHeight{last:height});world.insert(node,mirui::ui::dirty::Dirty);*changed=true;
    }
    Ok(height)
}
impl Property for Measure {
    type Value=();
    fn apply(world:&mut World,entity:Entity,():())->PropertyChange {
        let parent=world.get::<Parent>(entity).map(|parent|parent.0);
        let direction=parent.and_then(|parent|world.get::<Style>(parent)).map(|style|style.layout.direction).unwrap_or(FlexDirection::Column);
        let mut changed=false;
        match natural(world,entity,direction,None,0,&mut 4096,&mut changed) {
            Ok(_)=>{world.remove::<ContentFitIssue>(entity);},
            Err(error)=>{if !world.get::<ContentFitIssue>(entity).is_some_and(|issue|issue.error==error) {world.insert(entity,ContentFitIssue{error});}},
        }
        if changed {PropertyChange::Layout} else {PropertyChange::Unchanged}
    }
}
