use std::env;

use anyhow::Ok;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    // to add more if needed
}

impl Config {
    pub fn get_env_vars() -> anyhow::Result<Self> {
        Ok(Self {
            database_url: env::var("DATABASE_URL")?,
        })
    }
}
