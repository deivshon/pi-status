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

async fn index(data: web::Data<Arc<RwLock<String>>>) -> impl Responder {
    let data_ref = data.read().unwrap();
    
    return HttpResponse::Ok().insert_header(ContentType::json()).body(String::from(&*data_ref));
}

async fn hello() -> impl Responder {
    format!("Hello world!")
}

pub fn continous_update(status: Arc<RwLock<Status>>, status_str: Arc<RwLock<String>>, field: StatusFields, ms: u64) {
    let mut get_func: fn() -> StatusFields;
    loop {
        match &field {
            StatusFields::Temp(_) => get_func = status::temp::get,
            StatusFields::Dummy(_) => get_func = status::dummy::get
        }


        let data = get_func();
        {
            let mut status_ref = status.write().unwrap();
            match data {
                StatusFields::Temp(t) => status_ref.temp = t,
                StatusFields::Dummy(v) => status_ref.dummy = v,
            };
        }

        {
            let status_ref = status.read().unwrap();
            let mut status_str_ref = status_str.write().unwrap();
            *status_str_ref = serde_json::to_string(&*status_ref).unwrap();
        }

        thread::sleep(time::Duration::from_millis(ms));
    }
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
        dummy: None
    }));

    // Spawn status updating threads
    let temp_ptr = Arc::clone(&data);
    let temp_ptr_str = Arc::clone(&data_str);
    thread::spawn(move || continous_update(temp_ptr, temp_ptr_str, StatusFields::Temp(None), 1000));

    let dummy_ptr = Arc::clone(&data);
    let dummy_ptr_str = Arc::clone(&data_str);
    thread::spawn(move || continous_update(dummy_ptr, dummy_ptr_str, StatusFields::Dummy(None), 1000));

    // Encapsule structure in web::Data
    let web_data = web::Data::new(data_str);

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
