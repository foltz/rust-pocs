
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "oanda_candlestick_query_log")]
pub struct Model {

    #[sea_orm(primary_key)]
    pub id: i32,

    pub timestamp: DateTimeUtc, // - or: DateTime<Utc>
    pub import_type: String,

    // - TODO: consider creating a composite key...
    pub instrument: String,
    pub granularity: String,
    
    pub from: Option<DateTimeUtc>, // - or: DateTime<Utc>
    pub to: Option<DateTimeUtc>, // - or: DateTime<Utc>

    pub result_count: Option<i32>,
    pub first_result: Option<DateTimeUtc>, // - or: DateTime<Utc>
    pub last_result: Option<DateTimeUtc>, // - or: DateTime<Utc>

    pub insert_id: Option<i32>,
    pub query_error: Option<Json>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}