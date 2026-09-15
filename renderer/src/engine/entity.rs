/// Entity reference, tagged with the entity kind
pub enum BuildingEntity {
    Building(bevy::prelude::Entity),
    Construct(bevy::prelude::Entity),
}
