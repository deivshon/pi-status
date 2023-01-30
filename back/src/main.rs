pub mod status;

use status::{STATUS_STR, PROC_AND_CPU, StatusFields, continous_update};
use status::proc::PAGE_SIZE;

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::http::header::ContentType;
use actix_files::NamedFile;

use std::error::Error;
use std::path::PathBuf;
use std::thread;
use std::sync::atomic::Ordering;

fn store_page_size() {
    let mut page_size_cmd = std::process::Command::new("getconf");
        page_size_cmd.arg("PAGE_SIZE");

    let Ok(output) = page_size_cmd.output() else {return};
    let Ok(output_str) = String::from_utf8(output.stdout) else {return};
    let Ok(page_size) = output_str.trim().parse::<u64>() else {return};


    PAGE_SIZE.store(page_size, Ordering::Relaxed);
}

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
    // Get page size to compute processes' memory usage in bytes
    store_page_size();
    if PAGE_SIZE.load(Ordering::Relaxed) == 0 {
        println!("Could not get page size, processes' memory usage will not be fetched")
    }

    // Initialize the logger
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    PROC_AND_CPU.store(true, Ordering::Relaxed);

    // Spawn status updating threads
    thread::spawn(move || continous_update(StatusFields::Host(None), 5000));
    thread::spawn(move || continous_update(StatusFields::Temp(None), 1000));
    thread::spawn(move || continous_update(StatusFields::NetStats(None), 1000));
    thread::spawn(move || continous_update(StatusFields::CpuUsage(None), 1000));
    thread::spawn(move || continous_update(StatusFields::Ram(None), 1000));
    thread::spawn(move || continous_update(StatusFields::Disk(None), 1000));
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
