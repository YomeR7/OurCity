use crate::common::Building;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetOurCityResponse {
    building_list: Vec<Building>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum OkNotOk {
    Ok,
    NotOk { reason: String },
}
