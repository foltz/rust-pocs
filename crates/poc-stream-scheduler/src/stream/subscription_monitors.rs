use std::ops::Deref;
use std::sync::{Arc, RwLock};
use chrono::{Duration, Utc};
use tokio::time::interval;
use crate::stream::services::{BroadcastRelay, BroadcastType, ControlMsg, StatusMsg};


pub async fn monitor_subscription_relay(
    sub_id: i32,
    relay: Arc<BroadcastRelay>,
    kill_switch: Arc<RwLock<bool>>,
) {

    let mut reciever = relay.get_receiver();
    loop {

        let msg = reciever.recv().await.unwrap();
        // println!("id: {:} msg: {:?}", sub_id, msg);

        if let BroadcastType::Control(ctl_msg) = msg {

            if let ControlMsg::DisconnectSubscription(id) = ctl_msg {
                if id == 0 || id == sub_id {
                    *kill_switch.write().unwrap() = true;
                }
            }
        }
    }
}

pub async fn monitor_subscription_heartbeat(
    sub_id: i32,
    relay: Arc<BroadcastRelay>,
    kill_switch: Arc<RwLock<bool>>,
) {

    let start_time = Utc::now();

    let mut ticker = interval(
        Duration::milliseconds(1000).to_std().unwrap()
    );

    loop {

        ticker.tick().await;

        let disconnect = *kill_switch.read().unwrap().deref();
        if disconnect == true {
            return ();
        }

        let cur_time = Utc::now();
        let duration = cur_time - start_time;

        // println!("heart-beat: {} :: {}ms", sub_id, duration.num_milliseconds());

        if duration.num_seconds() > 15 {
            let _ = relay.status_msg(StatusMsg::HeartbeatTimeout(sub_id));
        }
    }
}