use chrono::{Timelike, Utc};
use tokio::sync::broadcast;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::sync::broadcast::error::SendError;

#[derive(Clone, Debug)]
pub enum BroadcastType {
    Status(StatusMsg),
    Control(ControlMsg),
}

#[derive(Clone, Debug)]
pub enum StatusMsg {
    Connecting(i32),
    Connected(i32),
    Disconnecting(i32),
    Disconnected(i32),

    Active(i32),
    HeartbeatTimeout(i32),
    ConnectionTimeout(i32),
    ResponseError(i32),
}

#[derive(Clone, Debug)]
pub enum ControlMsg {
    DisconnectSubscription(i32),
    SpawnSubscription(SpawnType),
}

#[derive(Clone, Debug)]
pub enum SpawnType {
    Scheduled,
    HeartbeatTimeout,
    ConnectionTimeout,
}

pub struct BroadcastRelay {
    _sender: Sender<BroadcastType>,
    _reciever: Receiver<BroadcastType>,
}

impl BroadcastRelay {

    pub fn new() -> Self {
        let (sender, receiver) = broadcast::channel::<BroadcastType>(8);
        Self {
            _sender: sender,
            _reciever: receiver,
        }
    }

    pub fn get_sender(&self) -> Sender<BroadcastType> {
        self._sender.clone()
    }
    pub fn get_receiver(&self) -> Receiver<BroadcastType> {
        self._sender.subscribe()
    }

    pub fn control_msg(&self, msg: ControlMsg) -> Result<usize, SendError<BroadcastType>> {
        println!("::control_msg: {:?} ({:?})", msg, Utc::now().num_seconds_from_midnight());
        self._sender.send(BroadcastType::Control(msg))
    }
    pub fn status_msg(&self, msg: StatusMsg) -> Result<usize, SendError<BroadcastType>> {
        println!("::status_msg: {:?} ({:?})", msg, Utc::now().num_seconds_from_midnight());
        self._sender.send(BroadcastType::Status(msg))
    }
}
