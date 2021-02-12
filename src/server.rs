use actix_web::{web, App, Error, HttpResponse, HttpServer};

async fn handle_poll(body: web::Bytes) -> Result<HttpResponse, Error> {
    dbg!(body);
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body("ok"))
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(web::resource("/poll").route(web::post().to(handle_poll)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
