use std::sync::Arc;

use axum::{
    Extension, Router,
    routing::{delete, get, post},
};

use crate::{
    api::handlers::{
        channels::handler::get_group_by_channel,
        groups::{
            handler::{create_group, delete_group, get_group, list_user_groups},
            members::handler::{add_group_members, list_group_members, remove_group_members},
        },
        health::health_check,
    },
    application::service::GroupService,
};

// FIXME(Sa4dUs): Move this away
pub type SharedService = Arc<dyn GroupService + Send + Sync + 'static>;

pub struct AppState {
    pub service: SharedService,
}
pub type SharedState = Arc<AppState>;

pub fn build(state: SharedState) -> Router {
    Router::new()
        .merge(unprotected(state.clone()))
        .merge(protected(state.clone()))
}

fn protected(state: SharedState) -> Router {
    Router::new()
        .route("/groups", post(create_group).get(list_user_groups))
        .route("/groups/{group_id}", get(get_group).delete(delete_group))
        .route(
            "/groups/{group_id}/members",
            get(list_group_members).post(add_group_members),
        )
        .route(
            "/groups/{group_id}/members/{user_id}",
            delete(remove_group_members),
        )
        .route("/channels/{channel_id}/group", get(get_group_by_channel))
        .with_state(state)
}

fn unprotected(state: SharedState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .with_state(state)
}
