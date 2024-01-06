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
use clap::Parser;
use log::{error, warn};

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
                error!("Error occurred in WS receive operation: {}", e)
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

#[derive(Parser)]
#[command(about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
    #[arg(short, long, default_value_t = false)]
    force_public: bool,
    #[arg(short = 'c', long, default_value_t = false)]
    suppress_cpu_errors: bool,
    #[arg(short = 'd', long, default_value_t = false)]
    suppress_disk_errors: bool,
    #[arg(short = 'o', long, default_value_t = false)]
    suppress_host_errors: bool,
    #[arg(short = 'n', long, default_value_t = false)]
    suppress_net_errors: bool,
    #[arg(short = 'p', long, default_value_t = false)]
    suppress_proc_errors: bool,
    #[arg(short = 'n', long, default_value_t = false)]
    suppress_ram_errors: bool,
    #[arg(short = 't', long, default_value_t = false)]
    suppress_temperature_errors: bool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let mut allowed_subnets = vec![
        "127.0.0.1",
        "10.*.*.*",
        "172.{16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31}.*.*",
        "192.168.1.*",
    ];

    if args.force_public {
        allowed_subnets.push("*.*.*.*");
        warn!("The monitored resources data is now accessible to anyone, including processes data");
    }

    thread::spawn(move || {
        status::continous_update(status::ErrorSuppressions {
            cpu: args.suppress_cpu_errors,
            disk: args.suppress_disk_errors,
            host: args.suppress_host_errors,
            temp: args.suppress_temperature_errors,
            net: args.suppress_net_errors,
            proc: args.suppress_proc_errors,
            ram: args.suppress_ram_errors,
        })
    });

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
    .bind(("0.0.0.0", args.port))?
    .run()
    .await
}
