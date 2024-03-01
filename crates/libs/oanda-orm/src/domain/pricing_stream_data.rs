
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "oanda_pricing_stream_data")]
pub struct Model {

    // pub id: i32,

    pub timestamp: DateTimeUtc,

    // - this is for the hyper_table:

    #[sea_orm(primary_key, auto_increment=false)]
    pub instrument: String,

    #[sea_orm(primary_key, auto_increment=false)]
    pub time: DateTimeUtc,

    #[sea_orm(column_type = "Decimal(Some((9, 6)))")]
    pub bid_price: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub bid_liquidity: Decimal,
    // #[sea_orm(column_type = "Decimal(Some((9, 6)))")]
    // pub closeout_bid: Decimal,

    #[sea_orm(column_type = "Decimal(Some((9, 6)))")]
    pub ask_price: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub ask_liquidity: Decimal,
    // #[sea_orm(column_type = "Decimal(Some((9, 6)))")]
    // pub closeout_ask: Decimal,

}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}


/*

instrument: "EUR_USD"
time,
price
bucket_width

buckets (JSON)
 - price, long_count_percent, short_count_percent

    /// The order book’s instrument
    pub instrument: String,

    /// The time when the order book snapshot was created.
    pub time: DateTime<Utc>,

    /// The price (midpoint) for the order book’s instrument at the time of the
    /// order book snapshot

    #[serde(deserialize_with = "serde_parse_f32")]
    pub price: f32,

    /// The price width for each bucket. Each bucket covers the price range from
    /// the bucket’s price to the bucket’s price + bucketWidth.

    #[serde(deserialize_with = "serde_parse_f32")]
    pub bucket_width: f32,

    /// The partitioned order book, divided into buckets using a default bucket
    /// width. These buckets are only provided for price ranges which actually
    /// contain order or position data.
    pub buckets: Vec<OrderBookBucket>

 */