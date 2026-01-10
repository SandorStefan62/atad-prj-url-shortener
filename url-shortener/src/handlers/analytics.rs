use crate::db::queries;
use crate::error::AppResult;
use crate::{AppState, models::ClickStats};
use axum::{
    Json,
    extract::{Path, State},
};

pub async fn get_url_stats(
    State(state): State<AppState>,
    Path(short_code): Path<String>,
) -> AppResult<Json<ClickStats>> {
    let url = queries::get_url_by_code(&state.db, &short_code).await?;

    let stats = queries::get_url_stats(&state.db, url.id).await?;

    Ok(Json(stats))
}
