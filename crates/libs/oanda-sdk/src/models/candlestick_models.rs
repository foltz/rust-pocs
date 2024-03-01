use std::fmt;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde_derive::Deserialize;

use crate::support::helpers::serde_helpers::serde_parse_decimal;
use crate::support::errors::QueryError;


pub type CandlestickQueryResult = Result<CandlestickQueryData, QueryError>;

/// - NOTE: Referred to as CandlestickPricing in the v20 API:
#[derive(Clone, Debug, Deserialize)]
pub struct CandlestickQueryData {

    /// The instrument whose Prices are represented by the candlesticks.
    pub instrument: String,

    /// The granularity of the candlesticks provided.
    pub granularity: CandlestickGranularity,

    /// The list of candlesticks that satisfy the request.
    pub candles: Vec<CandlestickSnapshot>
}

// - TODO: remove...
// #[derive(Clone, Debug, Deserialize)]
// pub struct CandlestickQueryResponse {
// 
//     pub instrument: String,
//     pub granularity: CandlestickGranularity,
//     pub from: DateTime<Utc>,
//     pub to: Option<DateTime<Utc>>,
// 
//     pub data: Option<CandlestickData>,
//     pub error: Option<QueryError>,
// }

#[derive(Clone, Debug, Default, Deserialize)]
pub enum CandlestickGranularity {

    #[default]
    S5, // - 5 second candlesticks, minute alignment
    S10, // - 10 second candlesticks, minute alignment
    S15, // - 15 second candlesticks, minute alignment
    S30, // - 30 second candlesticks, minute alignment

    M1, // - 1 minute candlesticks, minute alignment
    M2, // - 2 minute candlesticks, hour alignment
    M4, // - 4 minute candlesticks, hour alignment
    M5, // - 5 minute candlesticks, hour alignment
    M10, // - 10 minute candlesticks, hour alignment
    M15, // - 15 minute candlesticks, hour alignment
    M30, // - 30 minute candlesticks, hour alignment

    H1, // - 1 hour candlesticks, hour alignment
    H2, // - 2 hour candlesticks, day alignment
    H3, // - 3 hour candlesticks, day alignment
    H4, // - 4 hour candlesticks, day alignment
    H6, // - 6 hour candlesticks, day alignment
    H8, // - 8 hour candlesticks, day alignment
    H12, // - 12 hour candlesticks, day alignment

    D, // - 1 day candlesticks, day alignment
    W, // - 1 week candlesticks, aligned to start of week
    M, // - 1 month candlesticks, aligned to first day of the month
}

impl fmt::Display for CandlestickGranularity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

/// - NOTE: Referred to as CandlestickPrice in v20 API
#[derive(Clone, Debug, Default, Deserialize)]
pub enum CandlestickPrice {

    #[default]
    BAM, // - Bid+Ask+Mid

    B, // - Bid
    A, // - Ask
    M, // - Mid
    BA, // - Bid+Ask
    BM, // - Bid+Mid
    AM, // - Ask+Mid
}

impl fmt::Display for CandlestickPrice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}


/// - NOTE: Referred to as Candlestick in v20 API
#[derive(Clone, Debug, Deserialize)]
pub struct CandlestickSnapshot {

    /// The start time of the candlestick
    pub time: DateTime<Utc>,

    /// The candlestick data based on bids. Only provided if bid-based candles
    /// were requested.
    pub bid: Option<Candlestick>,

    /// The candlestick data based on asks. Only provided if ask-based candles
    /// were requested.
    pub ask: Option<Candlestick>,

    /// The candlestick data based on midpoints. Only provided if midpoint-based
    /// candles were requested.
    pub mid: Option<Candlestick>,

    /// The number of prices created during the time-range represented by the
    /// candlestick.
    pub volume: i32,

    /// A flag indicating if the candlestick is complete. A complete candlestick
    /// is one whose ending time is not in the future.
    pub complete: bool
}

/// - NOTE: Referred to as CandlestickData in v20 API
#[derive(Clone, Debug, Deserialize)]
pub struct Candlestick {

    /// The first (open) price in the time-range represented by the candlestick.
    #[serde(deserialize_with = "serde_parse_decimal")]
    pub o: Decimal,

    /// The highest price in the time-range represented by the candlestick.
    #[serde(deserialize_with = "serde_parse_decimal")]
    pub h: Decimal,

    /// The lowest price in the time-range represented by the candlestick.
    #[serde(deserialize_with = "serde_parse_decimal")]
    pub l: Decimal,

    /// The last (closing) price in the time-range represented by the
    /// candlestick.
    #[serde(deserialize_with = "serde_parse_decimal")]
    pub c: Decimal
}
