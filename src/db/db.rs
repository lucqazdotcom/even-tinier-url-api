use core::str;
use serde::Serialize;
use sqlx::{sqlite::{SqliteConnectOptions, SqlitePool}, types::chrono::{NaiveDateTime}};
use std::str::FromStr;

const URL: &str = "sqlite:tinyurl.db";

pub async fn init_pool(url: &str) -> SqlitePool {
    let opts = SqliteConnectOptions::from_str(url).expect("something").create_if_missing(true);
    let pool = SqlitePool::connect_with(opts).await.expect("failed to connect to db");
    sqlx::migrate!("src/db/migrations").run(&pool).await.expect("failed to migrate");
    pool
}

#[derive(sqlx::FromRow, Serialize)]
pub struct Urls {
    pub id: i64,
    pub short_code: String,
    pub long_url: String,
    pub created_at: NaiveDateTime,
}

pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Urls>, sqlx::Error> {
    let urls = sqlx::query_as::<_, Urls>("SELECT * FROM urls")
        .fetch_all(pool)
        .await?;
    Ok(urls)
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
