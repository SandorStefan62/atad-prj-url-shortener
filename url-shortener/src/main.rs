use sqlx::postgres::PgPoolOptions;
use tracing_subscriber;

use crate::config::Config;

mod config;
mod db;
mod error;
mod models;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let config = Config::get_env_vars()?;
    println!("Database URL = {}", config.database_url);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to database.");

    tracing::info!("Database connected successfully");

    sqlx::query("DELETE FROM urls").execute(&pool).await?;

    let created = db::queries::create_url(&pool, "https://example.com", "ex1234", None).await?;
    println!("Created: {:?}", created);

    let exists = db::queries::code_exists(&pool, "ex1234").await?;
    println!("Code exists? {}", exists);

    let fetched = db::queries::get_url_by_code(&pool, "ex1234").await?;
    println!("Fetched: {:?}", fetched);

    db::queries::increment_click(&pool, fetched.id).await?;

    let fetched = db::queries::get_url_by_code(&pool, "ex1234").await?;
    println!("Fetched: {:?}", fetched);

    let all = db::queries::list_all_urls(&pool).await?;
    println!("All urls: {:?}", all);

    Ok(())
}
