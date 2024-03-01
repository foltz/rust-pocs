use sea_orm::{ColumnTrait, EntityName, IntoIdentity};
use sea_orm::sea_query::{ColumnDef, IntoIden};
use crate::domain::candlestick_query_log;

#[test]
fn show_table_name() {
    let ent = candlestick_query_log::Entity;
    println!("table name: {}", ent.table_name());

}


#[test]
fn show_col_name() {
    let ent = candlestick_query_log::Column::Timestamp;
    // ent
    println!("column name: {}", ColumnDef::new(ent).get_column_name());

}