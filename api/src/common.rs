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
    id: uuid::Uuid,
    created_at: chrono::DateTime<chrono::Utc>,
    kind: BuildingKind,
    pos: Position,
    size: Size,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone)]
pub struct BuildingStatus {
    cost: u32,
    vote: u32,
}
