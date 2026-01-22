use crate::AppState;
use crate::db::queries;
use crate::error::{AppError, AppResult};
use axum::{
    extract::{ConnectInfo, Path, State},
    http::{HeaderMap, header},
    response::{IntoResponse, Redirect},
};
use chrono::Utc;
use std::net::SocketAddr;

pub async fn redirect(
    State(state): State<AppState>,
    Path(short_code): Path<String>,
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> AppResult<impl IntoResponse> {
    tracing::info!("Redirect requested for code: {}", short_code);

    let url = queries::get_url_by_code(&state.db, &short_code).await?;

    if let Some(expires_at) = url.expires_at {
        if Utc::now() > expires_at {
            tracing::warn!("Attempted to access expired URL: {}", short_code);
            return Err(AppError::UrlExpired);
        }
    }

    let ip_address = Some(addr.ip().to_string());
    let user_agent = headers
        .get(header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(String::from);
    let referer = headers
        .get(header::REFERER)
        .and_then(|v| v.to_str().ok())
        .map(String::from);

    let db = state.db.clone();
    let url_id = url.id;
    tokio::spawn(async move {
        let _ = queries::record_click(&db, url_id.clone(), ip_address, user_agent, referer).await;
        let _ = queries::increment_click(&db, url_id).await;
    });

    Ok(Redirect::to(&url.original_url))
}
