use actix_web::{web, http, get, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde::{Serialize, Deserialize};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
            let cors = Cors::default()
                .allowed_origin("http://localhost:8000")
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600);
        App::new()
        .wrap(cors)
        .route("/", web::get().to(index))
        .service(manual_hello)
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

#[get["/hey"]]
async fn manual_hello() -> HttpResponse {

    let body = MessageBdy {status: 200, message: "hey there".to_string()};

    let serialized = serde_json::to_string(&body).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .json(serialized)
}
