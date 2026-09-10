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

#[cfg(feature = "server")]
impl axum::response::IntoResponse for OkNotOk {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Ok => axum::http::StatusCode::OK.into_response(),
            Self::NotOk { reason } => (axum::http::StatusCode::BAD_REQUEST, reason).into_response(),
        }
    }
}
