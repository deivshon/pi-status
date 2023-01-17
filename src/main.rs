pub mod status;

use actix_web::{web, App, HttpServer, Responder};
use serde::Serialize;

use std::{thread, time};
use std::sync::{Arc, RwLock};

#[derive(Serialize)]
struct Status {
    temp: f32
}

async fn index(r: web::Data<Arc<RwLock<Status>>>) -> impl Responder {
    let data = r.read().unwrap();

    web::Json(Status {
        temp: data.temp
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let data = Arc::new(RwLock::new(Status {
        temp: 0.0
    }));

    let data_clone = Arc::clone(&data);
    
    thread::spawn(move || {
        loop {
            match status::temp::get() {
                Ok(t) => {
                    let mut d = data_clone.write().unwrap();
                    d.temp = t;
                },
                Err(_) => ()
            };

            thread::sleep(time::Duration::from_millis(1000));
        }
    });

    // Note: web::Data created _outside_ HttpServer::new closure
    let web_data = web::Data::new(data);

    HttpServer::new(move || {
        // move counter into the closure
        App::new()
            .app_data(web_data.clone()) // <- register the created data
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

