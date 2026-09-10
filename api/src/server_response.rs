#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetOurCityResponse {
    pub buildings: Vec<crate::common::Building>,
    pub constructs: Vec<crate::common::Construct>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum OkNotOk {
    Ok,
    NotOk { reason: String },
}
