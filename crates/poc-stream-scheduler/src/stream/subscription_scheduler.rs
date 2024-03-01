use std::sync::Arc;
use chrono::{Duration, Utc};
use crate::stream::services::{BroadcastRelay, ControlMsg, SpawnType};

pub async fn schedule_subscriptions(
    relay: Arc<BroadcastRelay>,
    interval: i64,
) {

    let start_time = Utc::now();

    println!("scheduler-begin: {:?}ms", (Utc::now() - start_time).num_seconds());

    // - spawn a new subscription every interval....


    let mut ticker = tokio::time::interval(
        Duration::milliseconds(interval).to_std().unwrap()
    );

    loop {

        ticker.tick().await;
        // println!("scheduler-loop-begin: {:?}ms", (Utc::now() - start_time).num_seconds());

        let _ = relay.control_msg(
            ControlMsg::SpawnSubscription(SpawnType::Scheduled)
        );

        // println!("scheduler-loop-end: {:?}ms", (Utc::now() - start_time).num_seconds());
    }
}