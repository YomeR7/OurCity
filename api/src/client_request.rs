#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetOurCityRequest;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ConstructRequest {
    pub kind: crate::common::BuildingKind,
    pub pos: crate::common::Position,
    pub size: crate::common::Size,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct VoteForConstruct {
    pub id: uuid::Uuid,
    pub vote: crate::common::UpVoteDownVote,
}
