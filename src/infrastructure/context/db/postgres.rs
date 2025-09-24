use sqlx::{PgPool, migrate::Migrator, postgres::PgPoolOptions};

use crate::config::Config;

const MIGRATOR: Migrator = sqlx::migrate!();

pub async fn init(config: Config) -> anyhow::Result<PgPool> {
    let Config { db_url, .. } = config;

    let pool = PgPoolOptions::new().connect(&db_url).await?;
    MIGRATOR.run(&pool).await?;

    Ok(pool)
}
