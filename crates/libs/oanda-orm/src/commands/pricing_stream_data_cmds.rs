use chrono::Utc;
use rust_decimal::Decimal;

use sea_orm::{ActiveValue, DbConn, DbErr, EntityTrait, InsertResult};
use sea_orm::prelude::DateTimeUtc;
use sea_orm::sea_query::OnConflict;
use oanda_sdk::models::pricing_models::ClientPrice;

use crate::domain::pricing_stream_data;


pub async fn insert_pricing_stream_data(
    db_conn: &DbConn,
    data: &ClientPrice,

) -> Result<DateTimeUtc, DbErr> { //Box<dyn std::error::Error>> {

    let instrument = data.instrument.clone();
    let time = data.time;
    
    fn zero_prices(closeout_price: Decimal) -> (Decimal, Decimal, Decimal) {(
        Decimal::new(0, 2),
        Decimal::new(0, 2),
        closeout_price,
    )}
    
    let (bid_price, bid_liquidity, closeout_bid) = match data.bids.len() {
        0 => zero_prices(data.closeout_bid),
        _ => {
            let bid = &data.bids[0];
            (bid.price, bid.liquidity.unwrap_or_default(), data.closeout_bid) 
        }
    };
    
    let (ask_price, ask_liquidity, closeout_ask) = match data.asks.len() {
        0 => zero_prices(data.closeout_ask),
        _ => {
            let ask = &data.asks[0];
            (ask.price, ask.liquidity.unwrap_or_default(), data.closeout_ask)
        }
    };
    

    let model = pricing_stream_data::ActiveModel {

        // id: ActiveValue::NotSet,

        timestamp: ActiveValue::Set(Utc::now()),

        instrument: ActiveValue::Set(instrument),
        time: ActiveValue::Set(time),

        bid_price: ActiveValue::Set(bid_price),
        bid_liquidity: ActiveValue::Set(bid_liquidity),
        // closeout_bid: ActiveValue::Set(closeout_bid),

        ask_price: ActiveValue::Set(ask_price),
        ask_liquidity: ActiveValue::Set(ask_liquidity),
        // closeout_ask: ActiveValue::Set(closeout_ask),
    };

    let on_idx_conflict = OnConflict::columns([
        pricing_stream_data::Column::Instrument,
        pricing_stream_data::Column::Time,
    ]).update_columns([
        pricing_stream_data::Column::Timestamp,
    ]).to_owned();

    let res = pricing_stream_data::Entity::insert(model)
        .on_conflict(on_idx_conflict.clone())
        // .build(Postgres);
        .exec(db_conn)
        .await; //?;

    let res = match res {
        Ok(ins_res) => { ins_res }
        Err(err) => {
            println!("insert_error: {:?}", err);
            panic!("{:?}", err);
        }
    };

    // println!("insert_pricing_stream_data::SQL: {:?}", res);

    // - was:
    // Ok(res.last_insert_id)

    // - with hyper_table/composite key:
    let (_, time ) = res.last_insert_id;
    Ok(time)
}
