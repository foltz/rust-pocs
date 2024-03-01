use chrono::{DateTime, Utc};
use serde_derive::Deserialize;
use crate::models::client_extensions_model::ClientExtensions;

#[derive(Deserialize)]
pub enum OrderState {
    PENDING,
    FILLED,
    TRIGGERED,
    CANCELLED
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    /// The Order’s identifier, unique within the Order’s Account.
    pub id: String,
    /// The time when the Order was created.
    pub create_time: DateTime<Utc>,
    /// The current state of the Order.
    pub state: OrderState,
    /// The client extensions of the Order. Do not set, modify, or delete
    /// clientExtensions if your account is associated with MT4.
    pub client_extensions: ClientExtensions
}