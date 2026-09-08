use crate::common;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone)]
pub struct BuildingAsked {
    building: common::Building,
    status: common::BuildingStatus,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone)]
pub struct BuildingConfirmed {
    building: common::Building,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone)]
pub struct BuildingStatusUpdated {
    id: uuid::Uuid,
    status: common::BuildingStatus,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone)]
pub enum ServerEvent {
    BuildingAsked(BuildingAsked),
    BuildingConfirmed(BuildingConfirmed),
    BuildingStatusUpdated(BuildingStatusUpdated),
}
