use std::pin::Pin;
use std::task::{Context, Poll};
use chrono::{DateTime, Utc};
use futures::Stream;

pub struct HttpMock {
    interval: u64,
    timeout: u64,
}

impl HttpMock {
    pub fn new(interval: u64, timeout: u64) -> Self {
        Self {
            interval,
            timeout,
        }
    }

    // pub fn insert_id(&self) -> i32 {
    //
    //     let next_id = self._next_id.read().unwrap().clone();
    //     *self._next_id.write().unwrap() = next_id + 1;
    //     next_id
    // }
}

// struct MockResponse {
//
// }
//
// impl Stream for MockResponse {
//     type Item = DateTime<Utc>;
//
//     fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
//         todo!()
//     }
// }