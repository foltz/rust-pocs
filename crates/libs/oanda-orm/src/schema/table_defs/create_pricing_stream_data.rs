use sea_orm::{error::*, sea_query, DbConn, ExecResult};
use sea_query::{ColumnDef, Table};
use crate::domain::pricing_stream_data;
use crate::schema::db_cmds::{create_table, create_hypertable};

pub async fn create_pricing_stream_data(conn: &DbConn) -> Result<ExecResult, DbErr> {

    let table_stmt = Table::create()
        .table(pricing_stream_data::Entity)

        .col(ColumnDef::new(pricing_stream_data::Column::Timestamp).timestamp_with_time_zone().not_null())

        .col(ColumnDef::new(pricing_stream_data::Column::Instrument).string().not_null())
        .col(ColumnDef::new(pricing_stream_data::Column::Time).timestamp_with_time_zone().not_null())

        .col(ColumnDef::new(pricing_stream_data::Column::BidPrice).decimal_len(9, 6).not_null())
        .col(ColumnDef::new(pricing_stream_data::Column::BidLiquidity).decimal_len(10, 2).not_null())
        // .col(ColumnDef::new(pricing_stream_data::Column::CloseoutBid).decimal_len(9, 6).not_null())

        .col(ColumnDef::new(pricing_stream_data::Column::AskPrice).decimal_len(9, 6).not_null())
        .col(ColumnDef::new(pricing_stream_data::Column::AskLiquidity).decimal_len(10, 2).not_null())
        // .col(ColumnDef::new(pricing_stream_data::Column::CloseoutAsk).decimal_len(9, 6).not_null())

        .primary_key(
            sea_query::Index::create()
                .name("pk-oanda_pricing_stream_data")
                .col(pricing_stream_data::Column::Instrument)
                .col(pricing_stream_data::Column::Time)
        )
        .to_owned();

    create_table(conn, &table_stmt, pricing_stream_data::Entity).await?;

    create_hypertable(
        conn,
        pricing_stream_data::Entity,
        pricing_stream_data::Column::Time
    ).await
}