use crate::error::{AppError, AppResult};
use crate::models::{Click, ClickStats, CountryCount, DateCount, RefererCount, Url};

use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

pub async fn create_url(
    pool: &SqlitePool,
    original_url: &str,
    short_code: &str,
    expires_at: Option<DateTime<Utc>>,
) -> AppResult<Url> {
    //convert to string
    let expires_at_str = expires_at.map(|dt| dt.to_rfc3339());

    let url = sqlx::query_as::<_, Url>(
        r#"
        INSERT INTO urls (original_url, short_code, expires_at)
        VALUES (?, ?, ?)
        RETURNING *
        "#,
    )
    .bind(original_url)
    .bind(short_code)
    .bind(expires_at_str)
    .fetch_one(pool)
    .await?;

    Ok(url)
}

pub async fn get_url_by_code(pool: &SqlitePool, short_code: &str) -> AppResult<Url> {
    let url = sqlx::query_as::<_, Url>(
        r#"
        SELECT * FROM urls WHERE short_code = ?
        "#,
    )
    .bind(short_code)
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::UrLNotFound)?;

    Ok(url)
}
pub async fn code_exists(pool: &SqlitePool, short_code: &str) -> AppResult<bool> {
    let exists: i32 = sqlx::query_scalar(
        r#"
        SELECT EXISTS(SELECT 1 FROM urls WHERE short_code = ?)
        "#,
    )
    .bind(short_code)
    .fetch_one(pool)
    .await?;

    Ok(exists != 0)
}

pub async fn increment_click(pool: &SqlitePool, url_id: String) -> AppResult<()> {
    sqlx::query(
        r#"
        UPDATE urls SET click_count = click_count + 1 WHERE id = ?
        "#,
    )
    .bind(url_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn list_all_urls(pool: &SqlitePool) -> AppResult<Vec<Url>> {
    let urls = sqlx::query_as::<_, Url>(
        r#"
        SELECT * FROM urls ORDER BY created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(urls)
}

pub async fn record_click(
    pool: &SqlitePool,
    url_id: String,
    ip_address: Option<String>,
    user_agent: Option<String>,
    referer: Option<String>,
) -> AppResult<Click> {
    let click = sqlx::query_as::<_, Click>(
        r#"
        INSERT INTO clicks (url_id, ip_address, user_agent, referer)
        VALUES (?, ?, ?, ?)
        RETURNING *
        "#,
    )
    .bind(url_id)
    .bind(ip_address)
    .bind(user_agent)
    .bind(referer)
    .fetch_one(pool)
    .await?;

    Ok(click)
}

pub async fn get_url_stats(pool: &SqlitePool, url_id: String) -> AppResult<ClickStats> {
    let total_clicks: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*) FROM clicks WHERE url_id = ?
        "#,
    )
    .bind(&url_id)
    .fetch_one(pool)
    .await?;

    let unique_ips: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(DISTINCT ip_address) FROM clicks WHERE url_id = ? AND ip_address IS NOT NULL
        "#,
    )
    .bind(&url_id)
    .fetch_one(pool)
    .await?;

    let clicks_by_date: Vec<DateCount> = sqlx::query_as(
        r#"
        SELECT DATE(clicked_at) as date, COUNT(*) as count
        FROM clicks
        WHERE url_id = ?
        GROUP BY DATE(clicked_at)
        ORDER BY date DESC
        LIMIT 30
        "#,
    )
    .bind(&url_id)
    .fetch_all(pool)
    .await?;

    let top_countries: Vec<CountryCount> = sqlx::query_as(
        r#"
        SELECT country, COUNT(*) as count
        FROM clicks
        WHERE url_id = ? AND country IS NOT NULL
        GROUP BY country
        ORDER BY count DESC
        LIMIT 10
        "#,
    )
    .bind(&url_id)
    .fetch_all(pool)
    .await?;

    let top_referers: Vec<RefererCount> = sqlx::query_as(
        r#"
        SELECT referer, COUNT(*) as count
        FROM clicks
        WHERE url_id = ? AND referer IS NOT NULL
        GROUP BY referer
        ORDER BY count DESC
        LIMIT 10
        "#,
    )
    .bind(&url_id)
    .fetch_all(pool)
    .await?;

    Ok(ClickStats {
        total_clicks,
        unique_ips,
        clicks_by_date,
        top_countries,
        top_referers,
    })
}
