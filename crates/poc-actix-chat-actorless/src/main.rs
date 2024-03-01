//! Multi-room WebSocket chat server.
//!
//! Open `http://localhost:8080/` in browser to test.

use std::sync::Arc;
use crate::app::{AppContext, ArcAppContext, run_timer};
use crate::chat::ChatServer;
use crate::http::run_http_server;

mod chat;
mod http;
mod app;

// note that the `actix` based WebSocket handling would NOT work under `tokio::main`
#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at http://localhost:8080");

    let (chat_server, chat_relay) = ChatServer::new();

    let app_ctx = AppContext::new(chat_relay.clone());

    let arc_app_ctx: ArcAppContext = Arc::new(app_ctx.clone());

    let chat_server_task = tokio::spawn(chat_server.run());
    let app_timer_task = tokio::spawn(run_timer(arc_app_ctx.clone()));

    let http_server_8080 = run_http_server(app_ctx.clone(), 8080);
    // let http_server_9090 = run_http_server(app_ctx.clone(), 9090);

    let _ = tokio::join!(
        http_server_8080,
        // http_server_9090,
        chat_server_task,
        app_timer_task,
    );

    Ok(())
}
