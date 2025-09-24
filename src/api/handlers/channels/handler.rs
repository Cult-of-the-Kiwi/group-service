use axum::{Json, extract::State};
use devcord_middlewares::{middlewares::auth::Authenticated, models::claims::Claims};
use uuid::Uuid;

use crate::api::{
    dto::{GetGroupByChannelRequest, GetGroupByChannelResponse},
    error::HandlerError,
    router::{AppState, SharedState},
};

pub async fn get_group_by_channel(
    State(AppState { service }): State<SharedState>,
    Authenticated {
        claims: Claims { user_id, .. },
        ..
    }: Authenticated,
    Json(GetGroupByChannelRequest { channel_id }): Json<GetGroupByChannelRequest>,
) -> Result<Json<GetGroupByChannelResponse>, HandlerError> {
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| HandlerError::BadRequest("Invalid `user_id".to_string()))?;

    let group = service
        .get_by_channel(user_uuid, channel_id)
        .await
        .map_err(|e| HandlerError::InternalServerError(e))?;
    Ok(axum::Json(group))
}
