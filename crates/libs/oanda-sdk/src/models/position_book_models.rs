use chrono::{DateTime, Utc};
use serde_derive::Deserialize;
use crate::models::shared_book_models::SharedBookQueryData;
use crate::support::errors::QueryError;

pub type PositionBookQueryResult = Result<SharedBookQueryData, QueryError>;
// pub type PositionBookQueryData = SharedBookQueryData;

/// This Object packages key query-parameters with the response so that the query can be logged
#[derive(Clone, Debug, Deserialize)]
pub struct PositionBookQueryResponse {

    pub instrument: String,
    pub frequency: i32,

    pub time: DateTime<Utc>,

    pub data: Option<SharedBookQueryData>,
    pub error: Option<QueryError>,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionBookResponse {
    pub position_book: SharedBookQueryData,
}

// #[derive(Clone, Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct PositionBook {
//
//     /// The position book’s instrument
//     pub instrument: String,
//
//     /// The time when the position book snapshot was created.
//     pub time: DateTime<Utc>,
//
//     /// The price (midpoint) for the position book’s instrument at the time of the
//     /// position book snapshot
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
//     /// The partitioned position book, divided into buckets using a default bucket
//     /// width. These buckets are only provided for price ranges which actually
//     /// contain position or position data.
//     pub buckets: Vec<PositionBookBucket>
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct PositionBookBucket {
//
//     /// The price (midpoint) for the position book’s instrument at the time of the
//     /// position book snapshot
//
//     #[serde(deserialize_with = "serde_parse_decimal")]
//     pub price: Decimal,
//
//     /// The percentage of the total number of positions represented by the long
//     /// positions found in this bucket.
//
//     #[serde(deserialize_with = "serde_parse_decimal")]
//     pub long_count_percent: Decimal,
//
//     /// The percentage of the total number of positions represented by the short
//     /// positions found in this bucket.
//
//     #[serde(deserialize_with = "serde_parse_decimal")]
//     pub short_count_percent: Decimal,
// }
