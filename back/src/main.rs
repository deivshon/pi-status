pub mod status;
pub mod utils;

use status::{StatusFields, Status, continous_update};

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::http::header::ContentType;
use actix_files::NamedFile;

use std::thread;
use std::sync::{Arc, RwLock};
use std::path::PathBuf;

use std::error::Error;

async fn index() -> Result<NamedFile, Box<dyn Error>> {
    let path: PathBuf = std::fs::canonicalize("./front/pi-status-front/dist/index.html")?;
    
    return Ok(NamedFile::open(path)?)
}

async fn serve_data(data: web::Data<Arc<RwLock<String>>>) -> impl Responder {
    let data_ref = data.read().unwrap();

    return HttpResponse::Ok().insert_header(ContentType::json()).body((&*data_ref).to_owned());
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let data_str: Arc<RwLock<String>> = Arc::new(RwLock::new(String::new()));
    // Setup the structure
    let data = Arc::new(RwLock::new(Status {
        temp: None,
        net_stats: None
    }));

    // Spawn status updating threads
    let temp_ptr = Arc::clone(&data);
    let temp_ptr_str = Arc::clone(&data_str);
    thread::spawn(move || continous_update(temp_ptr, temp_ptr_str, StatusFields::Temp(None), 500));

    let net_ptr = Arc::clone(&data);
    let net_ptr_str = Arc::clone(&data_str);
    thread::spawn(move || continous_update(net_ptr, net_ptr_str, StatusFields::NetStats(None), 1000));

    // Encapsule structure in web::Data
    let web_data = web::Data::new(data_str);

    // Start the server
    HttpServer::new(move || {
        App::new()
            .route("/data", web::get().to(serve_data))
            .route("/", web::get().to(index))
            .service(actix_files::Files::new("/", "./front/pi-status-front/dist/"))
            .app_data(web_data.clone())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
