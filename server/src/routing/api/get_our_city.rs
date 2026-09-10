use super::api_utils;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sea_orm::EntityTrait;

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
                width: match api_utils::convert_i32_nonzerou32(building.width) {
                    Ok(width) => width,
                    Err(e) => {
                        tracing::warn!(e);
                        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
                    }
                },
                height: match api_utils::convert_i32_nonzerou32(building.height) {
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
                    width: match api_utils::convert_i32_nonzerou32(construct.width) {
                        Ok(width) => width,
                        Err(e) => {
                            tracing::warn!(e);
                            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
                        }
                    },
                    height: match api_utils::convert_i32_nonzerou32(construct.height) {
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
                vote: construct.votes,
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
