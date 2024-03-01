
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "oanda_candlestick_data")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    pub timestamp: DateTimeUtc, // - or: DateTime<Utc>
    pub import_type: String,

    // - TODO: consider creating a composite key...
    pub instrument: String,
    pub granularity: String,

    pub time: DateTimeUtc, // - or: DateTime<Utc>
    pub volume: i32,

    #[sea_orm(column_type = "Decimal(Some((9, 6)))")]
    pub bid_o: Decimal,
    #[sea_orm(column_type = "Decimal(Some((9, 6)))")]
    pub bid_h: Decimal,
    #[sea_orm(column_type = "Decimal(Some((9, 6)))")]
    pub bid_l: Decimal,
    #[sea_orm(column_type = "Decimal(Some((9, 6)))")]
    pub bid_c: Decimal,

    #[sea_orm(column_type = "Decimal(Some((9, 6)))")]
    pub ask_o: Decimal,
    #[sea_orm(column_type = "Decimal(Some((9, 6)))")]
    pub ask_h: Decimal,
    #[sea_orm(column_type = "Decimal(Some((9, 6)))")]
    pub ask_l: Decimal,
    #[sea_orm(column_type = "Decimal(Some((9, 6)))")]
    pub ask_c: Decimal,

    // #[sea_orm(column_type = "Decimal(Some((9, 6)))")]
    // pub mid_o: Decimal,
    // #[sea_orm(column_type = "Decimal(Some((9, 6)))")]
    // pub mid_h: Decimal,
    // #[sea_orm(column_type = "Decimal(Some((9, 6)))")]
    // pub mid_l: Decimal,
    // #[sea_orm(column_type = "Decimal(Some((9, 6)))")]
    // pub mid_c: Decimal,
    //
    // pub bid: Json,
    // pub ask: Json,
    // pub mid: Json,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}


/*

candlestick table:

granularity: S2, etc
instrument: "EUR_USD"
time,



// - CandleStickData: o,h,l,c
could do:
bid_o,
bid_h,
bid_l,
bid_c

or:

bid {o,h,l,c} as json(?)
 - try both and figure out which is more performant....
 


//     /// The start time of the candlestick
//     pub time: DateTime<Utc>,
// 
//     /// The candlestick data based on bids. Only provided if bid-based candles
//     /// were requested.
//     pub bid: Option<CandlestickData>,
// 
//     /// The candlestick data based on asks. Only provided if ask-based candles
//     /// were requested.
//     pub ask: Option<CandlestickData>,
// 
//     /// The candlestick data based on midpoints. Only provided if midpoint-based
//     /// candles were requested.
//     pub mid: Option<CandlestickData>,
// 
//     /// The number of prices created during the time-range represented by the
//     /// candlestick.
//     pub volume: i32,

 */