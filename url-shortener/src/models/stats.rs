use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Click {
    pub id: String,
    pub url_id: String,
    pub clicked_at: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub referer: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct DateCount {
    pub date: NaiveDate,
    pub count: i64,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct CountryCount {
    pub country: String,
    pub count: i64,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct RefererCount {
    pub referer: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct ClickStats {
    pub total_clicks: i64,
    pub unique_ips: i64,
    pub clicks_by_date: Vec<DateCount>,
    pub top_countries: Vec<CountryCount>,
    pub top_referers: Vec<RefererCount>,
}
