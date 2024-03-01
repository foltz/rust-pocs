
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "oanda_position_book_query_log")]
pub struct Model {

    #[sea_orm(primary_key)]
    pub id: i32,

    pub timestamp: DateTimeUtc, // - or: DateTime<Utc>
    pub import_type: String,
    
    pub instrument: String,
    pub frequency: i32,
    
    pub time: Option<DateTimeUtc>, // - or: DateTime<Utc>

    pub insert_id: Option<i32>,
    pub query_error: Option<Json>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}