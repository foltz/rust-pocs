use chrono::{DateTime, Utc};
use serde_derive::{Deserialize};
use crate::models::shared_book_models::SharedBookQueryData;
use crate::support::errors::QueryError;

pub type OrderBookQueryResult = Result<SharedBookQueryData, QueryError>;
// pub type OrderBookQueryData = SharedBookQueryData;

/// This Object packages key query-parameters with the response so that the query can be logged
// - TODO: remove...
#[derive(Clone, Debug, Deserialize)]
pub struct OrderBookQueryResponse {

    pub instrument: String,
    pub time: DateTime<Utc>,
    pub frequency: i32,

    pub data: Option<SharedBookQueryData>,
    pub error: Option<QueryError>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookResponse {
    pub order_book: SharedBookQueryData,
}

// #[derive(Clone, Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct OrderBook {
//
//     /// The order book’s instrument
//     pub instrument: String,
//
//     /// The time when the order book snapshot was created.
//     pub time: DateTime<Utc>,
//
//     /// The price (midpoint) for the order book’s instrument at the time of the
//     /// order book snapshot
//
//     #[serde(deserialize_with = "serde_parse_decimal")]
//     pub price: Decimal,
//
//     /// The price width for each bucket. Each bucket covers the price range from
//     /// the bucket’s price to the bucket’s price + bucketWidth.
//
//     #[serde(deserialize_with = "serde_parse_decimal")]
//     pub bucket_width: Decimal,
//
//     /// The partitioned order book, divided into buckets using a default bucket
//     /// width. These buckets are only provided for price ranges which actually
//     /// contain order or position data.
//     pub buckets: Vec<OrderBookBucket>
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct OrderBookBucket {
//
//     /// The price (midpoint) for the order book’s instrument at the time of the
//     /// order book snapshot
//
//     #[serde(deserialize_with = "serde_parse_decimal")]
//     pub price: Decimal,
//
//     /// The percentage of the total number of orders represented by the long
//     /// orders found in this bucket.
//
//     #[serde(deserialize_with = "serde_parse_decimal")]
//     pub long_count_percent: Decimal,
//
//     /// The percentage of the total number of orders represented by the short
//     /// orders found in this bucket.
//
//     #[serde(deserialize_with = "serde_parse_decimal")]
//     pub short_count_percent: Decimal,
// }