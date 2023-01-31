pub mod status;

use status::{STATUS_STR, continous_update};
use status::proc::PAGE_SIZE;

use std::path::PathBuf;
use std::thread;
use std::sync::atomic::Ordering;
use std::error::Error;

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::http::header::ContentType;
use actix_files::NamedFile;

use nix::unistd;

fn store_page_size() {
    if let Ok(Some(page_size)) = unistd::sysconf(unistd::SysconfVar::PAGE_SIZE) {
        PAGE_SIZE.store(page_size as u64, Ordering::Relaxed);
    };
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
    // only using /proc/pid/stat
    store_page_size();
    if PAGE_SIZE.load(Ordering::Relaxed) == 0 {
        eprintln!("Could not get page size, processes' memory usage will not be fetched")
    }

    // Initialize the logger
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Spawn status updating thread
    thread::spawn(move || continous_update(1000));

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
