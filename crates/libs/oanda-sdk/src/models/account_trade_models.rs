use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde_derive::Deserialize;
use crate::models::client_extensions_model::ClientExtensions;

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
pub enum TradeState {
    OPEN,
    CLOSED,
    CLOSE_WHEN_TRADEABLE
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeSummary {
    /// The Trade’s identifier, unique within the Trade’s Account.
    pub id: String,
    /// The Trade’s Instrument.
    pub instrument: String,
    /// The execution price of the Trade.
    pub price: Decimal,
    /// The date/time when the Trade was opened.
    pub open_time: DateTime<Utc>,
    /// The current state of the Trade.
    pub state: TradeState,
    /// The initial size of the Trade. Negative values indicate a short Trade,
    /// and positive values indicate a long Trade.
    pub initial_units: Decimal,
    /// The number of units currently open for the Trade. This value is reduced
    /// to 0.0 as the Trade is closed.
    pub current_units: Decimal,
    /// The total profit/loss realized on the closed portion of the Trade.
    #[serde(rename = "realizedPL")]
    pub realized_pl: Decimal,
    /// The unrealized profit/loss on the open portion of the Trade.
    #[serde(rename = "unrealizedPL")]
    pub unrealized_pl: Decimal,
    /// The average closing price of the Trade. Only present if the Trade has
    /// been closed or reduced at least once.
    pub average_close_price: Option<Decimal>,
    /// The IDs of the Transactions that have closed portions of this Trade.
    #[serde(rename = "closingTransactionIDs")]
    pub closing_transaction_ids: Vec<String>,
    /// The financing paid/collected for this Trade.
    pub financing: Decimal,
    /// The date/time when the Trade was fully closed. Only provided for Trades
    /// whose state is CLOSED.
    pub close_time: Option<DateTime<Utc>>,
    /// The client extensions of the Trade.
    pub client_extensions: ClientExtensions,
    /// ID of the Trade’s Take Profit Order, only provided if such an Order
    /// exists.
    #[serde(rename = "takeProfitOrderID")]
    pub take_profit_order_id: Option<String>,
    /// ID of the Trade’s Stop Loss Order, only provided if such an Order exists.
    #[serde(rename = "stopLossOrderID")]
    pub stop_loss_order_id: Option<String>,
    /// ID of the Trade’s Trailing Stop Loss Order, only provided if such an
    /// Order exists.
    #[serde(rename = "trailingStopLossOrderID")]
    pub trailing_stop_loss_order_id: Option<String>
}