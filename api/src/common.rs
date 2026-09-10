#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone)]
pub enum BuildingKind {
    House,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone)]
pub struct Size {
    pub width: std::num::NonZeroU32,
    pub height: std::num::NonZeroU32,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone)]
pub struct Building {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub kind: BuildingKind,
    pub pos: Position,
    pub size: Size,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone)]
pub struct Construct {
    pub building: Building,
    pub status: ConstructStatus,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone)]
pub struct ConstructStatus {
    pub cost: u32,
    pub vote: u32,
}
