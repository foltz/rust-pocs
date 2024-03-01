use actix_files::NamedFile;
use actix_web::{Error, get, HttpRequest, HttpResponse, Responder, web};
use tokio::task::spawn_local;
use crate::app::WebAppContext;
use crate::chat::ws_message_handler;

#[get("/")]
pub async fn get_index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}

/// Handshake and start WebSocket handler with heartbeats.

#[get("/ws")]
pub async fn get_ws(
    req: HttpRequest,
    stream: web::Payload,
    web_app_ctx: WebAppContext,
) -> Result<HttpResponse, Error> {

    let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;
    let chat_relay = web_app_ctx.clone_relay();

    // spawn websocket handler (and don't await it) so that the response is returned immediately
    spawn_local(ws_message_handler(
        chat_relay,
        session,
        msg_stream,
    ));

    Ok(res)
}