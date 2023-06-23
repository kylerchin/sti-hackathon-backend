use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

use postgres::{Client, NoTls};

async fn index(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Server", "Actix"))
        .insert_header(("Content-Type", "text/plain"))
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .body("Hello world!")
}

async fn addpatient(req: HttpRequest) -> impl Responder {
    let postgresstring = arguments::parse(std::env::args())
        .unwrap()
        .get::<String>("postgres")
        .unwrap() as str;

    let mut client = Client::connect(&postgresstring, NoTls);

    match client {
        Ok(c) => {
            //insert into database

            HttpResponse::Ok()
                .insert_header(("Server", "Actix"))
                .insert_header(("Content-Type", "text/plain"))
                .insert_header(("Access-Control-Allow-Origin", "*"))
                .body("Hello world!")
        }
        Err(e) => HttpRequest::InternalServerError()
            .insert_header(("Server", "Actix"))
            .insert_header(("Content-Type", "text/plain"))
            .insert_header(("Access-Control-Allow-Origin", "*"))
            .body("Postgres string not correct!"),
    }
}

#[actix_web::main]
fn main() -> std::io::Result<()> {
    let postgresstring = arguments::parse(std::env::args())
        .unwrap()
        .get::<String>("postgres")
        .unwrap();

    let mut client = Client::connect(postgresstring, NoTls).unwrap();

    client
        .batch_execute(
            "
            CREATE TABLE IF NOT EXISTS patients (
                id bigint PRIMARY KEY,
                first varchar,
                last varchar,
                region int,
                wealth int,
                age int,
                educational int,
                age_of_first int,
                working_status int,
                marital int,
                internet int,
                alcohol int,
                ethnicity int,
                sti boolean,
                sex int,
                connections bigint[],
                status boolean,
                probability decimal,
                address varchar,
                lat decimal,
                lng decimal,
                dis bigint,
                physician bigint
              )
            ",
        )
        .unwrap();

    // Create a new HTTP server.
    let builder = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/addpatient", web::get().to(addpatient))
    })
    .workers(8);

    // Bind the server to port 8080.
    let _ = builder.bind("127.0.0.1:8080").unwrap().run().await;

    Ok(())
}
