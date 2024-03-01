use std::env;

#[derive(Debug)]
pub struct OandaEnv {
    pub account_id: String,
    pub api_key: String,
    pub api_uri: String,
    pub stream_uri: String,
}

impl OandaEnv {
    pub fn load() -> OandaEnv {

        let _ = dotenvy::dotenv();

        let account_id = env::var("OANDA_ACCOUNT_ID");
        let api_key = env::var("OANDA_API_KEY");
        let api_uri = env::var("OANDA_API_URI");
        let stream_uri = env::var("OANDA_STREAM_URI");

        if account_id.is_err() {
            panic!("missing env-value: OANDA_ACCOUNT_ID");
        }
        if api_key.is_err() {
            panic!("missing env-value: OANDA_API_KEY");
        }
        if api_uri.is_err() {
            panic!("missing env-value: OANDA_API_URI");
        }
        if stream_uri.is_err() {
            panic!("missing env-value: OANDA_STREAM_URI");
        }


        OandaEnv {
            account_id:  account_id.unwrap(),
            api_key:  api_key.unwrap(),
            api_uri: api_uri.unwrap(),
            stream_uri: stream_uri.unwrap(),
        }
    }
}