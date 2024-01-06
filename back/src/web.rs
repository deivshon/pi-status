use crate::status::{ACTIVE_WS_CONNECTIONS, STATUS_STR};

use std::error::Error;
use std::path::PathBuf;
use std::sync::atomic::Ordering;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use actix::{Actor, AsyncContext, StreamHandler};
use actix_files::NamedFile;
use actix_web::{web as ActixWeb, Error as ActixError, HttpRequest, HttpResponse};
use actix_web_actors::ws::{self, Message, ProtocolError};
use log::error;

const FRONT_PATH: &str = "./front/pi-status-front/dist/index.html";

pub async fn index() -> Result<NamedFile, Box<dyn Error>> {
    let path: PathBuf = std::fs::canonicalize(FRONT_PATH)?;

    Ok(NamedFile::open(path)?)
}

pub async fn serve_data(
    req: HttpRequest,
    stream: ActixWeb::Payload,
) -> Result<HttpResponse, ActixError> {
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
