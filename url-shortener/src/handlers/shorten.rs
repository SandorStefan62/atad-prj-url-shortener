use crate::AppState;
use crate::db::queries;
use crate::error::{AppError, AppResult};
use crate::models::{CreateUrlRequest, CreateUrlResponse, Url};
use crate::services::shorten::{generate_unique_code, validate_custom_code, validate_url};
use axum::{Json, extract::State};

pub async fn create_short_url(
    State(state): State<AppState>,
    Json(payload): Json<CreateUrlRequest>,
) -> AppResult<Json<CreateUrlResponse>> {
    validate_url(&payload.url)?;

    let short_code = if let Some(custom_code) = payload.custom_code {
        validate_custom_code(&custom_code)?;

        if queries::code_exists(&state.db, &custom_code).await? {
            return Err(AppError::CodeAlreadyExists);
        }

        custom_code
    } else {
        generate_unique_code(&state.db, state.config.short_code_length).await?
    };

    let url = queries::create_url(&state.db, &payload.url, &short_code, payload.expires_at).await?;

    let short_url = format!("{}/{}", state.config.base_url, url.short_code);

    Ok(Json(CreateUrlResponse {
        short_url,
        short_code: url.short_code,
        original_url: url.original_url,
        expires_at: url.expires_at,
    }))
}

pub async fn list_urls(State(state): State<AppState>) -> AppResult<Json<Vec<Url>>> {
    let urls = queries::list_all_urls(&state.db).await?;
    Ok(Json(urls))
}
