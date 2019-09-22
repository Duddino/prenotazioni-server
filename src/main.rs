use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Login {
    name: String,
    password: String
}

fn index(request: web::Query<Login>) -> impl Responder {
    HttpResponse::Ok().body(format!("HAI INSERITO IL NOME {} E LA PASSWORD {}", request.name, request.password))
}

fn main() -> actix_web::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8000")?
    .run()?;
    Ok(())
}
