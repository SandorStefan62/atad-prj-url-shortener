use std::env;

use anyhow::Ok;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub short_code_length: usize,
    pub base_url: String,
}

impl Config {
    pub fn get_env_vars() -> anyhow::Result<Self> {
        Ok(Self {
            database_url: env::var("DATABASE_URL")?,
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()?,
            short_code_length: env::var("SHORT_CODE_LENGTH")
                .unwrap_or_else(|_| "6".to_string())
                .parse()?,
            base_url: env::var("BASE_URL")?,
        })
    }
}
