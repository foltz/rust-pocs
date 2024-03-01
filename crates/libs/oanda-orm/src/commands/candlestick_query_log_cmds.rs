use chrono::Utc;
use sea_orm::{ActiveValue, DbConn, DbErr, EntityTrait};
use sea_orm::prelude::DateTimeUtc;
use sea_orm::sea_query::OnConflict;
use oanda_sdk::models::candlestick_models::{CandlestickQueryData, CandlestickSnapshot};
use oanda_sdk::queries::candlestick_query::CandlestickQuery;
use oanda_sdk::support::errors::QueryError;

use crate::domain::candlestick_query_log;
use crate::domain::enums::ImportType;

pub async fn insert_candlestick_query_error(
    db_conn: &DbConn,
    import_type: &ImportType,
    query: &CandlestickQuery,
    error: &QueryError,

) -> Result<(), DbErr> {

    let import_type = import_type.to_string();

    let instrument = query.instrument.clone();
    let granularity = query.granularity.to_string();

    let from = query.from.clone();
    let to = query.to.clone();

    // let query_error = Some(query_response.error.clone().unwrap().to_json());
    let query_error = Some(error.to_json());

    let model = candlestick_query_log::ActiveModel {

        id: ActiveValue::NotSet,
        timestamp: ActiveValue::Set(Utc::now()),

        import_type: ActiveValue::Set(import_type),

        instrument: ActiveValue::Set(instrument),
        granularity: ActiveValue::Set(granularity),

        from: ActiveValue::Set(from),
        to: ActiveValue::Set(to),

        result_count: ActiveValue::Set(None),
        first_result: ActiveValue::Set(None),
        last_result: ActiveValue::Set(None),

        insert_id: ActiveValue::Set(None),
        query_error: ActiveValue::Set(query_error),
        // query_error: Default::default(), // - could also be this...
    };

    // When `id` column have conflicting value, do nothing
    let on_conflict = OnConflict::column(
        candlestick_query_log::Column::Id
    ).do_nothing().to_owned();

    let res = candlestick_query_log::Entity::insert(model)
        .on_conflict(on_conflict.clone())
        // .build(Postgres);
        .exec(db_conn)
        .await?;

    println!("insert_candlestick_query_error::SQL: {:?}", res);
    Ok(())
}




pub async fn insert_candlestick_query_success(
    db_conn: &DbConn,
    import_type: &ImportType,
    query: &CandlestickQuery,
    data: &CandlestickQueryData,
    insert_id: Option<i32>,

) -> Result<(), DbErr> {

    let import_type = import_type.to_string();

    let instrument = query.instrument.clone();
    let granularity = query.granularity.to_string();

    let from = query.from.clone();
    let to = query.to.clone();

    let completed: Vec<&CandlestickSnapshot> = data.candles.iter()
        .filter(|&c| c.complete == true)
        .collect();

    let result_count = completed.len() as i32;
    let first_result: Option<DateTimeUtc> = match completed.first() {
        None => None,
        Some(candle) => Some(candle.time)
    };
    let last_result: Option<DateTimeUtc> = match completed.last() {
        None => None,
        Some(candle) => Some(candle.time)
    };

    let model = candlestick_query_log::ActiveModel {

        id: ActiveValue::NotSet,

        timestamp: ActiveValue::Set(Utc::now()),
        import_type: ActiveValue::Set(import_type),

        instrument: ActiveValue::Set(instrument),
        granularity: ActiveValue::Set(granularity),

        from: ActiveValue::Set(from),
        to: ActiveValue::Set(to),

        result_count: ActiveValue::Set(Some(result_count)),
        first_result: ActiveValue::Set(first_result),
        last_result: ActiveValue::Set(last_result),

        insert_id: ActiveValue::Set(insert_id),
        query_error: ActiveValue::Set(None),
    };

    // When `id` column have conflicting value, do nothing
    let on_conflict = OnConflict::column(
        candlestick_query_log::Column::Id
    ).do_nothing().to_owned();

    let _ = candlestick_query_log::Entity::insert(model)
        .on_conflict(on_conflict.clone())
        // .build(Postgres);
        .exec(db_conn)
        .await?;

    // println!("insert_candlestick_query_success::SQL: {:?}", res);
    Ok(())
}