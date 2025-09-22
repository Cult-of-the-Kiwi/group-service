use anyhow::Result;
use lazy_static::lazy_static;

#[cfg(not(test))]
use std::env;

lazy_static! {
    pub static ref GLOBAL_CONFIG: Config = {
        // TODO(Sa4dUs): How should we handle this `Result`?
        Config::load().unwrap()
    };
}

pub struct Config {
    pub db_url: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        #[cfg(test)]
        return Self::load_test();
        #[cfg(not(test))]
        return Self::load_prod();
    }

    #[cfg(test)]
    fn load_test() -> Result<Self> {
        let db_url = "postgres://postgres:password@group-db:5432/groupdb".to_string();
        Ok(Config { db_url })
    }

    #[cfg(not(test))]
    fn load_prod() -> Result<Self> {
        let db_url = env::var("GROUP_DATABASE_URL")?;
        Ok(Config { db_url })
    }
}
