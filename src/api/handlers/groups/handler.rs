use axum::{Json, extract::State, handler::Handler, response::IntoResponse};
use axum_macros::debug_handler;
use devcord_middlewares::{middlewares::auth::Authenticated, models::claims::Claims};
use uuid::Uuid;

use crate::{
    api::{
        dto::{
            CreateGroupRequest, CreateGroupResponse, DeleteGroupRequest, GetGroupRequest,
            GetGroupResponse, ListUserGroupsResponse,
        },
        error::HandlerError,
        router::{AppState, SharedState},
    },
    application::dto::GroupInfo,
};

#[debug_handler]
pub async fn create_group(
    State(AppState { service }): State<SharedState>,
    Authenticated {
        claims: Claims { user_id, .. },
        ..
    }: Authenticated,
    Json(CreateGroupRequest { name, member_ids }): Json<CreateGroupRequest>,
) -> Result<Json<CreateGroupResponse>, HandlerError> {
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| HandlerError::BadRequest("Invalid `user_id".to_string()))?;

    let GroupInfo { id, .. } = service
        .create_new_group(name, user_uuid, member_ids)
        .await
        .map_err(HandlerError::InternalServerError)?;

    Ok(axum::Json(CreateGroupResponse { group_id: id }))
}

pub async fn list_user_groups(
    State(AppState { service }): State<SharedState>,
    Authenticated {
        claims: Claims { user_id, .. },
        ..
    }: Authenticated,
) -> Result<Json<ListUserGroupsResponse>, HandlerError> {
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| HandlerError::BadRequest("Invalid `user_id".to_string()))?;

    let groups = service
        .get_user_groups(user_uuid)
        .await
        .map_err(|e| HandlerError::InternalServerError(e))?;

    Ok(axum::Json(groups))
}
pub async fn get_group(
    State(AppState { service }): State<SharedState>,
    Authenticated {
        claims: Claims { user_id, .. },
        ..
    }: Authenticated,
    Json(GetGroupRequest { group_id }): Json<GetGroupRequest>,
) -> Result<Json<GetGroupResponse>, HandlerError> {
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| HandlerError::BadRequest("Invalid `user_id".to_string()))?;

    let group = service
        .get_group(user_uuid, group_id)
        .await
        .map_err(|e| HandlerError::InternalServerError(e))?;

    Ok(axum::Json(group))
}

pub async fn delete_group(
    State(AppState { service }): State<SharedState>,
    Authenticated {
        claims: Claims { user_id, .. },
        ..
    }: Authenticated,
    Json(DeleteGroupRequest { group_id }): Json<DeleteGroupRequest>,
) -> Result<(), HandlerError> {
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| HandlerError::BadRequest("Invalid `user_id".to_string()))?;

    service
        .delete_group(user_uuid, group_id)
        .await
        .map_err(|e| HandlerError::InternalServerError(e))?;

    Ok(())
}
