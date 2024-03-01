
use crate::clients::https_client::ArcClient;
use crate::models::pricing_models::PricingQueryResult;
use crate::queries::pricing_query::PricingQuery;
use crate::support::errors::QueryError;

static ENDPOINT_URI: &'static str = "v3/accounts";

// - TODO: change Err type from String to Box<dyn Err>?
pub type PricingChannelPayload = Result<PricingQueryResult, String>;

pub struct PricingApi<'a> {
    api_client: &'a ArcClient,
}

impl<'a> PricingApi<'a> {
    pub fn new(api_client: &'a ArcClient) -> Self {
        Self {
            api_client,
        }
    }


    pub async fn pricing_query(&self, query: &PricingQuery) -> PricingQueryResult {

        let endpoint = format!("{}/{}/pricing{}", ENDPOINT_URI,
                               query.account_id,
                               query.to_querystring()
        );

        println!("url: {}", endpoint);

        let result = self.api_client.api_get(&endpoint).await;
        if let Err(e) = result {
            return Err(
                QueryError::ApiRequest(
                    e.to_string(),
                    Some("pricing_query.api_get()".to_string()))
            );
        };

        let api_response = result.unwrap();
        if api_response.status_code != 200 {
            return Err(
                QueryError::ApiResponse(
                    api_response.response_body,
                    Some(format!("pricing_query.status_code:{}", api_response.status_code))
                )
            )
        }

        return match serde_json::from_str(&api_response.response_body) {
            Err(e) => Err(
                QueryError::ParseJson(
                    e.to_string(),
                    Some("pricing_query.api_response.body".to_string())
                )
            ),

            Ok(data) => Ok(data)
        };
    }
}