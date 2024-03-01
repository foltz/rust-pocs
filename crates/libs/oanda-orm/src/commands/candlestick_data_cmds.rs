use chrono::Utc;
use sea_orm::{ActiveValue, DbConn, DbErr, EntityTrait};
use sea_orm::sea_query::OnConflict;
use oanda_sdk::models::candlestick_models::CandlestickQueryData;
use crate::domain::candlestick_data;
use crate::domain::enums::ImportType;


// fn cd_to_json(cd: &CandlestickData) -> serde_json::Value {
//     json!({"o": cd.o, "h": cd.h, "l": cd.l, "c": cd.c})
//
// }

// fn f32_to_dec(v: f32) -> Decimal {
//     Decimal::from_f32_retain(v).unwrap() // - from_f32 or from_f32_retain
// }


pub async fn insert_candlestick_data(
    db_conn: &DbConn,
    import_type: &ImportType,
    data: &CandlestickQueryData,

) -> Result<Option<i32>, DbErr> { //Box<dyn std::error::Error>> {

    let completed = data.candles.iter()
        .filter(|&c| c.complete == true);

    // println!("all: {} complete: {}", data.candles.len(), completed.count());

    let mut inserts = Vec::new();
    completed.for_each(|c| {

        let bid = c.bid.as_ref().unwrap();
        let ask = c.ask.as_ref().unwrap();

        inserts.push(candlestick_data::ActiveModel {

            id: ActiveValue::NotSet,

            timestamp: ActiveValue::Set(Utc::now()),
            import_type: ActiveValue::Set(import_type.to_string()),

            instrument: ActiveValue::Set(data.instrument.clone()),
            granularity: ActiveValue::Set(data.granularity.to_string()),

            time: ActiveValue::Set(c.time),
            volume: ActiveValue::Set(c.volume),

            bid_o: ActiveValue::Set(bid.o),
            bid_h: ActiveValue::Set(bid.h),
            bid_l: ActiveValue::Set(bid.l),
            bid_c: ActiveValue::Set(bid.c),

            ask_o: ActiveValue::Set(ask.o),
            ask_h: ActiveValue::Set(ask.h),
            ask_l: ActiveValue::Set(ask.l),
            ask_c: ActiveValue::Set(ask.c),
        });
    });

    // println!("orig: {}  insert: {}", data.candles.len(), inserts.len());
    if inserts.len() > 0 {

        // When `id` column have conflicting value, do nothing
        // let on_id_conflict = OnConflict::column(candlestick::Column::Id).do_nothing().to_owned();
        let on_idx_conflict = OnConflict::columns([
            candlestick_data::Column::Granularity,
            candlestick_data::Column::Instrument,
            candlestick_data::Column::Time,
        ]).update_columns([
            candlestick_data::Column::Timestamp,
        ]).to_owned();

        let res = candlestick_data::Entity::insert_many(inserts)
            // .on_conflict(on_id_conflict)
            .on_conflict(on_idx_conflict)
            // .build(Postgres);
            .exec(db_conn)
            .await?;


        // println!("insert_candlestick_result::SQL: {:?}", res);
        return Ok(Some(res.last_insert_id))
    }

    return Ok(None)
}