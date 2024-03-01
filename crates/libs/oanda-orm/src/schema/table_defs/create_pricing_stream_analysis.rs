use sea_orm::{error::*, sea_query, DbConn, ExecResult};
use sea_query::{ColumnDef, Table};
use crate::domain::pricing_stream_data_log;
use crate::schema::db_cmds::create_table;

pub async fn create_pricing_stream_analysis(conn: &DbConn) -> Result<ExecResult, DbErr> {

    let stmt = Table::create()
        .table(pricing_stream_data_log::Entity)
        .col(
            ColumnDef::new(pricing_stream_data_log::Column::Id)
                .integer().not_null()
                .auto_increment().primary_key(),
        )

        .col(ColumnDef::new(pricing_stream_data_log::Column::Timestamp).timestamp_with_time_zone().not_null())
        .col(ColumnDef::new(pricing_stream_data_log::Column::SessionId).integer().not_null())

        .col(ColumnDef::new(pricing_stream_data_log::Column::Instrument).string().not_null())
        .col(ColumnDef::new(pricing_stream_data_log::Column::Payload).string())

        .col(ColumnDef::new(pricing_stream_data_log::Column::InsertKey).timestamp_with_time_zone())
        .col(ColumnDef::new(pricing_stream_data_log::Column::Error).json())

        .to_owned();

    create_table(conn, &stmt, pricing_stream_data_log::Entity).await
}