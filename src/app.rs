use std::sync::Arc;

use axum::Router;

use crate::{
    api::router::{self, AppState},
    application::service::GroupServiceImpl,
    config::Config,
    infrastructure::context::db::postgres::init,
};

pub(crate) async fn build(config: Option<Config>) -> anyhow::Result<Router> {
    // TODO(Sa4dUs): Maybe it's too much, but shouldn't we move this whole block away?
    let cfg = Config::get(config).await;
    let repo = init((*cfg).clone()).await?;
    let bus = (); // TODO(Sa4dUs): Change it with the proper event bus
    let service = Arc::new(GroupServiceImpl::new(Arc::new(repo), Arc::new(bus)));

    let state = AppState { service };

    let router = router::build(state);
    Ok(router)
}

pub async fn run() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    let app = build(None).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
