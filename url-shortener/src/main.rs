use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use sqlx::postgres::PgPoolOptions;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};
use tracing_subscriber;

use crate::config::Config;

mod config;
mod db;
mod error;
mod handlers;
mod models;
mod services;
mod templates;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub config: Arc<Config>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let config = Arc::new(Config::get_env_vars()?);

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to database.");

    tracing::info!("Database connected successfully");

    let code = services::shorten::generate_short_code(6);
    println!("Short code: {}", code);

    let unique_code = services::shorten::generate_unique_code(&db, 6).await?;
    println!("Unique short code: {:?}", unique_code);

    println!(
        "Is custom code valid: {:?}",
        services::shorten::validate_custom_code("valid")
    );

    println!(
        "Is custom code valid: {:?}",
        services::shorten::validate_custom_code("invalid243953945309546")
    );

    println!(
        "Is custom code valid: {:?}",
        services::shorten::validate_custom_code("")
    );

    println!(
        "Is custom code valid: {:?}",
        services::shorten::validate_custom_code(";")
    );

    println!(
        "Is url valid: {:?}",
        services::shorten::validate_url("http://www.example.com")
    );

    println!(
        "Is url valid: {:?}",
        services::shorten::validate_url("www.example.com")
    );
    println!("Is url valid: {:?}", services::shorten::validate_url(""));

    let state = AppState {
        db,
        config: config.clone(),
    };

    let app = Router::new()
        .route("/", get(handlers::web::index))
        .route("/dashboard", get(handlers::web::dashboard))
        .route("/api/urls", get(handlers::shorten::list_urls))
        .route("/api/shorten", post(handlers::shorten::create_short_url))
        .nest_service("/static", ServeDir::new("static"))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let address = format!("{}:{}", config.server_host, config.server_port);
    let listener = tokio::net::TcpListener::bind(&address).await?;

    tracing::info!("Server running on http://{}", address);

    axum::serve(listener, app).await?;

    Ok(())
}
