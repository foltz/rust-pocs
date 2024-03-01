use std::env;
use std::sync::Arc;
use oanda_sdk::clients::https_client::HttpsClient;

pub fn get_oanda_client() -> Arc<HttpsClient> {

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

    println!("uri: {}", &api_uri);
    println!("key: {}", &api_key);


    Arc::new(HttpsClient::new(api_uri, api_key))
}