use crate::common;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetOurCityRequest;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AskForBuilding {
    pub building: common::BuildingKind,
    pub pos: common::Position,
    pub size: common::Size,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct VoteForBuilding {
    pub id: uuid::Uuid,
}
