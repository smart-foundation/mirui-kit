//! Typed native state property. UserState is deliberately not a query Component
//! in Mirui 0.51; use its property extension point rather than inserting it via
//! Component-only deferred commands or querying it as a component.
use mirui::ecs::{Entity, World};
use mirui::ui::{UserState, property::{Property, PropertyChange}};
#[mirui::component]
pub struct InputBlocked;
pub struct InputAvailability;
impl Property for InputAvailability {
    type Value = bool;
    fn apply(world: &mut World, entity: Entity, enabled: bool) -> PropertyChange {
        let blocked = world.has::<InputBlocked>(entity);
        if blocked == !enabled { return PropertyChange::Unchanged; }
        if enabled {
            world.remove::<InputBlocked>(entity);
            if matches!(world.get::<UserState>(entity), Some(UserState::Disabled)) {
                world.remove::<UserState>(entity);
            }
        } else {
            world.insert(entity, InputBlocked);
            world.insert(entity, UserState::Disabled);
        }
        PropertyChange::Visual
    }
}
