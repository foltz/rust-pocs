use std::ops::Deref;
use std::sync::{Arc, RwLock};
use chrono::{Duration, Utc};
use tokio::time::interval;
use crate::stream::services::{
    BroadcastRelay,
    StatusMsg,
    SubscriptionContext
};
use crate::stream::subscription_monitors::{
    monitor_subscription_heartbeat,
    monitor_subscription_relay
};
use crate::stream::services::StatusMsg::{Connected, Connecting, Active};

pub async fn subscribe_to_stream(
    sub_id: i32,
    relay: Arc<BroadcastRelay>,
    context: Arc<SubscriptionContext>,
) {

    let kill_switch = Arc::new(RwLock::new(false));

    let mut monitors = Vec::new();

    monitors.push(tokio::spawn(
        monitor_subscription_relay(sub_id, relay.clone(), kill_switch.clone())
    ));

    monitors.push(tokio::spawn(
        monitor_subscription_heartbeat(sub_id, relay.clone(), kill_switch.clone())
    ));

    let _ = tokio::spawn(
        stream_events(
            sub_id,
            relay.clone(),
            context.clone(),
            kill_switch.clone(),
        )
    ).await;

    for task in &monitors {
        task.abort();
    }

    println!("subscription-finished: {}", sub_id);
}


async fn stream_events(
    sub_id: i32,
    relay: Arc<BroadcastRelay>,
    context: Arc<SubscriptionContext>,
    kill_switch: Arc<RwLock<bool>>,
) {

    let start_time = Utc::now();

    let mut ticker = interval(
        Duration::milliseconds(742).to_std().unwrap()
    );

    let mut has_prev_response = false;

    ticker.tick().await;
    let _ = relay.status_msg(Connecting(sub_id));

    ticker.tick().await;
    let _ = relay.status_msg(Connected(sub_id));

    loop {

        ticker.tick().await;

        let disconnect = *kill_switch.read().unwrap().deref();
        if disconnect == true {

            ticker.tick().await;
            let _ = relay.status_msg(StatusMsg::Disconnecting(sub_id));

            // println!("disconnecting: {}", sub_id);

            ticker.tick().await;
            let _ = relay.status_msg(StatusMsg::Disconnected(sub_id));
            return ();
        }

        // let cur_time = Utc::now();
        // let duration = cur_time - start_time;

        // println!("stream-event: {} :: {}ms", sub_id, duration.num_milliseconds());
        if !has_prev_response {
            // println!("first-event!");
            let _ = relay.status_msg(Active(sub_id));
            has_prev_response = true;

            // println!(
            //     "CONTEXT: c:{:?} / {:?}  n:{:?} / {:?}",
            //     context.get_cur_id(), context.get_cur_status(),
            //     context.get_next_id(), context.get_next_status(),
            // );

        }

        // ticker.tick().await;
    }

}