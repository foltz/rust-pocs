use chrono::Utc;
use sea_orm::{ActiveValue, DbConn, DbErr, EntityTrait};
use sea_orm::prelude::DateTimeUtc;
use sea_orm::sea_query::OnConflict;
use oanda_sdk::support::errors::StreamError;
use crate::domain::pricing_stream_data_log;

pub async fn insert_pricing_stream_data_log_error(
    db_conn: &DbConn,
    session_id: i32,

    instrument: String,
    payload: Option<String>,

    error: &StreamError,

) -> Result<(), DbErr> {

    let query_error = Some(error.to_json());

    let formatted_payload = match payload {
        None => "None".to_string(),
        Some(str) => format!("({str})")
    };

    let model = pricing_stream_data_log::ActiveModel {

        id: ActiveValue::NotSet,

        timestamp: ActiveValue::Set(Utc::now()),
        session_id: ActiveValue::Set(session_id),

        instrument: ActiveValue::Set(instrument),
        payload: ActiveValue::Set(Some(formatted_payload)),

        insert_key: ActiveValue::Set(None),
        error: ActiveValue::Set(query_error),
    };

    // When `id` column have conflicting value, do nothing
    let on_conflict = OnConflict::column(
        pricing_stream_data_log::Column::Id
    ).do_nothing().to_owned();

    let res = pricing_stream_data_log::Entity::insert(model)
        .on_conflict(on_conflict.clone())
        // .build(Postgres);
        .exec(db_conn)
        .await?;

    println!("insert_pricing_stream_event_error::SQL: {:?}", res);
    Ok(())
}




pub async fn insert_pricing_stream_data_log_success(
    db_conn: &DbConn,
    session_id: i32,

    instrument: String,
    payload: String,

    insert_key: DateTimeUtc,

) -> Result<(), DbErr> {

    // let event_type = event_type.to_string();

    let model = pricing_stream_data_log::ActiveModel {

        id: ActiveValue::NotSet,

        timestamp: ActiveValue::Set(Utc::now()),
        session_id: ActiveValue::Set(session_id),

        instrument: ActiveValue::Set(instrument),

        // event_type: ActiveValue::Set(Some(event_type)),
        payload: ActiveValue::Set(Some(payload)),

        insert_key: ActiveValue::Set(Some(insert_key)),
        error: ActiveValue::Set(None),
    };

    // When `id` column have conflicting value, do nothing
    let on_conflict = OnConflict::column(
        pricing_stream_data_log::Column::Id
    ).do_nothing().to_owned();

    let res = pricing_stream_data_log::Entity::insert(model)
        .on_conflict(on_conflict.clone())
        // .build(Postgres);
        .exec(db_conn)
        .await?;

    // println!("insert_pricing_stream_event_success::SQL: {:?}", res);
    Ok(())
}