use chrono::Duration;
use tokio::sync::mpsc;
use tokio::time::interval;
use crate::app::ArcAppContext;

pub async fn run_timer(
    arc_app_ctx: ArcAppContext
) {

    let mut ticker = interval(
        Duration::milliseconds(1000).to_std().unwrap()
    );

    let chat_relay = arc_app_ctx.clone_relay();

    let (conn_tx, _) = mpsc::unbounded_channel();

    // unwrap: chat server is not dropped before the HTTP server
    let conn_id = chat_relay.connect(conn_tx).await;

    let mut secs: i32 = 0;
    loop {

        ticker.tick().await;

        arc_app_ctx.set_elapsed(secs);
        chat_relay.send_message(conn_id, format!("timer: {secs}")).await;

        secs += 1;
    }
}