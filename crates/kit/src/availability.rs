//! Native state property with a queryable ownership marker. Mirui's UserState
//! is not a Component; its public Property extension point supplies typed access.
use mirui::ecs::{Entity, World};
use mirui::ui::{UserState, property::{Property, PropertyChange}};
#[derive(Clone,Copy)]
enum Previous { None, Disabled, Errored }
#[mirui::component]
pub struct InputBlocked { previous: Previous }
pub struct InputAvailability;
impl Property for InputAvailability {
    type Value = bool;
    fn apply(world: &mut World, entity: Entity, enabled: bool) -> PropertyChange {
        let blocked = world.has::<InputBlocked>(entity);
        if blocked == !enabled { return PropertyChange::Unchanged; }
        if enabled {
            let previous = world.get::<InputBlocked>(entity).map(|b|b.previous).unwrap_or(Previous::None);
            world.remove::<InputBlocked>(entity);
            // Do not erase an error written by another owner during blocking.
            if matches!(world.get::<UserState>(entity), Some(UserState::Disabled)) {
                match previous {
                    Previous::None => { world.remove::<UserState>(entity); },
                    Previous::Disabled => {},
                    Previous::Errored => { world.insert(entity, UserState::Errored); },
                }
            }
        } else {
            let previous = match world.get::<UserState>(entity) {
                Some(UserState::Disabled) => Previous::Disabled,
                Some(UserState::Errored) => Previous::Errored,
                None => Previous::None,
            };
            world.insert(entity, InputBlocked { previous });
            world.insert(entity, UserState::Disabled);
        }
        PropertyChange::Visual
    }
}
