use std::sync::Arc;

use axum::Router;

use crate::{
    api::router::{self, AppState},
    application::service::GroupServiceImpl,
    infrastructure::context::db::postgres::init,
};

pub(crate) async fn build() -> anyhow::Result<Router> {
    // TODO(Sa4dUs): Maybe it's too much, but shouldn't we move this whole block away?
    let repo = init().await?;
    let bus = (); // TODO(Sa4dUs): Change it with the proper event bus
    let service = Arc::new(GroupServiceImpl::new(Arc::new(repo), Arc::new(bus)));

    let state = Arc::new(AppState { service });

    let router = router::build(state);
    Ok(router)
}

pub async fn run() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    let app = build().await?;
    axum::serve(listener, app).await?;
    Ok(())
}
