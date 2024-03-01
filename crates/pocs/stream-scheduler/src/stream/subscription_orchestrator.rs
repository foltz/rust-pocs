use std::sync::Arc;
use crate::stream::services::{
    BroadcastRelay,
    BroadcastType,
    ControlMsg,
    StatusMsg,
    SpawnType,
    SubscriptionContext,
};

pub async fn orchestrate_subscriptions(
    relay: Arc<BroadcastRelay>,
    context: Arc<SubscriptionContext>,
) {


    let mut reciever = relay.get_receiver();

    loop {

        let msg = reciever.recv().await.unwrap();

        if let BroadcastType::Status(status_msg) = msg {

            if let StatusMsg::HeartbeatTimeout(sub_id) = status_msg {

                // - if CUR-SUBSCRIPTION has a HEARTBEAT-TIMEOUT:
                // - spawn a new subscription (if one doesn't already exist)

                if Some(sub_id) == context.get_cur_id() && None == context.get_next_id() {
                    let _ = relay.control_msg(ControlMsg::SpawnSubscription(SpawnType::HeartbeatTimeout));
                }

                // - if NEXT-SUBSCRIPTION has a HEARTBEAT-TIMEOUT:
                // - spawn a new subscription again....
                // - TODO: we might want to limit the number of times this happens?

                if Some(sub_id) == context.get_next_id() {
                    let _ = relay.control_msg(ControlMsg::SpawnSubscription(SpawnType::HeartbeatTimeout));
                }
            }

            if let StatusMsg::ConnectionTimeout(sub_id) = status_msg {

                // - if CUR-SUBSCRIPTION has a CONNECTION-TIMEOUT:
                // - spawn a new subscription (if one doesn't already exist)

                if Some(sub_id) == context.get_cur_id() && None == context.get_next_id() {
                    let _ = relay.control_msg(ControlMsg::SpawnSubscription(SpawnType::ConnectionTimeout));
                }

                // - if NEXT-SUBSCRIPTION has a CONNECTION-TIMEOUT:
                // - spawn a new subscription again....
                // - TODO: we might want to limit the number of times this happens?

                if Some(sub_id) == context.get_next_id() {
                    let _ = relay.control_msg(ControlMsg::SpawnSubscription(SpawnType::ConnectionTimeout));
                }
            }

            if let StatusMsg::Active(sub_id) = status_msg {

                // - if NEXT-SUBSCRIPTION becomes ACTIVE:
                // - disconnect the current subscription.

                if Some(sub_id) == context.get_next_id() {
                    if let Some(cur_id) = context.get_cur_id() {
                        let _ = relay.control_msg(ControlMsg::DisconnectSubscription(cur_id));
                    }
                }
            }

            if let StatusMsg::Disconnected(sub_id) = status_msg {

                println!("test-orchestrator-DISCONNECTED: {sub_id}");

                // println!(
                //     "CONTEXT: c:{:?} / {:?}  n:{:?} / {:?}",
                //     context.get_cur_id(), context.get_cur_status(),
                //     context.get_next_id(), context.get_next_status(),
                // );

                // - if CUR-SUBSCRIPTION becomes DISCONNECTED:
                // - move next to cur.
                // - otherwise remove cur.

                if Some(sub_id) == context.get_cur_id() {
                    if let Some(_) = context.get_next_id() {
                        context.move_next_to_cur();
                    } else {
                        context.remove_cur();
                    }
                }

                // - if NEXT-SUBSCRIPTION becomes DISCONNECTED:
                // - remove next.

                if Some(sub_id) == context.get_next_id() {
                    context.remove_next();
                }
            }
        }
    }
}