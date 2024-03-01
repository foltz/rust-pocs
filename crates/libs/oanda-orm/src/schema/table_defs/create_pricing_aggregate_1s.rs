use sea_orm::{error::*, sea_query, DbConn, ExecResult};
use sea_query::{ColumnDef, Table};
use crate::domain::pricing_aggregate_1s;
use crate::schema::db_cmds::{create_table, create_hypertable};

pub async fn create_pricing_aggregate_1s(conn: &DbConn) -> Result<ExecResult, DbErr> {

    let table_stmt = Table::create()
        .table(pricing_aggregate_1s::Entity)

        .col(ColumnDef::new(pricing_aggregate_1s::Column::Timestamp).timestamp_with_time_zone().not_null())

        .col(ColumnDef::new(pricing_aggregate_1s::Column::Instrument).string().not_null())
        .col(ColumnDef::new(pricing_aggregate_1s::Column::Time).timestamp_with_time_zone().not_null())

        .col(ColumnDef::new(pricing_aggregate_1s::Column::Volume).integer().not_null())
        .col(ColumnDef::new(pricing_aggregate_1s::Column::BidOpen).decimal_len(9, 6).not_null())
        .col(ColumnDef::new(pricing_aggregate_1s::Column::BidClose).decimal_len(9, 6).not_null())
        .col(ColumnDef::new(pricing_aggregate_1s::Column::BidHigh).decimal_len(9, 6).not_null())
        .col(ColumnDef::new(pricing_aggregate_1s::Column::BidLow).decimal_len(9, 6).not_null())
        .col(ColumnDef::new(pricing_aggregate_1s::Column::BidAvg).decimal_len(9, 6).not_null())

        .col(ColumnDef::new(pricing_aggregate_1s::Column::AskOpen).decimal_len(9, 6).not_null())
        .col(ColumnDef::new(pricing_aggregate_1s::Column::AskClose).decimal_len(9, 6).not_null())
        .col(ColumnDef::new(pricing_aggregate_1s::Column::AskHigh).decimal_len(9, 6).not_null())
        .col(ColumnDef::new(pricing_aggregate_1s::Column::AskLow).decimal_len(9, 6).not_null())
        .col(ColumnDef::new(pricing_aggregate_1s::Column::AskAvg).decimal_len(9, 6).not_null())

        .primary_key(
            sea_query::Index::create()
                .name("pk-oanda_pricing_aggregate_1s")
                .col(pricing_aggregate_1s::Column::Instrument)
                .col(pricing_aggregate_1s::Column::Time)
        )
        .to_owned();

    create_table(conn, &table_stmt, pricing_aggregate_1s::Entity).await?;
    
    create_hypertable(
        conn,
        pricing_aggregate_1s::Entity,
        pricing_aggregate_1s::Column::Time
    ).await
}