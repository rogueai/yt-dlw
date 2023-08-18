
use actix::{Actor, StreamHandler};
use actix_web::{App, get, HttpRequest, HttpResponse, HttpServer, post, Responder, web, Error};
use actix_web::middleware::Logger;
use actix_web_actors::ws;
use anyhow::Result;
use apalis::{layers::TraceLayer, redis::RedisStorage};
use apalis::prelude::*;
use futures::future;

use crate::ydl_service::{download, info, Video};

mod ydl_service;

struct ApiWs;

impl Actor for ApiWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ApiWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, actix_web::Error> {
    let resp = ws::start(ApiWs {}, &req, stream);
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

#[get("/")]
async fn hello() -> impl Responder {
    log::info!("Progress called!");
    HttpResponse::Ok().body("Hello world!")
}

#[post("/internal")]
async fn internal(mut payload: web::Bytes) -> Result<HttpResponse, Error> {
    let s = String::from_utf8(payload.to_vec())?;
    Ok(HttpResponse::Ok().finish())
}

#[actix_web::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let storage = RedisStorage::connect("redis://127.0.0.1/").await?;
    let data = web::Data::new(storage.clone());
    let http = async {
        HttpServer::new(move || {
            App::new()
                .wrap(Logger::default())
                .app_data(data.clone())
                .service(web::resource("/ws").route(web::get().to(ws_index)))
                .service(web::scope("/api").route("/download", web::post().to(api_download)))
                .service(hello)
                .service(internal)
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
        // .register_with_count(1, move |c| {
        //     WorkerBuilder::new(format!("ydl-info-{c}"))
        //         .layer(TraceLayer::new())
        //         .with_storage(storage.clone())
        //         .build_fn(info)
        // })
        .run();

    future::try_join(http, worker).await?;
    Ok(())
}
