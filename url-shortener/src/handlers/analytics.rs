use crate::db::queries;
use crate::error::AppResult;
use crate::services::qr_code;
use crate::{AppState, models::ClickStats};
use axum::{
    Json,
    extract::{Path, State},
    http::{StatusCode, header},
    response::IntoResponse,
};

pub async fn get_url_stats(
    State(state): State<AppState>,
    Path(short_code): Path<String>,
) -> AppResult<Json<ClickStats>> {
    let url = queries::get_url_by_code(&state.db, &short_code).await?;

    let stats = queries::get_url_stats(&state.db, url.id).await?;

    Ok(Json(stats))
}

pub async fn get_qr_code(
    State(state): State<AppState>,
    Path(short_code): Path<String>,
) -> AppResult<impl IntoResponse> {
    let _url = queries::get_url_by_code(&state.db, &short_code).await?;

    let short_url = format!("{}/{}", state.config.base_url, short_code);

    let qr_image = qr_code::generate_qr_code(&short_url);

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "image/png")],
        qr_image,
    ))
}
