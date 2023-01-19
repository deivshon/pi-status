pub mod status;

use actix_web::{web, App, HttpServer, Responder, HttpResponse, HttpRequest};
use actix_web::http::header::ContentType;
use serde::Serialize;
use serde_json;
use status::dummy::DummyStruct;

use std::thread;
use std::sync::{Arc, RwLock};
use std::time;

#[derive(Serialize)]
pub struct Status {
    pub temp: Option<f32>,
    pub dummy: Option<status::dummy::DummyStruct>
}

pub enum StatusFields {
    Temp(Option<f32>),
    Dummy(Option<DummyStruct>)
}

async fn index(r: web::Data<Arc<RwLock<Status>>>) -> impl Responder {
    let data = r.read().unwrap();
    let obj = serde_json::to_string(&*data).unwrap();
    return HttpResponse::Ok().insert_header(ContentType::json()).body(obj);
}

async fn hello() -> impl Responder {
    format!("Hello world!")
}

pub fn continous_update(status: Arc<RwLock<Status>>, field: StatusFields, ms: u64) {
    let mut get_func: fn() -> StatusFields;
    loop {
        match &field {
            StatusFields::Temp(_) => get_func = status::temp::get,
            StatusFields::Dummy(_) => get_func = status::dummy::get
        }


        let data = get_func();
        {
            let mut d = status.write().unwrap();
            match data {
                StatusFields::Temp(t) => d.temp = t,
                StatusFields::Dummy(v) => d.dummy = v,
            };
        }

        thread::sleep(time::Duration::from_millis(ms));
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Setup the structure
    let data = Arc::new(RwLock::new(Status {
        temp: None,
        dummy: None
    }));

    // Spawn status updating threads
    let temp_ptr = Arc::clone(&data);
    thread::spawn(move || continous_update(temp_ptr, StatusFields::Temp(None), 1000));

    let dummy_ptr = Arc::clone(&data);
    thread::spawn(move || continous_update(dummy_ptr, StatusFields::Dummy(None), 1000));

    // Encapsule structure in web::Data
    let web_data = web::Data::new(data);

    // Start the server
    HttpServer::new(move || {
        App::new()
            .app_data(web_data.clone())
            .route("/", web::get().to(index))
            .route("/hello", web::get().to(hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
