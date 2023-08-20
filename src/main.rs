use std::fs;

use actix::{Actor, StreamHandler};
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use actix_web::middleware::Logger;
use actix_web_actors::ws;
use anyhow::Result;
use apalis::{layers::TraceLayer, redis::RedisStorage};
use apalis::prelude::*;
use futures::future;
use pyo3::Python;
use pyo3::types::PyModule;

use crate::ydl_service::{download, Video};

mod ydl_service;

struct Ws;

impl Actor for Ws {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Ws {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                println!("{}", text.to_string());
                ctx.text(text)
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, actix_web::Error> {
    let resp = ws::start(Ws {}, &req, stream);
    println!("{:?}", resp);
    resp
}

async fn api_download(
    video: web::Json<Video>,
    storage: web::Data<RedisStorage<Video>>,
) -> HttpResponse {
    let storage = &*storage.into_inner();
    let mut storage = storage.clone();
    let res = storage.push(video.into_inner()).await;
    match res {
        Ok(jid) => HttpResponse::Ok().body(format!("New job with job_id [{jid}] added to queue")),
        Err(e) => HttpResponse::InternalServerError().body(format!("{e}")),
    }
}

#[actix_web::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    init_pyo3();

    let storage = RedisStorage::connect("redis://127.0.0.1/").await?;
    let data = web::Data::new(storage.clone());
    let http = async {
        HttpServer::new(move || {
            App::new()
                .wrap(Logger::default())
                .app_data(data.clone())
                .service(web::resource("/ws").route(web::get().to(ws_index)))
                .service(web::scope("/api").route("/download", web::post().to(api_download)))
        })
        .bind("127.0.0.1:8000")?
        .run()
        .await?;
        Ok(())
    };
    let worker = Monitor::new()
        .register_with_count(1, move |c| {
            WorkerBuilder::new(format!("ydl-tasks-{c}"))
                .layer(TraceLayer::new())
                .with_storage(storage.clone())
                .build_fn(download)
        })
        .run();

    future::try_join(http, worker).await?;
    Ok(())
}

fn init_pyo3() {
    // init python
    let _: Result<()> = Python::with_gil(|py| {
        // PY: from yt_dlp import YoutubeDL
        let _ = PyModule::from_code(
            py,
            fs::read_to_string("python/yt_dlw.py").unwrap().as_str(),
            "yt_dlw.py",
            "yt_dlw",
        )
        .unwrap_or_else(|e| {
            e.display(py);
            panic!("{}", e)
        });

        Ok(())
    });
}
