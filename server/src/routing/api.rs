use std::{default, num::NonZeroU32};

use axum::{Json, http::StatusCode, response::IntoResponse};
use sea_orm::{ActiveModelTrait, EntityTrait, sea_query::ArrayType::Uuid, sqlx::types::chrono};

use crate::entities::construct;

/// Register all api method for v1
pub fn register_api(
    router: axum::Router<std::sync::Arc<crate::state::State>>,
) -> axum::Router<std::sync::Arc<crate::state::State>> {
    let router = router.route("/api/get_our_city", axum::routing::get(get_our_city_handler));
    let router = router.route("/api/ask_for_construct", axum::routing::post(ask_for_construct_handler));
    router
}

pub async fn get_our_city_handler(
    axum::extract::State(state): axum::extract::State<std::sync::Arc<crate::state::State>>,
) -> axum::response::Response {
    let buildings = match crate::entities::building::Entity::find().all(state.db_conn()).await {
        Ok(building) => building,
        Err(e) => {
            tracing::warn!("I think the db exploded: {e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
        }
    };

    let mut building_list = Vec::with_capacity(buildings.len());

    for building in buildings {
        building_list.push(api::common::Building {
            id: building.id,
            created_at: building.created_at.into(),
            kind: building.kind.into(),
            pos: api::common::Position {
                x: building.x,
                y: building.y,
            },
            size: api::common::Size {
                width: match convert_i32_nonzerou32(building.width) {
                    Ok(width) => width,
                    Err(e) => {
                        tracing::warn!(e);
                        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
                    }
                },
                height: match convert_i32_nonzerou32(building.height) {
                    Ok(width) => width,
                    Err(e) => {
                        tracing::warn!(e);
                        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
                    }
                },
            },
        })
    }

    let constructs = match crate::entities::construct::Entity::find().all(state.db_conn()).await {
        Ok(construct) => construct,
        Err(e) => {
            tracing::warn!("I think the db exploded: {e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
        }
    };
    let mut construct_list = Vec::with_capacity(constructs.len());

    for construct in constructs {
        construct_list.push(api::common::Construct {
            building: api::common::Building {
                id: construct.id,
                created_at: construct.created_at.into(),
                kind: construct.kind.into(),
                pos: api::common::Position {
                    x: construct.x,
                    y: construct.y,
                },
                size: api::common::Size {
                    width: match convert_i32_nonzerou32(construct.width) {
                        Ok(width) => width,
                        Err(e) => {
                            tracing::warn!(e);
                            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
                        }
                    },
                    height: match convert_i32_nonzerou32(construct.height) {
                        Ok(width) => width,
                        Err(e) => {
                            tracing::warn!(e);
                            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
                        }
                    },
                },
            },
            status: api::common::ConstructStatus {
                cost: match u32::try_from(construct.cost) {
                    Ok(c_val) => c_val,
                    Err(e) => {
                        tracing::warn!("Failed to convert i32 -> u32: {} {}", construct.cost, e);
                        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
                    }
                },
                vote: match u32::try_from(construct.votes) {
                    Ok(c_val) => c_val,
                    Err(e) => {
                        tracing::warn!("Failed to convert i32 -> u32: {} {}", construct.votes, e);
                        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
                    }
                },
            },
        })
    }

    let our_city = api::server_response::GetOurCityResponse {
        buildings: building_list,
        constructs: construct_list,
    };
    let our_city_response = match serde_json::to_string(&our_city) {
        Ok(response) => response,
        Err(e) => {
            tracing::warn!("Failed to json convert {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };
    our_city_response.into_response()
}

pub async fn ask_for_construct_handler(
    axum::extract::State(state): axum::extract::State<std::sync::Arc<crate::state::State>>,
    axum::extract::Json(construct_requested): axum::extract::Json<api::client_request::ConstructRequest>,
) -> axum::response::Response {
    //TODO: Check inputs (check collision)

    //Insert new construct in db
    let construct = crate::entities::construct::ActiveModel {
        id: Default::default(),
        created_at: Default::default(),
        kind: sea_orm::ActiveValue::Set(construct_requested.kind.into()),
        x: sea_orm::ActiveValue::Set(construct_requested.pos.x),
        y: sea_orm::ActiveValue::Set(construct_requested.pos.y),
        width: sea_orm::ActiveValue::Set(match i32::try_from(construct_requested.size.width.get()) {
            Ok(c_val) => c_val,
            Err(e) => {
                tracing::warn!("Failed to convert u32 -> i32: {} {}", construct_requested.size.width.get(), e);
                return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
            }
        }),
        height: sea_orm::ActiveValue::Set(match i32::try_from(construct_requested.size.height.get()) {
            Ok(c_val) => c_val,
            Err(e) => {
                tracing::warn!(
                    "Failed to convert u32 -> i32: {} {}",
                    construct_requested.size.height.get(),
                    e
                );
                return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
            }
        }),
        cost: sea_orm::ActiveValue::Set(5),
        votes: sea_orm::ActiveValue::Set(1),
    };

    match construct.insert(state.db_conn()).await {
        Ok(_) => (),
        Err(e) => {
            tracing::warn!("Failed to insert construct {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }

    api::server_response::OkNotOk::Ok.into_response()
}

pub async fn vote_for_construct_handler(
    axum::extract::State(state): axum::extract::State<std::sync::Arc<crate::state::State>>,
    axum::extract::Json(construct_requested): axum::extract::Json<api::client_request::ConstructRequest>,
) -> axum::response::Response {
    api::server_response::OkNotOk::Ok.into_response()
}

//helper func to convert i32 to NonZeroU32
fn convert_i32_nonzerou32(val: i32) -> Result<NonZeroU32, String> {
    let c_val: u32 = match u32::try_from(val) {
        Ok(c_val) => c_val,
        Err(e) => return Err(format!("Failed to convert i32 -> u32: {} {}", val, e)),
    };
    match NonZeroU32::new(c_val) {
        Some(c_val) => Ok(c_val),
        None => return Err(format!("Random zero shit is stored in db: {}", val)),
    }
}
