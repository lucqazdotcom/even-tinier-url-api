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

pub async fn get_long_by_short(pool: &SqlitePool, short_code: &str) -> Result<RedirectResponse, sqlx::Error> {
    let url = sqlx::query_as::<_, RedirectResponse>("SELECT long_url FROM urls WHERE short_code = ?")
        .bind(short_code)
        .fetch_one(pool)
        .await?;

    Ok(url)
}
// pub async fn init_db(){
//     if !Sqlite::database_exists(URL).await.unwrap_or(false) {
//         println!("creating db");
//         match Sqlite::create_database(URL).await {
//             Ok(_) => println!("created db my guy"),
//             Err(error) => panic!("error: {}", error),
//         }
//     }
//     else {
//         println!("this db already exists")
//     }
//
//
//     run_migration(&db).await;
// }
//
// async fn run_migration(pool: &SqlitePool) {
//     sqlx::query(
//         "CREATE TABLE IF NOT EXISTS urls (
//             id  INTEGER PRIMARY KEY AUTOINCREMENT,
//             short_code TEXT UNIQUE NOT NULL,
//             long_url TEXT NOT NULL,
//             created_at DATETIME DEFAULT CURRENT_TIMESTAMP
//         )"
//     )
//     .execute(pool)
//     .await
//     .expect("Failed to run migration");
// }
