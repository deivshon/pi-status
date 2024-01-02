pub mod status;

use crate::status::{ACTIVE_WS_CONNECTIONS, STATUS_STR};

use std::error::Error;
use std::path::PathBuf;
use std::sync::atomic::Ordering;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

use actix::{Actor, AsyncContext, StreamHandler};
use actix_files::NamedFile;
use actix_ip_filter::IPFilter;
use actix_web::{web, App, Error as ActixError, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws::{self, Message, ProtocolError};
use argparse::{ArgumentParser, Store, StoreTrue};

const FRONT_PATH: &str = "./front/pi-status-front/dist/index.html";

async fn index() -> Result<NamedFile, Box<dyn Error>> {
    let path: PathBuf = std::fs::canonicalize(FRONT_PATH)?;

    Ok(NamedFile::open(path)?)
}

async fn serve_data(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, ActixError> {
    ACTIVE_WS_CONNECTIONS.fetch_add(1, Ordering::Relaxed);
    ws::start(
        WsDataSession {
            data: STATUS_STR.clone(),
        },
        &req,
        stream,
    )
}

struct WsDataSession {
    data: Arc<RwLock<String>>,
}

impl StreamHandler<Result<Message, ProtocolError>> for WsDataSession {
    fn handle(&mut self, msg: Result<Message, ProtocolError>, _: &mut Self::Context) {
        match msg {
            Ok(m) => match m {
                Message::Close(_) => {
                    ACTIVE_WS_CONNECTIONS.fetch_sub(1, Ordering::Relaxed);
                }
                _ => (),
            },
            Err(e) => {
                eprintln!("Error occurred in WS receive operation: {}", e)
            }
        }
    }
}

impl Actor for WsDataSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_later(Duration::ZERO, |act, ctx| {
            let data = act.data.read().unwrap().clone();
            ctx.text(data);
        });

        ctx.run_interval(Duration::from_secs(1), |act, ctx| {
            let data = act.data.read().unwrap().clone();
            ctx.text(data);
        });
    }
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

    thread::spawn(move || status::continous_update());

    if separate_output {
        println!()
    }

    HttpServer::new(move || {
        App::new()
            .wrap(IPFilter::new().allow(allowed_subnets.iter().map(|x| *x).collect()))
            .service(web::resource("/ws_data").to(serve_data))
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
