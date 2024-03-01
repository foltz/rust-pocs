mod stream;
mod mocks;

use std::sync::Arc;
use crate::mocks::db_mock::DbMock;
use crate::stream::{orchestrate_subscriptions, schedule_subscriptions, spawn_subscriptions};
use crate::stream::services::{BroadcastRelay, monitor_cur_next_for_context, monitor_relay_for_context, SubscriptionContext};

#[tokio::main]
async fn main() {

    let relay = Arc::new(BroadcastRelay::new());
    let context = Arc::new(SubscriptionContext::new());
    let db = Arc::new(DbMock::new());

    let _ = tokio::spawn(
        monitor_relay_for_context(
            relay.clone(),
            context.clone(),
        )
    );

    let _ = tokio::spawn(
        orchestrate_subscriptions(
            relay.clone(),
            context.clone(),
        )
    );

    let _ = tokio::spawn(
        spawn_subscriptions(
            relay.clone(),
            context.clone(),
            db.clone(),
        )
    );

    let schedule_interval = 60 * 1000; // - 10s
    let _ = tokio::spawn(
        schedule_subscriptions(
            relay.clone(),
            schedule_interval,
        )
    );

    let mon_ctx_interval = 1 * 1000; // - 10s
    let _ = tokio::spawn(
        monitor_cur_next_for_context(
            context.clone(),
            mon_ctx_interval,
        )
    );


    loop {}
}