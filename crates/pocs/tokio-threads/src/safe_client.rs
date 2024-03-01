use std::sync::Arc;
use std::time::Duration;
use hyper::Client;
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use leaky_bucket::RateLimiter;

pub type ArcClient = Arc<SafeClient>;
pub struct SafeClient {
    api_uri: String,
    api_key: String,
    // headers: HeaderMap,
    https_client: Client<HttpsConnector<HttpConnector>>,
    limiter: RateLimiter,
}

impl SafeClient {
    pub fn new(api_uri: String, api_key: String) -> Self {


        let https_client = Client::builder()
            .build::<_, hyper::Body>(HttpsConnector::new());

        /// NOTE on RateLimiting:
        /// Client is allowed to have no more than 120 requests per second on average,
        /// with bursts of no more than 60 requests. Excess requests will be
        /// rejected. This restriction is applied for each access token,
        /// not for each individual connection.

        let interval = Duration::from_micros(7500);
        let limiter = RateLimiter::builder()
            // .max(120)
            .interval(interval)
            // .initial(60)
            .refill(1)
            .build()
            ;

        Self {
            api_uri,
            api_key,
            https_client,
            limiter,
        }
    }

    pub async fn get(&self) {
        self.limiter.acquire_one().await;
    }

    pub async fn api_get(&self, endpoint: &str) -> Result<String, ()> { //} -> Response<Body> {

        self.limiter.acquire_one().await;

        let endpoint_url = format!("{}/{}", self.api_uri, endpoint);
        println!("full_url: {endpoint_url}");
        // - TODO: add params...

        let req = hyper::Request::builder()
            .method(hyper::Method::GET)
            .uri(endpoint_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("AcceptDatetimeFormat", "RFC3339")
            .header("Connection", "Keep-Alive")
            .body(hyper::Body::from(""))
            .unwrap();

        // - TODO: handle ok/error....

        // - TODO: ratelimit:
        let resp = self.https_client.request(req).await;

        // - TODO: handle ok/error....

        let body_bytes = hyper::body::to_bytes(resp.unwrap().into_body()).await;
        let str = String::from_utf8(body_bytes.unwrap().to_vec()).unwrap();

        // - TODO: handle ok/error....

        Ok(str)
    }
}