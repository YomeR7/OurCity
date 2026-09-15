/// Building component
#[derive(bevy::prelude::Component)]
#[require(bevy::prelude::Transform, bevy::prelude::Visibility)]
pub struct Building {
    pub building: api::common::Building,
}

/// Component for an entity that is the ghost building for display
#[derive(bevy::prelude::Component)]
#[require(bevy::prelude::Transform, bevy::prelude::Visibility)]
pub struct BuildingGhost {
    pub position: Option<api::common::Position>,
}

/// Marker component for a building being selected by the user
#[derive(bevy::prelude::Component)]
#[require(bevy::prelude::Transform, bevy::prelude::Visibility)]
pub struct SelectedBuilding;
