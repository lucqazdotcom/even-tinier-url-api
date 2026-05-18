use serde::Serialize;

#[derive(Serialize, sqlx::FromRow)]
pub struct Url {
    pub id: i64,
    pub short_code: String,
    pub long_url: String
}
