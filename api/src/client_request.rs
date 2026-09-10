use crate::common;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetOurCityRequest;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ConstructRequest {
    pub kind: common::BuildingKind,
    pub pos: common::Position,
    pub size: common::Size,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct VoteForConstruct {
    pub id: uuid::Uuid,
}
