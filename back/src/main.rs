pub mod status;

use crate::status::{continous_update, STATUS_LAST, STATUS_STR};

use std::error::Error;
use std::path::PathBuf;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

use actix_files::NamedFile;
use actix_ip_filter::IPFilter;
use actix_web::http::header::ContentType;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use argparse::{ArgumentParser, Store, StoreTrue};

const FRONT_PATH: &str = "./front/pi-status-front/dist/index.html";

fn update_status_last() {
    STATUS_LAST.store(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        Ordering::Relaxed,
    );
}

async fn index() -> Result<NamedFile, Box<dyn Error>> {
    update_status_last();
    let path: PathBuf = std::fs::canonicalize(FRONT_PATH)?;

    return Ok(NamedFile::open(path)?);
}

async fn serve_data() -> impl Responder {
    update_status_last();
    let data_ref = STATUS_STR.read().unwrap();

    return HttpResponse::Ok()
        .insert_header(ContentType::json())
        .body((&*data_ref).to_owned());
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut allowed_subnets = vec![
        "127.0.0.1",
        "10.*.*.*",
        "172.{16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31}.*.*",
        "192.168.1.*",
    ];
    let mut port = 8080;
    let mut force_public = false;

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Pi-status, a minimal web resource monitor");

        ap.refer(&mut port).add_option(
            &["-p", "--port"],
            Store,
            "The port the pi-status will be run on",
        );

        ap.refer(&mut force_public).add_option(
            &["-f", "--force-public"],
            StoreTrue,
            "If set, the service will be available to everyone, not only on private subnets",
        );

        ap.parse_args_or_exit();
    }

    let mut separate_output = false;

    if force_public {
        allowed_subnets.push("*.*.*.*");
        println!(" \x1B[1;31mWARNING: the monitored resources data is now accessible to anyone, including processes names\x1B[0m");
        separate_output = true;
    }

    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    // Spawn status updating thread
    thread::spawn(move || continous_update());

    if separate_output {
        println!()
    }

    // Start the server
    HttpServer::new(move || {
        App::new()
            .wrap(IPFilter::new().allow(allowed_subnets.iter().map(|x| *x).collect()))
            .route("/data", web::get().to(serve_data))
            .route("/", web::get().to(index))
            .service(actix_files::Files::new(
                "/",
                "./front/pi-status-front/dist/",
            ))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
