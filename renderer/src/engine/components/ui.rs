/// Marker component for the info panel for buildings
#[derive(bevy::prelude::Component)]
pub struct BuildingInfoPanel;

/// Marker component for info panels titles
#[derive(bevy::prelude::Component)]
pub struct BuildingPanelTitle;

/// Marker component for info panels texts
#[derive(bevy::prelude::Component)]
pub struct BuildingPanelContent;

/// Marker component for the info panel for buildings
#[derive(bevy::prelude::Component)]
pub struct ConstructInfoPanel;

/// Marker component for info panels titles
#[derive(bevy::prelude::Component)]
pub struct ConstructPanelTitle;

/// Marker component for info panels texts
#[derive(bevy::prelude::Component)]
pub struct ConstructPanelContent;

/// Marker component for the UI selection beam
#[derive(bevy::prelude::Component)]
pub struct UiSelectionBeam;

/// Marker component for all objects that can be hidden for deselection
#[derive(bevy::prelude::Component)]
pub struct HiddableSelectionMarker;

/// Button kind to mark which buttons does what
#[derive(bevy::prelude::Component)]
pub enum ButtonKind {
    UpvoteBuilding,
    DownvoteBuilding,
}
