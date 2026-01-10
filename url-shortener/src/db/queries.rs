use crate::error::{AppError, AppResult};
use crate::models::{Click, ClickStats, CountryCount, DateCount, RefererCount, Url};

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

pub async fn record_click(
    pool: &PgPool,
    url_id: Uuid,
    ip_address: Option<String>,
    user_agent: Option<String>,
    referer: Option<String>,
) -> AppResult<Click> {
    let click = sqlx::query_as::<_, Click>(
        r#"
        INSERT INTO stats (url_id, ip_address, user_agent, referer)
        VALUES ($1, $2, $3, $4)
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

pub async fn get_url_stats(pool: &PgPool, url_id: Uuid) -> AppResult<ClickStats> {
    let total_clicks: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*) FROM stats WHERE url_id = $1
        "#,
    )
    .bind(url_id)
    .fetch_one(pool)
    .await?;
    println!("Passed");

    let unique_ips: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(DISTINCT ip_address) FROM stats WHERE url_id = $1 AND ip_address IS NOT NULL
        "#,
    )
    .bind(url_id)
    .fetch_one(pool)
    .await?;
    println!("Passed");

    let clicks_by_date: Vec<DateCount> = sqlx::query_as(
        r#"
        SELECT DATE(clicked_at) as date, COUNT(*) as count
        FROM stats
        WHERE url_id = $1
        GROUP BY DATE(clicked_at)
        ORDER BY date DESC
        LIMIT 30
        "#,
    )
    .bind(url_id)
    .fetch_all(pool)
    .await?;
    println!("Passed");

    let top_countries: Vec<CountryCount> = sqlx::query_as(
        r#"
        SELECT country, COUNT(*) as count
        FROM stats
        WHERE url_id = $1 AND country IS NOT NULL
        GROUP BY country
        ORDER BY count DESC
        LIMIT 10
        "#,
    )
    .bind(url_id)
    .fetch_all(pool)
    .await?;
    println!("Passed");

    let top_referers: Vec<RefererCount> = sqlx::query_as(
        r#"
        SELECT referer, COUNT(*) as count
        FROM stats
        WHERE url_id = $1 AND referer IS NOT NULL
        GROUP BY referer
        ORDER BY count DESC
        LIMIT 10
        "#,
    )
    .bind(url_id)
    .fetch_all(pool)
    .await?;
    println!("Passed");

    Ok(ClickStats {
        total_clicks,
        unique_ips,
        clicks_by_date,
        top_countries,
        top_referers,
    })
}
