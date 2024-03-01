use sea_orm::{error::*, sea_query, DbConn, ExecResult};
use sea_query::{ColumnDef, Table};
use crate::domain::{pricing_stream_data_log, pricing_stream_session};
use crate::schema::db_cmds::create_table;

pub async fn create_pricing_stream_session(conn: &DbConn) -> Result<ExecResult, DbErr> {

    let stmt = Table::create()
        .table(pricing_stream_session::Entity)
        .col(
            ColumnDef::new(pricing_stream_session::Column::Id)
                .integer().not_null()
                .auto_increment().primary_key(),
        )

        .col(ColumnDef::new(pricing_stream_session::Column::Timestamp).timestamp_with_time_zone().not_null())

        .col(ColumnDef::new(pricing_stream_session::Column::Instrument).string().not_null())
        .col(ColumnDef::new(pricing_stream_session::Column::Status).string().not_null())

        .col(ColumnDef::new(pricing_stream_session::Column::StartTime).timestamp_with_time_zone())
        .col(ColumnDef::new(pricing_stream_session::Column::EndTime).timestamp_with_time_zone())

        .col(ColumnDef::new(pricing_stream_session::Column::HasConnectionTimeout).boolean().not_null())
        .col(ColumnDef::new(pricing_stream_session::Column::HasHeartbeatTimeout).boolean().not_null())
        .col(ColumnDef::new(pricing_stream_session::Column::HasPayloadError).boolean().not_null())
        // .col(ColumnDef::new(pricing_stream_session::Column::HasResponseError).boolean().not_null())

        .col(ColumnDef::new(pricing_stream_session::Column::SpawnType).string().not_null())
        .col(ColumnDef::new(pricing_stream_session::Column::IsAborted).boolean().not_null())
        .col(ColumnDef::new(pricing_stream_session::Column::ResponseError).json())
        .to_owned();

    create_table(conn, &stmt, pricing_stream_session::Entity).await
}