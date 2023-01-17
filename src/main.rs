pub mod status;

use actix_web::{web, App, HttpServer, Responder};
use serde::Serialize;

use std::thread;
use std::sync::{Arc, RwLock};

#[derive(Serialize)]
pub struct Status {
    pub temp: f32
}

async fn index(r: web::Data<Arc<RwLock<Status>>>) -> impl Responder {
    let data = r.read().unwrap();

    web::Json(Status {
        temp: data.temp
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Setup the structure
    let data = Arc::new(RwLock::new(Status {
        temp: -1.0
    }));

    // Spawn status updating threads
    let temp_ptr = Arc::clone(&data);
    thread::spawn(move || status::temp::continous_update(temp_ptr, 1000));

    // Encapsule structure in web::Data
    let web_data = web::Data::new(data);

    // Start the server
    HttpServer::new(move || {
        App::new()
            .app_data(web_data.clone())
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
