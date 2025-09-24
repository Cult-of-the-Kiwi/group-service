use anyhow::Result;
use std::sync::Arc;
use tokio::sync::OnceCell;

#[derive(Clone)]
pub struct Config {
    pub db_url: String,
}

static GLOBAL_CONFIG: OnceCell<Arc<Config>> = OnceCell::const_new();

impl Config {
    pub async fn get(config: Option<Config>) -> Arc<Config> {
        GLOBAL_CONFIG
            .get_or_init(|| async {
                Arc::new(Config::load(config).expect("Failed to load Config"))
            })
            .await
            .clone()
    }

    pub fn load(config: Option<Config>) -> Result<Self> {
        if let Some(cfg) = config {
            return Ok(cfg);
        }

        #[cfg(test)]
        return Self::load_test();

        #[cfg(not(test))]
        return Self::load_prod();
    }

    #[cfg(test)]
    fn load_test() -> Result<Self> {
        Ok(Config {
            db_url: "postgres://postgres:password@127.0.0.1:5432/groupdb".to_string(),
        })
    }

    #[cfg(not(test))]
    fn load_prod() -> Result<Self> {
        Ok(Config {
            db_url: std::env::var("GROUP_DATABASE_URL").unwrap_or_else(|_| {
                "postgres://postgres:password@127.0.0.1:5432/groupdb".to_string()
            }),
        })
    }
}
