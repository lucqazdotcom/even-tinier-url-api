use core::str;
use sqlx::{sqlite::{SqliteConnectOptions, SqlitePool}};
use std::str::FromStr;
use uuid::Uuid;
use crate::models::models::{RedirectResponse, Url};

pub async fn init_pool(url: &str) -> SqlitePool {
    let opts = SqliteConnectOptions::from_str(url).expect("something").create_if_missing(true);
    let pool = SqlitePool::connect_with(opts).await.expect("failed to connect to db");
    sqlx::migrate!("src/db/migrations").run(&pool).await.expect("failed to migrate");
    pool
}

pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Url>, sqlx::Error> {
    let urls = sqlx::query_as::<_, Url>("SELECT * FROM urls")
        .fetch_all(pool)
        .await?;
    Ok(urls)
}

pub async fn insert_url(pool: &SqlitePool, short_code: &str, long_url: &str) -> Result<Url, sqlx::Error> {
    let id = Uuid::new_v4().to_string();

    let url = sqlx::query_as::<_, Url>("INSERT INTO urls (id, short_code, long_url) VALUES (?, ?, ?) RETURNING *")
        .bind(id)
        .bind(short_code)
        .bind(long_url)
        .fetch_one(pool)
        .await?;

    Ok(url)
}

pub async fn get_long_by_short(pool: &SqlitePool, short_code: &str) -> Result<Url, sqlx::Error> {
    let url = sqlx::query_as::<_, Url>("SELECT * FROM urls WHERE short_code = ?")
        .bind(short_code)
        .fetch_one(pool)
        .await?;
    Ok(url)
}

pub async fn get_redirect_by_short(pool: &SqlitePool, short_code: &str) -> Result<RedirectResponse, sqlx::Error> {
    let url = sqlx::query_as::<_, RedirectResponse>("SELECT long_url FROM urls WHERE short_code = ?")
        .bind(short_code)
        .fetch_one(pool)
        .await?;

    Ok(url)
}
