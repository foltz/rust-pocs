use chrono::Utc;
use crate::clients::https_client::ArcClient;

use crate::models::candlestick_models::CandlestickQueryResult;
use crate::models::order_book_models::{OrderBookQueryResult, OrderBookResponse};
use crate::models::position_book_models::{PositionBookQueryResult, PositionBookResponse};
use crate::queries::candlestick_query::CandlestickQuery;
use crate::queries::shared_book_query::SharedBookQuery;
use crate::support::errors::QueryError;

static ENDPOINT_URI: &'static str = "v3/instruments";

// - TODO: change Err type from String to Box<dyn Err>?
pub type CandlestickChannelPayload = Result<CandlestickQueryResult, String>;

pub struct InstrumentApi<'a> {
    api_client: &'a ArcClient,
}

impl<'a> InstrumentApi<'a> {
    pub fn new(api_client: &'a ArcClient) -> Self {
        Self {
            api_client,
            // instrument,
        }
    }


    pub async fn candles_query(&self, query: &CandlestickQuery) -> CandlestickQueryResult {

        let endpoint = format!("{}/{}/candles{}", ENDPOINT_URI,
            query.instrument,
            query.to_querystring()
        );

        println!("url: {}", endpoint);

        let result = self.api_client.api_get(&endpoint).await;
        if let Err(e) = result {
            return Err(
                QueryError::ApiRequest(
                    e.to_string(),
                    Some("candles_query.api_get()".to_string()))
            );
        };

        let api_response = result.unwrap();
        if api_response.status_code != 200 {
            return Err(
                QueryError::ApiResponse(
                    api_response.response_body,
                    Some(format!("candles_query.status_code:{}", api_response.status_code))
                )
            )
        }

        return match serde_json::from_str(&api_response.response_body) {
            Err(e) => Err(
                QueryError::ParseJson(
                    e.to_string(),
                    Some("candles_query.api_response.body".to_string())
                )
            ),

            Ok(data) => Ok(data)
        };
    }


    pub async fn order_book_query(&self, query: &SharedBookQuery) -> OrderBookQueryResult {

        let endpoint = format!("{}/{}/orderBook{}", ENDPOINT_URI,
                               query.instrument,
                               query.to_querystring()
        );

        // println!("url: {}", endpoint);

        let result = self.api_client.api_get(&endpoint).await;
        if let Err(e) = result {
            return Err(
                QueryError::ApiRequest(
                    e.to_string(),
                    Some("order_book_query.api_get()".to_string()))
            );
        };

        let api_response = result.unwrap();
        if api_response.status_code != 200 {
            return Err(
                QueryError::ApiResponse(
                    api_response.response_body,
                    Some(format!("order_book_query.status_code:{}", api_response.status_code))
                )
            )
        }

        // - NOTE: OrderBookResponse is NOT the OrderBookData we want to return:

        return match serde_json::from_str::<OrderBookResponse>(&api_response.response_body) {
            Err(e) => Err(
                QueryError::ParseJson(
                    e.to_string(),
                    Some("order_book_query.api_response.body".to_string())
                )
            ),

            // - NOTE: return order_book property as data:

            Ok(data) => Ok(data.order_book)
        };
    }

    pub async fn position_book_query(&self, query: &SharedBookQuery) -> PositionBookQueryResult {

        let endpoint = format!("{}/{}/positionBook{}", ENDPOINT_URI,
                               query.instrument,
                               query.to_querystring()
        );

        let result = self.api_client.api_get(&endpoint).await;
        if let Err(e) = result {
            return Err(
                QueryError::ApiRequest(
                    e.to_string(),
                    Some("position_book_query.api_get()".to_string()))
            );
        };

        let api_response = result.unwrap();
        if api_response.status_code != 200 {
            return Err(
                QueryError::ApiResponse(
                    api_response.response_body,
                    Some(format!("position_book_query.status_code:{}", api_response.status_code))
                )
            )
        }

        // - NOTE: PositionBookResponse is NOT the PositionBookData we want to return:

        return match serde_json::from_str::<PositionBookResponse>(&api_response.response_body) {
            Err(e) => {
                println!("now: {}", Utc::now().to_rfc3339());
                println!("body: {}", &api_response.response_body);
                return Err(
                    QueryError::ParseJson(
                        e.to_string(),
                        Some("position_book_query.api_response.body".to_string())
                    )
                )
            },

            // - NOTE: return position_book property as data:

            Ok(data) => Ok(data.position_book)
        };
    }
}