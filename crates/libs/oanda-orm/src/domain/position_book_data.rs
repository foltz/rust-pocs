
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "oanda_position_book_data")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    pub timestamp: DateTimeUtc, // - or: DateTime<Utc>
    pub import_type: String,

    pub instrument: String,
    pub time: DateTimeUtc, // - or: DateTime<Utc>

    #[sea_orm(column_type = "Decimal(Some((9, 6)))")]
    pub price: Decimal,
    #[sea_orm(column_type = "Decimal(Some((9, 6)))")]
    pub bucket_width: Decimal,
    pub buckets: Json,

    // - ANALYSIS DATA:

    pub bucket_diff_count: Option<i32>,
    pub bucket_diffs: Option<Json>,
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
    pub buckets: Vec<PositionBookBucket>

 */