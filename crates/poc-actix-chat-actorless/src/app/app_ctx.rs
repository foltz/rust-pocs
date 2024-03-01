use std::sync::{Arc, RwLock};
use actix_web::web;
use crate::chat::ChatRelay;

pub type WebAppContext = web::Data<AppContext>;
pub type ArcAppContext = Arc<AppContext>;

#[derive(Clone)]
pub struct AppContext {

    chat_relay: ChatRelay,

    count: Arc<RwLock<i16>>,
    elapsed: Arc<RwLock<i32>>,

}

impl AppContext {

    pub fn new(chat_relay: ChatRelay) -> Self {
        Self {
            chat_relay,

            count: Arc::new(RwLock::new(0)),
            elapsed: Arc::new(RwLock::new(0)),

        }
    }
    
    pub fn clone_relay(&self) -> ChatRelay {
        self.chat_relay.clone()
    }

    pub fn get_count(&self) -> i16 {
        *self.count.read().unwrap()
    }

    pub fn inc_count(&self) -> i16 {
        let new_count = self.get_count() + 1;
        *self.count.write().unwrap() = new_count;
        new_count
    }


    pub fn get_elapsed(&self) -> i32 {
        *self.elapsed.read().unwrap()
    }

    pub fn set_elapsed(&self, val: i32) -> i32 {
        *self.elapsed.write().unwrap() = val;
        val
    }
}