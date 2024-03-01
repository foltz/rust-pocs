use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use hyper::{Body, Client, Response, };
use hyper::client::{HttpConnector, ResponseFuture};
use hyper_tls::HttpsConnector;
use ratelimit::Ratelimiter;

pub type ArcClient = Arc<HttpsClient>;
pub struct HttpsClient {

    pub account_id: String,

    api_key: String,
    root_uri: String,
    // headers: HeaderMap,
    https_client: Client<HttpsConnector<HttpConnector>>,
    rate_limiter: Ratelimiter,
}

#[derive(Debug)]
pub struct ApiResponse {
    pub status_code: u16,
    pub response_body: String,
}

impl HttpsClient {
    pub fn new(account_id: String, api_key: String, root_uri: String) -> Self {


        let https_client = Client::builder()
            .build::<_, hyper::Body>(HttpsConnector::new());

        /*
        NOTE on RateLimiting:
        Client is allowed to have no more than 120 requests per second on average,
        with bursts of no more than 60 requests. Excess requests will be
        rejected. This restriction is applied for each access token,
        not for each individual connection.
         */

        // - 9ms interval allows for 111 per second 8.3 would be 120:
        let interval = Duration::from_micros(8500); // - could be 8300 but round up to ensure we don't under count
        let rate_limiter = Ratelimiter::builder(1, interval)
            .initial_available(60)
            .max_tokens(60)
            .build()
            .unwrap();

        // let mut headers = HeaderMap::new();
        //
        // headers.insert("Authorization", format!("Bearer {}", api_key).parse().unwrap());
        // headers.insert("AcceptDatetimeFormat", "RFC3339".parse().unwrap());
        // headers.insert("Connection", "Keep-Alive".parse().unwrap());

        let client = HttpsClient {
            account_id,
            api_key,
            root_uri,
            // headers,
            https_client,
            rate_limiter,
        };

        // - TODO: update this code...
        // thread::spawn(move || {
        //     loop {
        //         rate_limiter.try_wait()
        //     }
        // });

        client
    }

    pub async fn api_get(&self, endpoint: &str) -> hyper::Result<ApiResponse> { //}, Box<dyn Error>> { //} -> Response<Body> {

        let endpoint_url = format!("{}/{}", self.root_uri, endpoint);
        // println!("full_url: {endpoint_url}");

        let req = hyper::Request::builder()
            .method(hyper::Method::GET)
            .uri(endpoint_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("AcceptDatetimeFormat", "RFC3339")
            .header("Connection", "Keep-Alive")
            // .body(hyper::Body::from(""))
            .body(hyper::Body::empty())
            .unwrap();

        let result = self.https_client.request(req).await?;

        let status_code = result.status().as_u16();

        let body = result.into_body();
        let body_bytes = hyper::body::to_bytes(body).await?;
        let response_body = String::from_utf8(body_bytes.to_vec()).unwrap();

        Ok(ApiResponse {
            status_code,
            response_body,
        })
    }

    pub async fn api_stream(&self, endpoint: &str) -> hyper::Result<Response<Body>> { //} -> Response<Body> {

        let endpoint_url = format!("{}/{}", self.root_uri, endpoint);
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

        self.https_client.request(req).await
    }


    // fn headers(&self) -> hyper {
    //     let mut headers = HeaderMap::new();
    //
    //     headers.insert("Authorization", format!("Bearer {}", self.api_key).into());
    //     headers.insert("AcceptDatetimeFormat", "RFC3339".into());
    //     headers.insert("Connection", "Keep-Alive".into());
    //
    //     // headers.set(Authorization(format!("Bearer {}", self.api_key)));
    //     // headers.set(AcceptDatetimeFormat("RFC3339".to_string()));
    //     // headers.set(Connection("Keep-Alive".to_string()));
    //
    //     headers
    // }
}