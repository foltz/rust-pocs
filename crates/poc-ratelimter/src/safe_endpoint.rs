use crate::safe_client::{ArcClient};

static ENDPOINT_URI: &'static str = "v3/accounts";
pub struct SafeEndpoint {
    client: ArcClient,
}

impl SafeEndpoint {

    pub fn new(client: ArcClient) -> Self {
        Self {
            client,
        }
    }

    pub async fn get_some(&self) -> Result<i32, ()> {
        let res = self.client.api_get("").await;
        Ok(2)
    }
}