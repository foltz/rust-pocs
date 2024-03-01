
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "oanda_pricing_stream_session")]
pub struct Model {

    #[sea_orm(primary_key)]
    pub id: i32,

    pub timestamp: DateTimeUtc,

    pub instrument: String,
    pub status: String,

    pub start_time: Option<DateTimeUtc>, // - time connected
    pub end_time: Option<DateTimeUtc>, // - time disconnected

    pub has_connection_timeout: bool,
    pub has_heartbeat_timeout: bool,
    pub has_payload_error: bool,
    // pub has_response_error: bool,

    pub spawn_type: String,
    pub is_aborted: bool,

    pub response_error: Option<Json>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}