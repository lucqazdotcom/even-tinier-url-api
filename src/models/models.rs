use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;

#[derive(Serialize, sqlx::FromRow)]
pub struct Url {
    pub id: String,
    pub short_code: String,
    pub long_url: String,
    pub created_at: NaiveDateTime,
}


#[derive(Deserialize, Debug)]
pub struct CreateRequest {
    pub long_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateResponse {
    pub long_url: String,
    pub short_code: String,
    pub short_url: String,
    pub status: i32,
}

#[derive(FromRow)]
pub struct RedirectResponse {
    pub long_url: String
}
