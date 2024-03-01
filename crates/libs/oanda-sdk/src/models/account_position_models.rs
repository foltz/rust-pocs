use rust_decimal::Decimal;
use serde_derive::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    /// The Position’s Instrument.
    pub instrument: String,
    /// Profit/loss realized by the Position over the lifetime of the Account.
    pub pl: Decimal,
    /// The unrealized profit/loss of all open Trades that contribute to this
    /// Position.
    #[serde(rename = "unrealizedPL")]
    pub unrealized_pl: Decimal,
    /// Profit/loss realized by the Position since the Account’s resettablePL was
    /// last reset by the client.
    #[serde(rename = "resettablePL")]
    pub resettable_pl: Decimal,
    /// The details of the long side of the Position.
    pub long: PositionSide,
    /// The details of the short side of the Position.
    pub short: PositionSide
}


#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionSide {
    /// Number of units in the position (negative value indicates short position,
    /// positive indicates long position).
    pub units: Decimal,
    /// Volume-weighted average of the underlying Trade open prices for the
    /// Position.
    pub average_price: Decimal,
    /// List of the open Trade IDs which contribute to the open Position.
    #[serde(rename = "tradeIDs")]
    pub trade_ids: Vec<String>,
    /// Profit/loss realized by the PositionSide over the lifetime of the
    /// Account.
    pub pl: Decimal,
    /// The unrealized profit/loss of all open Trades that contribute to this
    /// PositionSide.
    #[serde(rename = "unrealizedPL")]
    pub unrealized_pl: Decimal,
    /// Profit/loss realized by the PositionSide since the Account’s resettablePL
    /// was last reset by the client.
    #[serde(rename = "resettablePL")]
    pub resettable_pl: Decimal
}