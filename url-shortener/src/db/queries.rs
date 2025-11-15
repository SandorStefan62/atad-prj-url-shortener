use crate::error::{AppError, AppResult};
use crate::models::Url;

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_url(
    pool: &PgPool,
    original_url: &str,
    short_code: &str,
    expires_at: Option<DateTime<Utc>>,
) -> AppResult<Url> {
    let url = sqlx::query_as::<_, Url>(
        r#"
        INSERT INTO urls (original_url, short_code, expires_at)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(original_url)
    .bind(short_code)
    .bind(expires_at)
    .fetch_one(pool)
    .await?;

    Ok(url)
}

pub async fn get_url_by_code(pool: &PgPool, short_code: &str) -> AppResult<Url> {
    let url = sqlx::query_as::<_, Url>(
        r#"
        SELECT * FROM urls WHERE short_code = $1
        "#,
    )
    .bind(short_code)
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::UrLNotFound)?;

    Ok(url)
}

pub async fn code_exists(pool: &PgPool, short_code: &str) -> AppResult<bool> {
    let exists: bool = sqlx::query_scalar(
        r#"
        SELECT EXISTS(SELECT 1 FROM urls WHERE short_code = $1)
        "#,
    )
    .bind(short_code)
    .fetch_one(pool)
    .await?;

    Ok(exists)
}

pub async fn increment_click(pool: &PgPool, url_id: Uuid) -> AppResult<()> {
    sqlx::query(
        r#"
        UPDATE urls SET click_count = click_count + 1 WHERE id = $1
        "#,
    )
    .bind(url_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn list_all_urls(pool: &PgPool) -> AppResult<Vec<Url>> {
    let urls = sqlx::query_as::<_, Url>(
        r#"
        SELECT * FROM urls ORDER BY created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(urls)
}
