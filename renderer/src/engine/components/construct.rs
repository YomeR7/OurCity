/// Building component currently being voted.
#[derive(bevy::prelude::Component)]
#[require(bevy::prelude::Transform, bevy::prelude::Visibility)]
pub struct Construct {
    pub building: api::common::Building,
    pub status: api::common::ConstructStatus,
}

/// Marker component for a construct being selected by the user
#[derive(bevy::prelude::Component)]
#[require(bevy::prelude::Transform, bevy::prelude::Visibility)]
pub struct SelectedConstruct;
