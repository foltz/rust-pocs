use std::fmt;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub enum StreamSessionTrigger {
    Scheduled,
    HeartbeatTimeout,
    ConnectionTimeout,
    ResponseError,
}

impl fmt::Display for StreamSessionTrigger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}