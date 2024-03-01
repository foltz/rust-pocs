use chrono::Utc;
use sea_orm::{ActiveValue, DbConn, DbErr, EntityTrait};
use sea_orm::sea_query::OnConflict;
use oanda_sdk::queries::shared_book_query::SharedBookQuery;
use oanda_sdk::support::errors::QueryError;

use crate::domain::order_book_query_log;
use crate::domain::enums::ImportType;

pub async fn insert_order_book_query_error(
    db_conn: &DbConn,
    import_type: &ImportType,
    query: &SharedBookQuery,
    error: &QueryError,

) -> Result<(), DbErr> {

    let import_type = import_type.to_string();

    let instrument = query.instrument.clone();
    let frequency = query.frequency.clone();
    let time = query.time.clone();

    let query_error = Some(error.to_json());

    let model = order_book_query_log::ActiveModel {

        id: ActiveValue::NotSet,

        timestamp: ActiveValue::Set(Utc::now()),
        import_type: ActiveValue::Set(import_type),

        instrument: ActiveValue::Set(instrument),
        frequency: ActiveValue::Set(frequency),

        time: ActiveValue::Set(time),

        insert_id: ActiveValue::Set(None),
        query_error: ActiveValue::Set(query_error),
        // query_error: Default::default(), // - could also be this...
    };

    // When `id` column have conflicting value, do nothing
    let on_conflict = OnConflict::column(
        order_book_query_log::Column::Id
    ).do_nothing().to_owned();

    let res = order_book_query_log::Entity::insert(model)
        .on_conflict(on_conflict.clone())
        // .build(Postgres);
        .exec(db_conn)
        .await?;

    println!("insert_order_book_query_error::SQL: {:?}", res);
    Ok(())
}




pub async fn insert_order_book_query_success(
    db_conn: &DbConn,
    import_type: &ImportType,
    query: &SharedBookQuery,
    insert_id: i32,

) -> Result<(), DbErr> {

    let import_type = import_type.to_string();

    let instrument = query.instrument.clone();
    let frequency = query.frequency.clone();

    let time = query.time.clone();

    let model = order_book_query_log::ActiveModel {

        id: ActiveValue::NotSet,

        timestamp: ActiveValue::Set(Utc::now()),
        import_type: ActiveValue::Set(import_type),

        instrument: ActiveValue::Set(instrument),
        frequency: ActiveValue::Set(frequency),

        time: ActiveValue::Set(time),

        insert_id: ActiveValue::Set(Some(insert_id)),
        query_error: ActiveValue::Set(None),
        // query_error: Default::default(), // - could also be this...
    };

    // When `id` column have conflicting value, do nothing
    let on_conflict = OnConflict::column(
        order_book_query_log::Column::Id
    ).do_nothing().to_owned();

    let res = order_book_query_log::Entity::insert(model)
        .on_conflict(on_conflict.clone())
        // .build(Postgres);
        .exec(db_conn)
        .await?;

    // println!("insert_order_book_query_success::SQL: {:?}", res);
    Ok(())
}