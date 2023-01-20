pub mod status;
pub mod utils;

use status::{StatusFields, Status, continous_update};

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::http::header::ContentType;

use std::thread;
use std::sync::{Arc, RwLock};

async fn index(data: web::Data<Arc<RwLock<String>>>) -> impl Responder {
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
            .app_data(web_data.clone())
            .route("/data", web::get().to(index))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
