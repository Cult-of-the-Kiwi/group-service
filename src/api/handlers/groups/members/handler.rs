use axum::{Json, extract::State, response::IntoResponse};
use devcord_middlewares::{middlewares::auth::Authenticated, models::claims::Claims};
use uuid::Uuid;

use crate::api::{
    dto::{
        AddGroupMembersRequest, ListGroupMembersRequest, ListGroupMembersResponse,
        RemoveGroupMembersRequest,
    },
    error::HandlerError,
    router::{AppState, SharedState},
};

pub async fn list_group_members(
    State(AppState { service }): State<SharedState>,
    Authenticated {
        claims: Claims { user_id, .. },
        ..
    }: Authenticated,
    Json(ListGroupMembersRequest { group_id }): Json<ListGroupMembersRequest>,
) -> Result<Json<ListGroupMembersResponse>, HandlerError> {
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| HandlerError::BadRequest("Invalid `user_id".to_string()))?;

    let members = service
        .get_members(user_uuid, group_id)
        .await
        .map_err(|e| HandlerError::InternalServerError(e))?;

    Ok(axum::Json(members))
}
pub async fn add_group_members(
    State(AppState { service }): State<SharedState>,
    Authenticated {
        claims: Claims { user_id, .. },
        ..
    }: Authenticated,
    Json(AddGroupMembersRequest {
        group_id,
        member_ids,
    }): Json<AddGroupMembersRequest>,
) -> Result<(), HandlerError> {
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| HandlerError::BadRequest("Invalid `user_id".to_string()))?;

    service
        .add_members(user_uuid, group_id, member_ids)
        .await
        .map_err(|e| HandlerError::InternalServerError(e))?;
    Ok(())
}
pub async fn remove_group_members(
    State(AppState { service }): State<SharedState>,
    Authenticated {
        claims: Claims { user_id, .. },
        ..
    }: Authenticated,
    Json(RemoveGroupMembersRequest {
        group_id,
        member_ids,
    }): Json<RemoveGroupMembersRequest>,
) -> Result<(), HandlerError> {
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| HandlerError::BadRequest("Invalid `user_id".to_string()))?;

    service
        .remove_members(user_uuid, group_id, member_ids)
        .await
        .map_err(|e| HandlerError::InternalServerError(e))?;
    Ok(())
}
