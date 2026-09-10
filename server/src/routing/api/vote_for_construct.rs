use api::server_response::OkNotOk;
use api::server_ws;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sea_orm::ActiveModelTrait;
use sea_orm::EntityTrait;
use sea_orm::TransactionTrait;

pub async fn vote_for_construct_handler(
    axum::extract::State(state): axum::extract::State<std::sync::Arc<crate::state::State>>,
    axum::extract::Json(vote_request): axum::extract::Json<api::client_request::VoteForConstruct>,
) -> axum::response::Response {
    // Convert the vote into numerical values
    let vote_value = match vote_request.vote {
        api::common::UpVoteDownVote::UpVote => 1,
        api::common::UpVoteDownVote::DownVote => -1,
    };

    //Start a database transaction
    let db_tn = match state.db_conn().begin().await {
        Ok(db_tn) => db_tn,
        Err(e) => {
            tracing::warn!("Failed to start a database transaction {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let construct = match crate::entities::construct::Entity::find_by_id(vote_request.id)
        .one(&db_tn)
        .await
    {
        Ok(Some(construct)) => construct,
        Ok(None) => {
            return OkNotOk::NotOk {
                reason: format!("this building doesn't exist in db, uuid:{}", vote_request.id),
            }
            .into_response();
        }
        Err(e) => {
            tracing::warn!("Failed to retrieve {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };
    // Save the construct data for future message
    let construct_id = construct.id;
    let construct_votes = construct.votes;
    let construct_cost = construct.cost;

    let websocket_msg = if construct_votes + vote_value >= construct_cost {
        let building = crate::entities::building::ActiveModel {
            id: sea_orm::ActiveValue::Set(construct_id),
            created_at: Default::default(),
            kind: sea_orm::ActiveValue::Set(construct.kind.clone()),
            x: sea_orm::ActiveValue::Set(construct.x),
            y: sea_orm::ActiveValue::Set(construct.y),
            width: sea_orm::ActiveValue::Set(construct.width),
            height: sea_orm::ActiveValue::Set(construct.height),
        };

        // Move construct to building
        match building.insert(&db_tn).await {
            Ok(_) => (),
            Err(e) => {
                tracing::warn!("Failed to build the construct (in database O_o) {}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
            }
        }

        // Delete the construct
        let construct_to_remove: crate::entities::construct::ActiveModel = construct.into();
        match construct_to_remove.delete(&db_tn).await {
            Ok(_) => (),
            Err(e) => {
                tracing::warn!("Failed to clear the construct in database {}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
            }
        };

        api::server_ws::ServerEvent::BuildingBuilt(server_ws::BuildingBuilt { id: construct_id })
    } else {
        let new_vote_count = construct.votes + vote_value;
        let mut new_construct: crate::entities::construct::ActiveModel = construct.into();

        new_construct.votes = sea_orm::ActiveValue::Set(new_vote_count);

        match new_construct.update(&db_tn).await {
            Ok(_) => (),
            Err(e) => {
                tracing::warn!("Failed to update the vote in db {}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
            }
        };
        api::server_ws::ServerEvent::ConstructStatusUpdated(server_ws::ConstructStatusUpdated {
            id: construct_id,
            status: api::common::ConstructStatus {
                cost: match u32::try_from(construct_cost) {
                    Ok(c_val) => c_val,
                    Err(e) => {
                        tracing::warn!("Failed convert i32 to u32 {}", e);
                        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
                    }
                },
                vote: construct_votes,
            },
        })
    };

    // Close the database transaction
    match db_tn.commit().await {
        Ok(_) => (),
        Err(e) => {
            tracing::warn!("Failed to close the database transaction {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }

    // Websocket
    state.broadcast(websocket_msg);
    api::server_response::OkNotOk::Ok.into_response()
}
