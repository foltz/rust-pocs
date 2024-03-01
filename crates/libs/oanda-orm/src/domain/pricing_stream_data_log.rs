
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "oanda_pricing_stream_data_log")]
pub struct Model {

    #[sea_orm(primary_key)]
    pub id: i32,

    pub timestamp: DateTimeUtc,
    pub session_id: i32,

    pub instrument: String,
    pub payload: Option<String>,

    pub insert_key: Option<DateTimeUtc>,
    pub error: Option<Json>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}