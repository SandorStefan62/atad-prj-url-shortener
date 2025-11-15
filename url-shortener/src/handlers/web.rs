use crate::AppState;
use crate::db::queries;
use crate::error::AppResult;
use crate::templates::{DashboardTemplate, IndexTemplate};
use askama_axum::IntoResponse;
use axum::extract::State;

pub async fn index() -> impl IntoResponse {
    IndexTemplate {}
}

pub async fn dashboard(State(state): State<AppState>) -> AppResult<impl IntoResponse> {
    let urls = queries::list_all_urls(&state.db).await?;
    Ok(DashboardTemplate { urls })
}
