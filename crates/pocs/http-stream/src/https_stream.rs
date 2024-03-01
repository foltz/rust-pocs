use hyper::body::HttpBody;
use hyper::Client;
use hyper_tls::HttpsConnector;
use serde::Deserialize;
use app_support::helpers::SendableResult;
use oanda_sdk::models::pricing_models::PricingStreamEvent;

// pub struct PricingStreamMessage {
//     // - ClientPrice or Heartbeat
//     #[serde(rename(deserialize = "type"))]
//     pub msg_type: String,
// }

// #[derive(Debug, Deserialize)]
// #[serde(tag = "type")]
// enum PricingStreamMessage {
//     PRICE(PricingStreamPrice),
//     HEARTBEAT(PricingStreamHeartbeat),
// }
// #[derive(Debug, Deserialize)]
// pub struct PricingStreamPrice {
//     // - ClientPrice or Heartbeat
//     pub time: String,
//     pub instrument: String,
// }
// #[derive(Debug, Deserialize)]
// pub struct PricingStreamHeartbeat {
//     pub time: String,
// }



pub async fn https_stream(api_key: &str, url: &str) -> SendableResult<()> {

    let https_client = Client::builder()
        .build::<_, hyper::Body>(HttpsConnector::new());

    let req = hyper::Request::builder()
        .method(hyper::Method::GET)
        .uri(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("AcceptDatetimeFormat", "RFC3339")
        .header("Connection", "Keep-Alive")
        // .body(hyper::Body::from(""))
        .body(hyper::Body::empty())
        .unwrap();


    let mut result = https_client.request(req).await?;
    while let Some(next) = result.data().await {
        let body_bytes = next?;
        let response_body = String::from_utf8(body_bytes.to_vec()).unwrap();
        println!("chunk: {:?}", body_bytes);

        match serde_json::from_str::<PricingStreamEvent>(&response_body) {
            Err(e) => {
                panic!("{:?}", e);
            },

            // - NOTE: return order_book property as data:

            Ok(data) => {
                println!("pricing-data: {:?}", data);
            }
        };
    }

    // let status_code = result.status().as_u16();
    // let response_body = result.into_body();
    //
    // println!("status_code: {}", status_code);
    // println!("response_body: {:?}", response_body);



    Ok(())
}