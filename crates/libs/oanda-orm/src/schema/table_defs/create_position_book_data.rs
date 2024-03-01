use sea_orm::{error::*, sea_query, DbConn, ExecResult};
use sea_query::{ColumnDef, Table};
use crate::domain::position_book_data;
use crate::schema::db_cmds::{create_table, create_index};

pub async fn create_position_book_data(conn: &DbConn) -> Result<ExecResult, DbErr> {

    let table_stmt = Table::create()
        .table(position_book_data::Entity)
        .col(
            ColumnDef::new(position_book_data::Column::Id)
                .integer().not_null()
                .auto_increment().primary_key(),
        )

        .col(ColumnDef::new(position_book_data::Column::Timestamp).timestamp_with_time_zone().not_null())
        .col(ColumnDef::new(position_book_data::Column::ImportType).string().not_null())

        .col(ColumnDef::new(position_book_data::Column::Instrument).string().not_null())
        .col(ColumnDef::new(position_book_data::Column::Time).timestamp_with_time_zone().not_null())

        .col(ColumnDef::new(position_book_data::Column::Price).decimal_len(9, 6).not_null())
        .col(ColumnDef::new(position_book_data::Column::BucketWidth).decimal_len(9, 6).not_null())

        .col(ColumnDef::new(position_book_data::Column::Buckets).json().not_null())

        // - Analysis

        .col(ColumnDef::new(position_book_data::Column::BucketDiffCount).integer())
        .col(ColumnDef::new(position_book_data::Column::BucketDiffs).json())

        .to_owned();

    create_table(conn, &table_stmt, position_book_data::Entity).await?;

    let idx_stmt = sea_query::Index::create()
        .name("idx-position-book-data-instrument-time")
        .table(position_book_data::Entity)
        // .col(position_book::Column::Granularity)
        .col(position_book_data::Column::Instrument)
        .col(position_book_data::Column::Time)
        .unique()
        .to_owned();

    create_index(conn, &idx_stmt).await
}