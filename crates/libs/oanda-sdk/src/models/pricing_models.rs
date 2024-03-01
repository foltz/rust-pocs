use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde_derive::Deserialize;
use crate::support::errors::QueryError;
use crate::support::helpers::serde_helpers::serde_parse_decimal;

pub type PricingQueryResult = Result<PricingQueryData, QueryError>;

#[derive(Deserialize)]
pub struct PricingQueryData {

    pub prices: Vec<ClientPrice>,
    pub home_conversions: Vec<HomeConversion>,
    pub time: DateTime<Utc>,
}

// #[derive(Deserialize)]
// pub struct PricingStreamData {
// // - ClientPrice or Heartbeat
//     #[serde(rename(deserialize = "type"))]
//     pub msg_type: String,
// }

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum PricingStreamEvent {
    PRICE(ClientPrice),
    HEARTBEAT(PricingHeartbeat),
}

#[derive(Debug, Deserialize)]
pub struct PricingHeartbeat {
    pub time: DateTime<Utc>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientPrice {

    pub instrument: String,
    pub time: DateTime<Utc>,
    pub tradeable: bool,
    pub bids: Vec<PriceBucket>,
    pub asks: Vec<PriceBucket>,
    pub closeout_bid: Decimal,
    pub closeout_ask: Decimal,

}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HomeConversion {
    pub currency: String,
    pub account_gain: Decimal,
    pub account_loss: Decimal,
    pub position_value: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct PriceBucket {
    pub price: Decimal,
    pub liquidity: Option<Decimal>,
}