use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue, DbConn, DbErr, EntityTrait};
use sea_orm::sea_query::OnConflict;
use oanda_sdk::support::errors::StreamError;
use crate::domain::enums::{StreamSessionTrigger, StreamSessionStatus};

use crate::domain::{pricing_stream_session};

// -------------------------------------------------------
// - create_session_on_spawn_command >>
// -------------------------------------------------------

pub async fn insert_pricing_stream_session(
    db_conn: &DbConn,
    instrument: String,
    spawn_type: StreamSessionTrigger,

) -> Result<i32, DbErr> { //Box<dyn std::error::Error>> {

    let status = StreamSessionStatus::None.to_string();

    let model = pricing_stream_session::ActiveModel {

        id: ActiveValue::NotSet,

        timestamp: ActiveValue::Set(Utc::now()),

        instrument: ActiveValue::Set(instrument),
        status: ActiveValue::Set(status),

        start_time: ActiveValue::Set(None),
        end_time: ActiveValue::Set(None),

        has_connection_timeout: ActiveValue::Set(false),
        has_heartbeat_timeout: ActiveValue::Set(false),
        has_payload_error: ActiveValue::Set(false),
        // has_response_error: ActiveValue::Set(false),

        spawn_type: ActiveValue::Set(spawn_type.to_string()),
        is_aborted: ActiveValue::Set(false),
        response_error: ActiveValue::Set(None),
    };

    let on_idx_conflict = OnConflict::columns([
        pricing_stream_session::Column::Id,
    ]).update_columns([
        pricing_stream_session::Column::Timestamp,
    ]).to_owned();

    let res = pricing_stream_session::Entity::insert(model)
        .on_conflict(on_idx_conflict.clone())
        // .build(Postgres);
        .exec(db_conn)
        .await?;

    // println!("insert_pricing_stream)subscription::SQL: {:?}", res);
    Ok(res.last_insert_id)
}

pub async fn update_pricing_stream_session_aborted(
    db_conn: &DbConn,
    id: i32
) -> Result<(), DbErr> {

    let find = pricing_stream_session::Entity::find_by_id(id).one(db_conn).await?;

    let mut rec: pricing_stream_session::ActiveModel = find.unwrap().into();
    rec.is_aborted = ActiveValue::Set(true);
    rec.update(db_conn).await?;

    Ok(())
}


// -------------------------------------------------------
// - map_status_message_to_database >>
// -------------------------------------------------------

pub async fn update_pricing_stream_session_status(
    db_conn: &DbConn,
    id: i32,
    status: StreamSessionStatus
) -> Result<(), DbErr> {

    let find = pricing_stream_session::Entity::find_by_id(id).one(db_conn).await?;

    let mut rec: pricing_stream_session::ActiveModel = find.unwrap().into();
    rec.status = ActiveValue::Set(status.to_string());
    rec.update(db_conn).await?;

    Ok(())
}

pub async fn set_pricing_stream_session_connected(
    db_conn: &DbConn,
    id: i32,
) -> Result<(), DbErr> {

    let find = pricing_stream_session::Entity::find_by_id(id).one(db_conn).await?;

    let mut rec: pricing_stream_session::ActiveModel = find.unwrap().into();
    rec.status = ActiveValue::Set(StreamSessionStatus::Connected.to_string());
    rec.start_time = ActiveValue::Set(Some(Utc::now()));
    rec.update(db_conn).await?;

    Ok(())
}

pub async fn set_pricing_stream_session_disconnected(
    db_conn: &DbConn,
    id: i32,
) -> Result<(), DbErr> {

    let find = pricing_stream_session::Entity::find_by_id(id).one(db_conn).await?;

    let mut rec: pricing_stream_session::ActiveModel = find.unwrap().into();
    rec.status = ActiveValue::Set(StreamSessionStatus::Disconnected.to_string());
    rec.end_time = ActiveValue::Set(Some(Utc::now()));
    rec.update(db_conn).await?;

    Ok(())
}

pub async fn set_pricing_stream_session_connection_timeout(
    db_conn: &DbConn,
    id: i32,
) -> Result<(), DbErr> {

    let find = pricing_stream_session::Entity::find_by_id(id).one(db_conn).await?;

    let mut rec: pricing_stream_session::ActiveModel = find.unwrap().into();
    rec.status = ActiveValue::Set(StreamSessionStatus::ConnectionTimeout.to_string());
    rec.has_connection_timeout = ActiveValue::Set(true);
    rec.update(db_conn).await?;

    Ok(())
}

pub async fn set_pricing_stream_session_heartbeat_timeout(
    db_conn: &DbConn,
    id: i32,
) -> Result<(), DbErr> {

    let find = pricing_stream_session::Entity::find_by_id(id).one(db_conn).await?;

    let mut rec: pricing_stream_session::ActiveModel = find.unwrap().into();
    rec.status = ActiveValue::Set(StreamSessionStatus::HeartbeatTimeout.to_string());
    rec.has_heartbeat_timeout = ActiveValue::Set(true);
    rec.update(db_conn).await?;

    Ok(())
}

// -------------------------------------------------------
// - pricing_stream_import_task >>
// -------------------------------------------------------

pub async fn set_pricing_stream_session_payload_error(
    db_conn: &DbConn,
    id: i32,
) -> Result<(), DbErr> {

    let find = pricing_stream_session::Entity::find_by_id(id).one(db_conn).await?;

    let mut rec: pricing_stream_session::ActiveModel = find.unwrap().into();
    rec.status = ActiveValue::Set(StreamSessionStatus::PayloadError.to_string());
    rec.has_payload_error = ActiveValue::Set(true);
    // rec.error = ActiveValue::Set(Some(err.to_json()));
    rec.update(db_conn).await?;

    Ok(())
}

pub async fn set_pricing_stream_session_response_error(
    db_conn: &DbConn,
    id: i32,
    err: &StreamError,
) -> Result<(), DbErr> {

    let find = pricing_stream_session::Entity::find_by_id(id).one(db_conn).await?;

    let mut rec: pricing_stream_session::ActiveModel = find.unwrap().into();
    rec.status = ActiveValue::Set(StreamSessionStatus::ResponseError.to_string());
    // rec.has_response_error = ActiveValue::Set(true);
    rec.response_error = ActiveValue::Set(Some(err.to_json()));
    rec.update(db_conn).await?;

    Ok(())
}

