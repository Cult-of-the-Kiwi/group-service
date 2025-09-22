use axum::{Extension, Json, extract::State, response::IntoResponse};
use devcord_middlewares::{middlewares::auth::Authenticated, models::claims::Claims};

use crate::{
    api::router::{SharedService, SharedState},
    application::{dto::CreateGroupRequest, service::GroupService},
};

pub async fn create_group(
    State(state): State<SharedState>,
    Authenticated {
        claims: Claims { user_id, .. },
        ..
    }: Authenticated,
    Json(payload): Json<CreateGroupRequest>,
) -> impl IntoResponse {
    if payload.member_ids.is_empty() {
        // TODO(Sa4dUs): Throw error
    }
}

pub async fn list_user_groups() -> impl IntoResponse {}
pub async fn get_group() -> impl IntoResponse {}
pub async fn delete_group() -> impl IntoResponse {}
