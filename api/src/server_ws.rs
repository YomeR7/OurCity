use crate::common;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone)]
pub struct NewConstruct {
    pub building: common::Building,
    pub status: common::ConstructStatus,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone)]
pub struct BuildingBuilt {
    pub id: uuid::Uuid,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone)]
pub struct ConstructStatusUpdated {
    pub id: uuid::Uuid,
    pub status: common::ConstructStatus,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone)]
pub enum ServerEvent {
    NewConstruct(NewConstruct),
    BuildingBuilt(BuildingBuilt),
    ConstructStatusUpdated(ConstructStatusUpdated),
}
