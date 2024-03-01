use sea_orm::{error::*, sea_query, DbConn, ExecResult};
use sea_query::{ColumnDef, Table};
use crate::domain::position_book_query_log;
use crate::schema::db_cmds::create_table;

pub async fn create_position_book_query_log(conn: &DbConn) -> Result<ExecResult, DbErr> {

    let stmt = Table::create()
        .table(position_book_query_log::Entity)
        .col(
            ColumnDef::new(position_book_query_log::Column::Id)
                .integer().not_null()
                .auto_increment().primary_key(),
        )

        .col(ColumnDef::new(position_book_query_log::Column::Timestamp).timestamp_with_time_zone().not_null())
        .col(ColumnDef::new(position_book_query_log::Column::ImportType).string().not_null())


        .col(ColumnDef::new(position_book_query_log::Column::Instrument).string().not_null())
        .col(ColumnDef::new(position_book_query_log::Column::Frequency).integer().not_null())

        .col(ColumnDef::new(position_book_query_log::Column::Time).timestamp_with_time_zone())

        .col(ColumnDef::new(position_book_query_log::Column::InsertId).integer())
        .col(ColumnDef::new(position_book_query_log::Column::QueryError).json())

        .to_owned();

    create_table(conn, &stmt, position_book_query_log::Entity).await
}