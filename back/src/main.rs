pub mod status;
pub mod web;

use actix_ip_filter::IPFilter;
use actix_web::{web as ActixWeb, App, HttpServer};
use clap::Parser;
use log::warn;
use std::thread;

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
            .service(ActixWeb::resource("/ws_data").to(web::serve_data))
            .route("/", ActixWeb::get().to(web::index))
            .service(actix_files::Files::new(
                "/",
                "./front/pi-status-front/dist/",
            ))
    })
    .bind(("0.0.0.0", args.port))?
    .run()
    .await
}
