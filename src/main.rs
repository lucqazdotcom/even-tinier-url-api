use actix_web::{web, http, get, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde::{Serialize, Deserialize};
use tokio;
use sqlx::{self, SqlitePool};
mod db;
use db::db::{init_pool, get_all};

const URL: &str = "sqlite:tinyurl.db";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = init_pool(URL).await;
    HttpServer::new(move ||{
        let cors = Cors::default()
            .allowed_origin("http://localhost:8000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
        .wrap(cors)
        .app_data(web::Data::new(pool.clone()))
        .route("/hey", web::get().to(manual_hello))
        // .configure(routes::configure)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("hello")
}

#[derive(Serialize, Deserialize, Debug)]
struct MessageBdy {
    status: i32,
    message: String
}

async fn manual_hello(pool: web::Data<SqlitePool>) -> impl Responder {
    match get_all(pool.get_ref()).await {
        Ok(urls) => HttpResponse::Ok().json(urls),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string())
    }
}

