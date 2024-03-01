use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
// use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use sea_orm::{ActiveValue, DatabaseConnection, DbErr, EntityTrait, QueryTrait};
use sea_orm::DatabaseBackend::Postgres;
use sea_orm::sea_query::OnConflict;
use serde_json::json;
use oanda_orm::domain::candlestick_data;
use oanda_sdk::models::candlestick_models::{Candlestick, CandlestickQueryData};
// use crate::domain::oanda::candlestick;

fn cd_to_json(cd: &Candlestick) -> serde_json::Value {
    json!({"o": cd.o, "h": cd.h, "l": cd.l, "c": cd.c})

}

fn f32_to_dec(v: f32) -> Decimal {
    Decimal::from_f32_retain(v).unwrap() // - from_f32 or from_f32_retain
}
pub async fn insert_candlestick_result(conn: &DatabaseConnection, result: CandlestickQueryData) -> Result<(), DbErr> {
    
    // let granularity = result.granularity.to_string();
    // let instrument = result.instrument;

    let mut inserts = Vec::new();
    
    result.candles.iter().for_each(|c| {

        let bid = c.bid.as_ref().unwrap();
        let ask = c.ask.as_ref().unwrap();
        let mid = c.mid.as_ref().unwrap();

        println!("volume: {:?}", c.volume);

        inserts.push(candlestick_data::ActiveModel {
            id: ActiveValue::NotSet,
            granularity: ActiveValue::Set(result.granularity.to_string()),
            instrument: ActiveValue::Set(result.instrument.clone()),
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

            // mid_o: ActiveValue::Set(mid.o),
            // mid_h: ActiveValue::Set(mid.h),
            // mid_l: ActiveValue::Set(mid.l),
            // mid_c: ActiveValue::Set(mid.c),
            // 
            // bid: ActiveValue::Set(cd_to_json(bid)),
            // ask: ActiveValue::Set(cd_to_json(ask)),
            // mid: ActiveValue::Set(cd_to_json(mid)),

            // ..Default::default()
        });
        
    });

    println!("volume: done");
    // When `id` column have conflicting value, do nothing
    let on_conflict = OnConflict::column(candlestick_data::Column::Id).do_nothing().to_owned();

    let res = candlestick_data::Entity::insert_many(inserts)
        .on_conflict(on_conflict.clone())
        // .build(Postgres);
        .exec(conn)
        .await?;

    println!("SQL: {:?}", res);

    Ok(())
}