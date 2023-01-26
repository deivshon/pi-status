pub mod status;
pub mod utils;

use status::{STATUS_STR, StatusFields, continous_update};

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::http::header::ContentType;
use actix_files::NamedFile;

use std::error::Error;
use std::path::PathBuf;
use std::thread;


async fn index() -> Result<NamedFile, Box<dyn Error>> {
    let path: PathBuf = std::fs::canonicalize("./front/pi-status-front/dist/index.html")?;
    
    return Ok(NamedFile::open(path)?)
}

async fn serve_data() -> impl Responder {
    let data_ref = STATUS_STR.read().unwrap();

    return HttpResponse::Ok().insert_header(ContentType::json()).body((&*data_ref).to_owned());
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Spawn status updating threads
    thread::spawn(move || continous_update(StatusFields::Temp(None), 1000));
    thread::spawn(move || continous_update(StatusFields::NetStats(None), 1000));
    thread::spawn(move || continous_update(StatusFields::CpuUsage(None), 1000));
    thread::spawn(move || continous_update(StatusFields::Ram(None), 1000));
    thread::spawn(move || continous_update(StatusFields::Proc(None), 1000));

    // Start the server
    HttpServer::new(move || {
        App::new()
            .route("/data", web::get().to(serve_data))
            .route("/", web::get().to(index))
            .service(actix_files::Files::new("/", "./front/pi-status-front/dist/"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
