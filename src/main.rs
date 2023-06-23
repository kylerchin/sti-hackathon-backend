use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use tokio_postgres::{NoTls, Error};
use tokio;

async fn index(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Server", "Actix"))
        .insert_header(("Content-Type", "text/plain"))
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .body("Hello world!")
}

#[derive(serde::Deserialize, Debug)]
struct PatientInit {
    first: String,
    last: String,
    region: i32,
    wealth: i32,
    age: i32,
    education: i32,
    age_of_first: i32,
    working_status: i32,
    marital: i32,
    internet: i32,
    alcohol: i32,
    ethnicity: i32,
    sti: bool,
    sex: i32,
    connections: Option<Vec<i64>>,
    address: String,
    dis: Option<i64>,
    physician: i64,
}

async fn addpatient(info: web::Json<PatientInit>) -> impl Responder {
    let postgresstring = arguments::parse(std::env::args())
        .unwrap()
        .get::<String>("postgres")
        .unwrap();

    // Connect to the database.
   /*
    let (client, connection) =
        tokio_postgres::connect(&postgresstring, NoTls).await.unwrap();

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
     */

            //insert into database

            println!("{:?}", info);

            HttpResponse::Ok()
                .insert_header(("Server", "Actix"))
                .insert_header(("Content-Type", "text/plain"))
                .insert_header(("Access-Control-Allow-Origin", "*"))
                .body("Hello world!")
       
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let postgresstring = arguments::parse(std::env::args())
        .unwrap()
        .get::<String>("postgres")
        .unwrap();

    // Connect to the database.
    let (client, connection) =
        tokio_postgres::connect(&postgresstring, NoTls).await.unwrap();

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

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
                education int,
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
        ).await.unwrap();

    // Create a new HTTP server.
    let builder = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/addpatient", web::post().to(addpatient))
    })
    .workers(8);

    // Bind the server to port 8080.
    let _ = builder.bind("127.0.0.1:8080").unwrap().run().await;

    Ok(())
}
