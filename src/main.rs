use actix_web::{web, http, App, HttpServer};
use actix_cors::Cors;
use dotenvy;
mod db;
mod models;
mod handler;
use handler::urls::{get_urls, post_url};
use db::db::{init_pool};

use crate::handler::urls::redirect_url;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("Error fetching database url");
    let pool = init_pool(&db_url).await;

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
        .route("/urls", web::get().to(get_urls))
        .route("/new", web::post().to(post_url))
        .route("/{short_code}", web::get().to(redirect_url))
        // .route(path, route)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

