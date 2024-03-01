// use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use chrono::Duration;
use crate::stream::services::{BroadcastRelay, BroadcastType, StatusMsg};

#[derive(Debug, Clone, PartialEq)]
pub enum SubscriptionStatus {
    None,
    Connecting,
    Connected,
    Disconnecting,
    Disconnected,

    Active,
    HeartbeatTimeout,
    ConnectionTimeout,
    ResultError,
}


pub struct SubscriptionContext {

    // _map: RwLock<HashMap<i32, SubscriptionStatus>>,

    _cur_status: RwLock<SubscriptionStatus>,
    _next_status: RwLock<SubscriptionStatus>,
    
    _cur_id: RwLock<Option<i32>>,
    _next_id: RwLock<Option<i32>>,
}

impl SubscriptionContext {
    pub fn new() -> Self {
        Self {


            // _map: RwLock::new(HashMap::new()),

            _cur_status: RwLock::new(SubscriptionStatus::None),
            _next_status: RwLock::new(SubscriptionStatus::None),

            _cur_id: RwLock::new(None),
            _next_id: RwLock::new(None),
        }
    }

    pub fn get_cur_id(&self) -> Option<i32> {
        self._cur_id.read().unwrap().clone()
    }

    pub fn get_next_id(&self) -> Option<i32> {
        self._next_id.read().unwrap().clone()
    }
    pub fn get_cur_status(&self) -> SubscriptionStatus {
        self._cur_status.read().unwrap().clone()
    }

    pub fn get_next_status(&self) -> SubscriptionStatus {
        self._next_status.read().unwrap().clone()
    }

    pub fn set_new_cur(&self, id: i32) {
        self._set_cur_id(Some(id));
        self._set_cur_status(SubscriptionStatus::None)
    }
    pub fn remove_cur(&self) {
        self._set_cur_id(None);
        self._set_cur_status(SubscriptionStatus::None)
    }

    pub fn set_new_next(&self, id: i32) {
        self._set_next_id(Some(id));
        self._set_next_status(SubscriptionStatus::None)
    }
    pub fn remove_next(&self) {
        self._set_next_id(None);
        self._set_next_status(SubscriptionStatus::None)
    }

    pub fn move_next_to_cur(&self) {

        println!(
            "MOVE-NEXT-to-CUR-1 c: {:?} / {:?}  n: {:?} / {:?}",
            self.get_cur_id(), self.get_cur_status(),
            self.get_next_id(), self.get_next_status(),
        );

        self._set_cur_id(self.get_next_id());
        self._set_cur_status(self.get_next_status());
        self.remove_next();

        println!(
            "MOVE-NEXT-to-CUR-2 c: {:?} / {:?}  n: {:?} / {:?}",
            self.get_cur_id(), self.get_cur_status(),
            self.get_next_id(), self.get_next_status(),
        );
    }



    fn _set_cur_id(&self, id: Option<i32>) {
        // println!("set-cur-id: {:?}", id);
        *self._cur_id.write().unwrap() = id;
    }

    fn _set_next_id(&self, id: Option<i32>) {
        // println!("set-next-id: {:?}", id);
        *self._next_id.write().unwrap() = id;
    }
    
    fn _set_cur_status(&self, status: SubscriptionStatus) {
        // println!("set-cur-status: {:?}", status);
        *self._cur_status.write().unwrap() = status;
    }

    fn _set_next_status(&self, status: SubscriptionStatus) {
        // println!("set-next-status: {:?}", status);
        *self._next_status.write().unwrap() = status;
    }

    // pub fn get_map(&self) -> HashMap<i32, SubscriptionStatus> {
    //     self._map.read().unwrap().clone()
    // }
    //
    // fn _set_map_item(&self, id: i32, status: SubscriptionStatus) {
    //     self._map.write().unwrap().insert(id, status);
    // }

    fn _update_status(&self, id: i32, status: SubscriptionStatus) {

        // - update status of cur or next based-on id...

        if self.get_cur_id() == Some(id) { // - current
            self._set_cur_status(status.clone());
        }
        if self.get_next_id() == Some(id) {
            self._set_next_status(status.clone());
        }
    }
}

pub async fn monitor_relay_for_context(
    relay: Arc<BroadcastRelay>,
    context: Arc<SubscriptionContext>,
) {

    let mut reciever = relay.get_receiver();
    loop {

        let msg = reciever.recv().await.unwrap();

        if let BroadcastType::Status(status_msg) = msg {

            match status_msg {
                StatusMsg::Connecting(id) => {
                    context._update_status(id, SubscriptionStatus::Connecting);
                }
                StatusMsg::Connected(id) => {
                    context._update_status(id, SubscriptionStatus::Connected);
                }
                StatusMsg::Active(id) => {
                    context._update_status(id, SubscriptionStatus::Active);
                }
                StatusMsg::HeartbeatTimeout(id) => {
                    context._update_status(id, SubscriptionStatus::HeartbeatTimeout);
                }
                StatusMsg::ConnectionTimeout(id) => {
                    context._update_status(id, SubscriptionStatus::ConnectionTimeout);
                }
                StatusMsg::ResponseError(id) => {
                    context._update_status(id, SubscriptionStatus::ResultError);
                }
                StatusMsg::Disconnecting(id) => {
                    context._update_status(id, SubscriptionStatus::Disconnecting);
                }
                StatusMsg::Disconnected(id) => {
                    context._update_status(id, SubscriptionStatus::Disconnected);
                }
            }
        }
    }
}


pub async fn monitor_cur_next_for_context(
    context: Arc<SubscriptionContext>,
    interval: i64,
) {

    let mut ticker = tokio::time::interval(
        Duration::milliseconds(interval).to_std().unwrap()
    );

    loop {
        ticker.tick().await;

        println!(
            "MON-CONTEXT: c:{:?} / {:?}  n:{:?} / {:?}",
            context.get_cur_id(), context.get_cur_status(),
            context.get_next_id(), context.get_next_status(),
        );
    }
}