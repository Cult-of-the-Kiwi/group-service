use std::{sync::Arc, time::Duration};
use testcontainers::{ContainerAsync, GenericImage, ImageExt, core::WaitFor, runners::AsyncRunner};
use tokio::{sync::OnceCell, time::sleep};

use crate::config::Config;

static GLOBAL_CTX: OnceCell<Arc<TestContext>> = OnceCell::const_new();

pub struct TestContext {
    pub db_url: String,
    _db: ContainerAsync<GenericImage>,
    pub config: Config,
}

pub async fn ctx() -> Arc<TestContext> {
    GLOBAL_CTX
        .get_or_init(|| async {
            let ctx = setup().await;
            Arc::new(ctx)
        })
        .await
        .clone()
}

pub async fn setup() -> TestContext {
    sleep(Duration::from_secs(1)).await;

    let (db_url, db) = setup_db().await;
    let config = Config {
        db_url: db_url.clone(),
    };

    TestContext {
        db_url,
        _db: db,
        config,
    }
}

pub async fn setup_db() -> (
    std::string::String,
    testcontainers::ContainerAsync<testcontainers::GenericImage>,
) {
    let db = GenericImage::new("postgres", "16-alpine")
        .with_wait_for(WaitFor::message_on_stdout(
            "database system is ready to accept connections",
        ))
        .with_env_var("POSTGRES_USER", "postgres")
        .with_env_var("POSTGRES_PASSWORD", "password")
        .with_env_var("POSTGRES_DB", "groupdb")
        .start()
        .await
        .unwrap();

    let db_port = db.get_host_port_ipv4(5432).await.unwrap();
    let db_url = format!(
        "postgres://postgres:password@127.0.0.1:{}/groupdb?sslmode=disable",
        db_port
    );

    (db_url, db)
}
