use super::api_utils;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sea_orm::ActiveModelTrait;

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

    let new_construct = match construct.insert(state.db_conn()).await {
        Ok(new_construct) => new_construct,
        Err(e) => {
            tracing::warn!("Failed to insert construct {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    // Websocket
    state.broadcast(api::server_ws::ServerEvent::NewConstruct(api::server_ws::NewConstruct {
        building: api::common::Building {
            id: new_construct.id,
            created_at: new_construct.created_at.into(),
            kind: new_construct.kind.into(),
            pos: api::common::Position {
                x: new_construct.x,
                y: new_construct.y,
            },
            size: api::common::Size {
                width: match api_utils::convert_i32_nonzerou32(new_construct.width) {
                    Ok(width) => width,
                    Err(e) => {
                        tracing::warn!(e);
                        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
                    }
                },
                height: match api_utils::convert_i32_nonzerou32(new_construct.height) {
                    Ok(width) => width,
                    Err(e) => {
                        tracing::warn!(e);
                        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
                    }
                },
            },
        },
        status: api::common::ConstructStatus {
            cost: match u32::try_from(new_construct.cost) {
                Ok(c_val) => c_val,
                Err(e) => {
                    tracing::warn!("Failed convert i32 to u32 {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
                }
            },
            vote: new_construct.votes,
        },
    }));

    api::server_response::OkNotOk::Ok.into_response()
}
