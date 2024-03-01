use app_support::helpers::SendableResult;
use crate::https_stream::https_stream;

mod https_stream;

#[tokio::main]
async fn main() -> SendableResult<()> {

    let api_key = "cc193017cf350ea435f0c53c365e3f77-c613fc9733dea5d985fd3724b565fa2d";
    let endpoint_url = "https://stream-fxtrade.oanda.com/v3/accounts/001-001-10140985-001/pricing/stream?instruments=EUR_USD";

    // HTTPS requires picking a TLS implementation, so give a better
    // warning if the user tries to request an 'https' URL.
    // let url = endpoint_url.parse::<hyper::Uri>().unwrap();
    // if url.scheme_str() != Some("http") {
    //     println!("This example only works with 'http' URLs.");
    //     return Ok(());
    // }

    https_stream(api_key, endpoint_url).await
}
