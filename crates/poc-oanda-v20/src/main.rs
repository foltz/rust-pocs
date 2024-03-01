
mod test_dates;
mod oanda_client;

use oanda_sdk::apis::v3_account::AccountApi;
use oanda_sdk::models::candlestick_models::CandlestickGranularity;
use oanda_sdk::queries::candlestick_query::CandlestickQuery;
use crate::oanda_client::get_oanda_client;
use crate::test_dates::{get_mon_eod, get_sun_bod};


#[tokio::main]
async fn main() {

    let client = &get_oanda_client();

    match AccountApi::new(client).list_accounts().await {
        Ok(accounts) => println!("OK: {:?}", accounts),
        _ => println!("Error")
    }


    let instrument = &String::from("EUR_USD");
    let granularity = CandlestickGranularity::M1;

    // match InstrumentApi::new(client, instrument).candles_from(get_utc_now(), granularity).await {
    //     Ok(pricing) => println!("OK: {:?}", pricing),
    //     _ => println!("Error")
    // }


    let query = CandlestickQuery {
        granularity: granularity.clone(),
        from: Some(get_sun_bod()),
        to: Some(get_mon_eod()),
        ..Default::default()
    };

    // match InstrumentApi::new(client, instrument).candles_query(&query).await {
    //     Ok(pricing) => println!("OK: {:?}", pricing),
    //     _ => println!("Error")
    // }
    //
    // match InstrumentApi::new(client, instrument).order_book(get_valid_book_time()).await {
    //     Ok(pricing) => println!("OK: {:?}", pricing),
    //     _ => println!("Error")
    // }
    //
    // match InstrumentApi::new(client, instrument).position_book(get_valid_book_time()).await {
    //     Ok(pricing) => println!("OK: {:?}", pricing),
    //     _ => println!("Error")
    // }
}
