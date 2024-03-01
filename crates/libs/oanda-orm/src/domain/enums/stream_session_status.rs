use std::fmt;
use serde::{Serialize};

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum StreamSessionStatus {
    None,

    Connecting,
    Connected,

    Disconnecting,
    Disconnected,

    Active,

    HeartbeatTimeout,
    ConnectionTimeout,

    ResponseError,
    PayloadError,
}

impl fmt::Display for StreamSessionStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}