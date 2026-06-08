use actix_web::{HttpResponse, Responder, web};
use sqlx::{self, SqlitePool};
use uuid::Uuid;
use crate::db::db::{get_all, insert_url, get_long_by_short, get_redirect_by_short};
use crate::models::models::{CreateResponse, CreateRequest};

pub async fn get_urls(pool: web::Data<SqlitePool>) -> impl Responder {
    match get_all(pool.get_ref()).await {
        Ok(urls) => HttpResponse::Ok().json(urls),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string())
    }
}

pub async fn post_url(
    pool: web::Data<SqlitePool>,
    body: web::Json<CreateRequest>
    ) -> impl Responder {

    let short_code = Uuid::new_v4().to_string()[..6].to_string();

    match insert_url(pool.get_ref(), &short_code, &body.long_url).await {
        Ok(response) => HttpResponse::Ok().json(CreateResponse {
            long_url: response.long_url,
            short_url: format!("localhost:8080/{}", response.short_code),
            short_code: response.short_code,
            status: 200,
        }),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string())
    }

}

pub async fn retrieve_url(
    pool: web::Data<SqlitePool>,
    path: web::Path<String>
) -> impl Responder {
    let short_code = path.into_inner();

    match get_long_by_short(pool.get_ref(), &short_code).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string())
    }
}

pub async fn redirect_url(
    pool: web::Data<SqlitePool>,
    path: web::Path<String>
) -> impl Responder{
    let short_code = path.into_inner();

    match get_redirect_by_short(pool.get_ref(), &short_code).await {
        Ok(url) => HttpResponse::Found()
            .insert_header(("Location", url.long_url))
            .finish(),

        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }

}
