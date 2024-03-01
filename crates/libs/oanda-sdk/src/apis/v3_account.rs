use crate::clients::https_client::ArcClient;
use crate::models::account_models::{AccountProperties, AccountQueryAndResponse};
use crate::support::errors::QueryError;

static ENDPOINT_URI: &'static str = "v3/accounts";

pub type AccountQueryResult = Result<AccountQueryAndResponse, QueryError>;
pub struct AccountApi<'a> {
    api_client: &'a ArcClient,
}

impl<'a> AccountApi<'a> {
    pub fn new(api_client: &'a ArcClient) -> Self {
        Self {
            api_client,
        }
    }

    pub async fn list_accounts(&self) -> AccountQueryResult {

        todo!(); // - TODO: refactor to return QueryErrors...
        // let api_response = self.api_client.api_get(&ENDPOINT_URI).await?;
        //
        // let status_code = api_response.status_code;
        //
        // let mut data: Option<Vec<AccountProperties>> = None;
        // let mut err: Option<String> = None;
        //
        // if status_code == 200 {
        //     data = Some(serde_json::from_str(&api_response.body).unwrap());
        // } else {
        //     err = Some(api_response.body);
        // }
        //
        // Ok(AccountQueryAndResponse {
        //     status_code,
        //     data,
        //     err,
        // })

        // ORIGINAL CODE:

        // let result: AccountResponse = serde_json::from_str(&api_response.unwrap()).unwrap();
        // // print!("Accounts: {:?}", result.accounts);
        // Ok(result.accounts)
    }
}