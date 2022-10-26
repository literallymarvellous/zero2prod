use std::net::TcpListener;
use actix_web::{HttpResponse, Responder, get, HttpServer, App, HttpRequest, dev::Server, post, web::Form};
use serde::Deserialize;

#[get("/health_check")]
async fn health_check(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
struct FormData {
  name: String,
  email: String,
}

#[post("/subscriptions")]
async fn subscribe(form: Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().body(format!("name: {}; email: {}", form.name, form.email))
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| { App::new()
        .service(health_check)
        .service(subscribe)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
