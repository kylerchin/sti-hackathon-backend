use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn index(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Server", "Actix"))
        .insert_header(("Content-Type", "text/plain"))
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .body("Hello world!")
}

#[actix_web::main]
fn main() -> std::io::Result<()> {
    // Create a new HTTP server.
    let builder = HttpServer::new(|| App::new().route("/", web::get().to(index))).workers(4);

    // Bind the server to port 8080.
    let _ = builder.bind("127.0.0.1:8080").unwrap().run().await;

    Ok(())
}
