#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct NewConstruct {
    pub construct: crate::common::Construct,
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ConstructCancelled {
    pub id: uuid::Uuid,
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BuildingBuilt {
    pub id: uuid::Uuid,
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ConstructStatusUpdated {
    pub id: uuid::Uuid,
    pub status: crate::common::ConstructStatus,
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ServerEvent {
    NewConstruct(NewConstruct),
    ConstructCancelled(ConstructCancelled),
    BuildingBuilt(BuildingBuilt),
    ConstructStatusUpdated(ConstructStatusUpdated),
}
