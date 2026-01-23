use std::{net::SocketAddr, path::Path, sync::Arc};

use axum::{
    Router,
    routing::{get, post},
};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};
use tracing_subscriber;

use crate::{config::Config, services::rate_limiter::RateLimiter};

mod config;
mod db;
mod error;
mod handlers;
mod models;
mod services;
mod templates;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::SqlitePool,
    pub config: Arc<Config>,
    pub rate_limiter: Arc<RateLimiter>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let config = Arc::new(Config::get_env_vars()?);

    // create if not existant sqlite db file
    let db_path = config
        .database_url
        .strip_prefix("sqlite:")
        .unwrap_or("url_shortener.db");

    if !Path::new(db_path).exists() {
        tracing::info!(
            "Databasae file not found. Created new database at {}",
            db_path
        );
    } else {
        tracing::info!("Using existing database at: {}", db_path);
    }

    let connecting_options = SqliteConnectOptions::new()
        .filename(db_path)
        .create_if_missing(true);

    let db = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connecting_options)
        .await
        .expect("Failed to connect to database.");

    tracing::info!("Database connected successfully");

    sqlx::migrate!("./migrations").run(&db).await?;
    tracing::info!("Migrations completed successfully");

    let rate_limiter = Arc::new(RateLimiter::new(config.requests_per_minute));

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
        rate_limiter,
    };

    let app = Router::new()
        .route("/", get(handlers::web::index))
        .route("/dashboard", get(handlers::web::dashboard))
        .route("/api/urls", get(handlers::shorten::list_urls))
        .route("/api/shorten", post(handlers::shorten::create_short_url))
        .route(
            "/api/urls/:short_code",
            get(handlers::analytics::get_url_stats),
        )
        .route(
            "/api/urls/:short_code/qr",
            get(handlers::analytics::get_qr_code),
        )
        .route("/:short_code", get(handlers::redirect::redirect))
        .nest_service("/static", ServeDir::new("static"))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let address = format!("{}:{}", config.server_host, config.server_port);
    let listener = tokio::net::TcpListener::bind(&address).await?;

    tracing::info!("Server running on http://{}", address);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}
