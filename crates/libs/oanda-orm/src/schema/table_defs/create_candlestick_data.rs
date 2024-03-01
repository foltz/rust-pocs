use sea_orm::{error::*, sea_query, DbConn, ExecResult};
use sea_query::{ColumnDef, Table};
use crate::domain::candlestick_data;
use crate::schema::db_cmds::{create_table, create_index};

pub async fn create_candlestick_data(conn: &DbConn) -> Result<ExecResult, DbErr> {

    let table_stmt = Table::create()
        .table(candlestick_data::Entity)
        .col(
            ColumnDef::new(candlestick_data::Column::Id)
                .integer().not_null()
                .auto_increment().primary_key(),
        )

        .col(ColumnDef::new(candlestick_data::Column::Timestamp).timestamp_with_time_zone().not_null())
        .col(ColumnDef::new(candlestick_data::Column::ImportType).string().not_null())

        .col(ColumnDef::new(candlestick_data::Column::Instrument).string().not_null())
        .col(ColumnDef::new(candlestick_data::Column::Granularity).string().not_null())

        .col(ColumnDef::new(candlestick_data::Column::Time).timestamp_with_time_zone().not_null())
        .col(ColumnDef::new(candlestick_data::Column::Volume).integer().not_null())

        .col(ColumnDef::new(candlestick_data::Column::BidO).decimal_len(9, 6).not_null())
        .col(ColumnDef::new(candlestick_data::Column::BidH).decimal_len(9, 6).not_null())
        .col(ColumnDef::new(candlestick_data::Column::BidL).decimal_len(9, 6).not_null())
        .col(ColumnDef::new(candlestick_data::Column::BidC).decimal_len(9, 6).not_null())

        .col(ColumnDef::new(candlestick_data::Column::AskO).decimal_len(9, 6).not_null())
        .col(ColumnDef::new(candlestick_data::Column::AskH).decimal_len(9, 6).not_null())
        .col(ColumnDef::new(candlestick_data::Column::AskL).decimal_len(9, 6).not_null())
        .col(ColumnDef::new(candlestick_data::Column::AskC).decimal_len(9, 6).not_null())

        // .col(ColumnDef::new(candlestick::Column::MidO).decimal_len(9, 6).not_null())
        // .col(ColumnDef::new(candlestick::Column::MidH).decimal_len(9, 6).not_null())
        // .col(ColumnDef::new(candlestick::Column::MidL).decimal_len(9, 6).not_null())
        // .col(ColumnDef::new(candlestick::Column::MidC).decimal_len(9, 6).not_null())
        // 
        // .col(ColumnDef::new(candlestick::Column::Bid).json().not_null())
        // .col(ColumnDef::new(candlestick::Column::Ask).json().not_null())
        // .col(ColumnDef::new(candlestick::Column::Mid).json().not_null())

        .to_owned();

    create_table(conn, &table_stmt, candlestick_data::Entity).await?;

    let idx_stmt = sea_query::Index::create()
        .name("idx-candlestick-data-granularity-instrument-time")
        .table(candlestick_data::Entity)
        .col(candlestick_data::Column::Granularity)
        .col(candlestick_data::Column::Instrument)
        .col(candlestick_data::Column::Time)
        .unique()
        .to_owned();

    create_index(conn, &idx_stmt).await
}