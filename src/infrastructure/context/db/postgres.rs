use sqlx::{PgPool, migrate::Migrator, postgres::PgPoolOptions};

use crate::{
    config::{Config, GLOBAL_CONFIG},
    domain::repositories::group_repository::GroupRepository,
    types::Async,
};

const MIGRATOR: Migrator = sqlx::migrate!();

pub async fn init() -> anyhow::Result<PgPool> {
    let Config { db_url, .. } = &*GLOBAL_CONFIG;

    let pool = PgPoolOptions::new().connect(db_url).await?;
    MIGRATOR.run(&pool).await?;

    Ok(pool)
}

impl GroupRepository for PgPool {
    fn foo(&self) -> Async<'_, Result<(), crate::domain::error::DomainError>> {
        todo!()
    }
}
