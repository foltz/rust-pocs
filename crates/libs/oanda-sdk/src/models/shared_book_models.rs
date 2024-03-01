use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde_derive::{Deserialize, Serialize};

use crate::support::helpers::serde_helpers::{serde_parse_decimal, serde_parse_opt_decimal};


#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedBookQueryData {

    /// The book’s instrument
    pub instrument: String,

    /// The time when the position book snapshot was created.
    pub time: DateTime<Utc>,

    /// The price (midpoint) for the position book’s instrument at the time of the
    /// book snapshot

    // #[serde(deserialize_with = "serde_parse_opt_decimal")]
    pub price: Option<Decimal>,

    /// The price width for each bucket. Each bucket covers the price range from
    /// the bucket’s price to the bucket’s price + bucketWidth.

    // - TODO: do we need this trait?
    #[serde(deserialize_with = "serde_parse_decimal")]
    pub bucket_width: Decimal,

    /// The partitioned book, divided into buckets using a default bucket
    /// width. These buckets are only provided for price ranges which actually
    /// contain data.

    // - TODO: it looks like this is working without a serde trait - confirm!
    pub buckets: Vec<SharedBookBucket>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedBookBucket {

    /// The price (midpoint) for the book’s instrument at the time of the
    /// book snapshot

    #[serde(deserialize_with = "serde_parse_decimal")]
    pub price: Decimal,

    /// The percentage of the total number of entries represented by the long
    /// orders or positions found in this bucket.

    #[serde(deserialize_with = "serde_parse_decimal")]
    pub long_count_percent: Decimal,

    /// The percentage of the total number of entries represented by the short
    /// orders or positions found in this bucket.

    #[serde(deserialize_with = "serde_parse_decimal")]
    pub short_count_percent: Decimal,
}

impl PartialEq for SharedBookBucket {

    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
        && self.short_count_percent == other.short_count_percent
        && self.long_count_percent == other.long_count_percent
    }

    fn ne(&self, other: &Self) -> bool {
        self.price != other.price
            || self.short_count_percent != other.short_count_percent
            || self.long_count_percent != other.long_count_percent
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedBookBucketWithDiff {

    /// The price (midpoint) for the book’s instrument at the time of the
    /// book snapshot

    #[serde(deserialize_with = "serde_parse_decimal")]
    pub price: Decimal,
    #[serde(deserialize_with = "serde_parse_decimal")]
    pub price_diff: Decimal,

    /// The percentage of the total number of entries represented by the long
    /// orders or positions found in this bucket.

    #[serde(deserialize_with = "serde_parse_decimal")]
    pub long_count_percent: Decimal,
    #[serde(deserialize_with = "serde_parse_decimal")]
    pub long_count_percent_diff: Decimal,

    /// The percentage of the total number of entries represented by the short
    /// orders or positions found in this bucket.

    #[serde(deserialize_with = "serde_parse_decimal")]
    pub short_count_percent: Decimal,
    #[serde(deserialize_with = "serde_parse_decimal")]
    pub short_count_percent_diff: Decimal,
}