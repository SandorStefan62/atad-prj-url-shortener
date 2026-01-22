use crate::db::queries;
use crate::error::{AppError, AppResult};
use nanoid::nanoid;
use sqlx::SqlitePool;

const ALPHABET: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z',
];

pub fn generate_short_code(length: usize) -> String {
    nanoid!(length, &ALPHABET)
}

pub async fn generate_unique_code(pool: &SqlitePool, length: usize) -> AppResult<String> {
    const MAX_RETRIES: u8 = 10;

    for _ in 0..MAX_RETRIES {
        let code = generate_short_code(length);
        if !queries::code_exists(pool, &code).await? {
            return Ok(code);
        }
    }

    Err(AppError::Internal(anyhow::anyhow!(
        "Failed to generate unique short code after {} attempts",
        MAX_RETRIES
    )))
}

pub fn validate_custom_code(code: &str) -> AppResult<()> {
    if code.is_empty() {
        return Err(AppError::Validation(
            "Custom code cannot be empty".to_string(),
        ));
    }

    if code.len() > 20 {
        return Err(AppError::Validation(
            "Custom code must be 20 characters or less".to_string(),
        ));
    }

    if !code.chars().all(|c| ALPHABET.contains(&c)) {
        return Err(AppError::Validation(
            "Custom code can only contain alphanumeric characters".to_string(),
        ));
    }

    Ok(())
}

pub fn validate_url(url: &str) -> AppResult<()> {
    if url.is_empty() {
        return Err(AppError::InvalidUrl);
    }

    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(AppError::Validation(
            "URL must start with 'http://' or 'https://'".to_string(),
        ));
    }

    Ok(())
}
