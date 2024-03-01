use chrono::Utc;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use sea_orm::{ActiveModelTrait, ActiveValue, DbConn, DbErr, EntityTrait};
use sea_orm::sea_query::OnConflict;
use oanda_sdk::models::shared_book_models::{SharedBookQueryData, SharedBookBucketWithDiff};
use crate::domain::position_book_data;
use crate::domain::enums::ImportType;


pub async fn insert_position_book_data(
    db_conn: &DbConn,
    import_type: &ImportType,
    data: &SharedBookQueryData,

) -> Result<i32, DbErr> { //Box<dyn std::error::Error>> {

    let instrument = data.instrument.clone();
    let time = data.time;
    // let price = data.price;
    let price = data.price.unwrap_or_else(|| Decimal::new(0, 1));

    let bucket_width = data.bucket_width;
    let buckets = serde_json::to_value(data.buckets.clone()).unwrap();

    let model = position_book_data::ActiveModel {

        id: ActiveValue::NotSet,

        timestamp: ActiveValue::Set(Utc::now()),
        import_type: ActiveValue::Set(import_type.to_string()),

        instrument: ActiveValue::Set(instrument),
        time: ActiveValue::Set(time),

        price: ActiveValue::Set(price),
        bucket_width: ActiveValue::Set(bucket_width),
        buckets: ActiveValue::Set(buckets),

        // - analysis:

        bucket_diffs: Default::default(),
        bucket_diff_count: Default::default(),
    };

    let on_idx_conflict = OnConflict::columns([
        position_book_data::Column::Instrument,
        position_book_data::Column::Time,
    ]).update_columns([
        position_book_data::Column::Timestamp,
    ]).to_owned();

    let res = position_book_data::Entity::insert(model)
        .on_conflict(on_idx_conflict.clone())
        // .build(Postgres);
        .exec(db_conn)
        .await?;

    // println!("insert_position_book::SQL: {:?}", res);

    Ok(res.last_insert_id)
}

pub async fn update_position_book_with_diffs(
    db_conn: &DbConn,
    mut model: position_book_data::ActiveModel,
    data: &Vec<SharedBookBucketWithDiff>,

) -> Result<(), DbErr> { //Box<dyn std::error::Error>> {

    let bucket_change_count: Option<i32> = data.len().to_i32();
    let bucket_diffs = Some(serde_json::to_value(data).unwrap());

    model.bucket_diff_count = ActiveValue::Set(bucket_change_count);
    model.bucket_diffs = ActiveValue::Set(bucket_diffs);

    model.update(db_conn).await?;
    Ok(())
}