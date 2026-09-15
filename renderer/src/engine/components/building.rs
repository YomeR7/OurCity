/// Building component
#[derive(bevy::prelude::Component)]
#[require(bevy::prelude::Transform, bevy::prelude::Visibility)]
pub struct Building {
    building: api::common::Building,
}

impl Building {
    /// Creates a new building component
    pub fn new(building: api::common::Building) -> Self {
        Self { building }
    }

    /// Get the title of the building for in game informations.
    pub fn title(&self) -> String {
        self.building.kind.to_string()
    }

    /// Get the description of the building for the in game informations.
    pub fn description(&self) -> String {
        format!(
            "A cute lil house.\nCreated the {}",
            self.building.created_at.format("%Y-%m-%d at %H:%M")
        )
    }

    /// Get the grid position of this building
    pub fn grid_pos(&self) -> crate::engine::components::grid::GridPos {
        crate::engine::components::grid::GridPos {
            x: self.building.pos.x,
            y: self.building.pos.y,
        }
    }
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
