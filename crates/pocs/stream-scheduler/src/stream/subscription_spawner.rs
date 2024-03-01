use std::sync::Arc;
use crate::mocks::db_mock::DbMock;
use crate::stream::services::{
    BroadcastRelay,
    BroadcastType,
    ControlMsg,
    SpawnType,
    SubscriptionContext
};
use crate::stream::services::SubscriptionStatus::{Connecting,Connected,Active};
use crate::stream::subscribe_to_stream;

pub async fn spawn_subscriptions(
    relay: Arc<BroadcastRelay>,
    context: Arc<SubscriptionContext>,
    db: Arc<DbMock>,
) {

    // - on SpawnSubscription events, do the actual spawning...

    let mut reciever = relay.get_receiver();
    loop {

        let msg = reciever.recv().await.unwrap();
        if let BroadcastType::Control(ctl_msg) = msg {

            if let ControlMsg::SpawnSubscription(spawn_type) = ctl_msg {

                let mut abort_subscription = false;

                // - TODO: store rec in db and get the id....

                let sub_id = db.insert_id();

                // - ADD subscription to context:

                // - if it's the first subscription, set it to current.
                // - otherwise, set it to next.

                let ctx_cur_id = context.get_cur_id();
                let ctx_next_id = context.get_next_id();

                if ctx_cur_id == None {
                    context.set_new_cur(sub_id);

                } else if ctx_next_id == None {
                    context.set_new_next(sub_id);

                } else {

                    // - there's already a next.
                    // - if it's still trying to connect, don't create another one....

                    let ctx_next_status = context.get_next_status();

                    abort_subscription = match spawn_type {
                        SpawnType::Scheduled => {
                            vec![Connecting, Connected] // - if active, we replace subscription...
                                .contains(&ctx_next_status)
                        }
                        _ => {
                            vec![Connecting, Connected, Active]
                                .contains(&ctx_next_status)
                        }
                        // - TODO: we may want to handle these differently?
                        // SpawnType::HeartbeatTimeout => {}
                        // SpawnType::ConnectionTimeout => {}
                    };

                    if abort_subscription {

                        // - TODO: update subscription-record in db - as aborted.

                    } else {

                        // - flag existing-next for disconnect:
                        let existing_next_id = ctx_next_id.unwrap();
                        let _ = relay.control_msg(ControlMsg::DisconnectSubscription(existing_next_id));

                        context.set_new_next(sub_id);
                    }
                }

                if abort_subscription == false {
                    let _ = tokio::spawn(
                        subscribe_to_stream(
                            sub_id,
                            relay.clone(),
                            context.clone(),
                        )
                    );
                }
            }
        }
    }
}