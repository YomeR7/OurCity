#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum BuildingKind {
    House,
}

impl std::fmt::Display for BuildingKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::House => write!(f, "House"),
        }
    }
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum UpVoteDownVote {
    UpVote,
    DownVote,
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Size {
    pub width: std::num::NonZeroU32,
    pub height: std::num::NonZeroU32,
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Building {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub kind: BuildingKind,
    pub pos: Position,
    pub size: Size,
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Construct {
    pub building: Building,
    pub status: ConstructStatus,
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ConstructStatus {
    pub cost: u32,
    pub votes: i32,
}
