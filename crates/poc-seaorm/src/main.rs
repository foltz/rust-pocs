mod commands;
mod domain;
mod schemas;

use std::env;
use std::sync::Arc;
use chrono::{TimeZone, Utc};
use chrono_tz::America::New_York;
use sea_orm::{Database, DbErr};
use oanda_sdk::apis::v3_instrument::InstrumentApi;
use oanda_sdk::clients::https_client::HttpsClient;
use oanda_sdk::models::candlestick_models::CandlestickGranularity;
use oanda_sdk::queries::candlestick_query::CandlestickQuery;
// use oanda_sdk::queries::QueryFormatter;
use crate::commands::create_database::create_database;
use crate::commands::oanda::candlestick_commands::insert_candlestick_result;
// use crate::schemas::oanda_schema::{create_oanda_tables};

// Change this according to your database implementation,
// or supply it as an environment variable.
// the whole database URL string follows the following format:
// "protocol://username:password@host:port/database"
// We put the database name (that last bit) in a separate variable simply for convenience.


async fn run() -> Result<(), DbErr> {

    let db_url: &str = "postgres://postgres:password@localhost:5432";
    let db_name: &str = "liq2";

    // ---------------------------
    // - CREATE DB:

    // let conn = Database::connect(db_url).await?;
    // println!("conn: {:?}", conn);
    //
    // create_database(&conn, db_name, true).await?;
    //
    // conn.close().await?;

    // ---------------------------
    // - CONNECT TO DB:

    let db_url_name = format!("{db_url}/{db_name}");
    let conn = Database::connect(db_url_name).await?;

    // - CREATE TABLE:
    // create_oanda_tables(&conn).await?;

    // - QUERY FROM API AND INSERT:

    // - Pull in the url for OANDA API and the API Key
    // - if the .env file isn't specified, assume that it's passed in via the docker env
    let _ = dotenvy::dotenv();

    let api_uri = env::var("OANDA_API_URI");
    let api_key = env::var("OANDA_API_KEY");
    if api_uri.is_err() || api_key.is_err() {
        panic!("must have env-vars");
    }
    let api_uri = api_uri.unwrap();
    let api_key = api_key.unwrap();


    let sun_bod = New_York.with_ymd_and_hms(
        2023, 08, 20,
        17, 03, 0
    ).unwrap().with_timezone(&Utc);

    let mon_eod = New_York.with_ymd_and_hms(
        2023, 08, 21,
        16, 58, 0
    ).unwrap().with_timezone(&Utc);


    let client = Arc::new(HttpsClient::new(api_uri, api_key));

    // let instrument = "EUR_USD".to_string();
    // let api = InstrumentApi::new(client, instrument);
    // 
    // let candles_query = CandlestickQuery::new(sun_bod)
    //     .with_to(mon_eod)
    //     .with_price("BAM".to_string())
    //     .with_granularity(CandlestickGranularity::H1)
    //     .format();
    // 
    // let result = api.candles_query(&candles_query).await;
    // 
    // if result.is_err() {
    //     println!("api-err: {:?}", result);
    // } else {
    //     println!("api-result: {:?}", result);
        insert_candlestick_result(&conn, result.unwrap()).await?;
    // }



    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        panic!("{}", err);
    }
}
