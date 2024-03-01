
use chrono::{DateTime, Utc};
use tokio::sync::broadcast;
use crate::apis::v3_instrument::{CandlestickChannelPayload, InstrumentApi};
use crate::clients::https_client::ArcClient;
use crate::models::candlestick_models::CandlestickGranularity;
use crate::queries::candlestick_query::CandlestickQuery;
use crate::support::metrics::metrics_clock::ArcMutexClock;


pub type CandlestickChannelRelay = broadcast::Sender<CandlestickChannelPayload>;
pub type CandlestickChannelSubscription = broadcast::Receiver<CandlestickChannelPayload>;

pub struct CandlestickFeed {
    client: ArcClient,
    clock: ArcMutexClock, // - TODO: replace with messaging system
    relay: CandlestickChannelRelay,

    instrument: String,
    granularity: CandlestickGranularity,

    from: Option<DateTime<Utc>>,
    to: Option<DateTime<Utc>>,
}

impl CandlestickFeed {
    pub fn new(

        client: ArcClient,
        clock: ArcMutexClock,

        instrument: String,
        granularity: CandlestickGranularity,

        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,

    ) -> Self {

        let (relay, _) =
            broadcast::channel::<CandlestickChannelPayload>(8);

        Self {

            client,
            clock,
            relay,

            instrument,
            granularity,

            from,
            to,
        }
    }

    pub fn subscribe(&self) -> CandlestickChannelSubscription {
        self.relay.subscribe()
    }

    pub fn run(&self) {


        tokio::spawn(
            candlestick_feed_task(

                self.client.clone(),
                self.clock.clone(),
                self.relay.clone(),

                self.instrument.clone(),
                self.granularity.clone(),

                self.from.clone(),
                self.to.clone(),

                1000,
            )
        );

    }
}

async fn candlestick_feed_task(

    client: ArcClient,
    clock: ArcMutexClock, // - TODO: replace this with the messaging system....
    relay: CandlestickChannelRelay,

    instrument: String,
    granularity: CandlestickGranularity,

    from: Option<DateTime<Utc>>,
    to: Option<DateTime<Utc>>,

    freq_ms: u64,

) {

    todo!();

    let api = InstrumentApi::new(&client);

    // let mut candles_query = CandlestickQuery::new(granularity, from)
    //     .with_to(to);
    let mut candles_query = CandlestickQuery {
        granularity, from, to,
        ..Default::default()
    };

    loop {

        // - TODO: refactor!

        // let start = Instant::now();
        //
        // // - TODO: setup ability to update the from value:
        // // candles_query.from = from;
        //
        // let relay_result = relay.send(
        //     api.candles_query(&candles_query).await
        // );
        //
        // // - TODO: come up with a better way to handle this...
        // if relay_result.is_err() {
        //     panic!("{:?}", relay_result.unwrap());
        // }
        //
        // // - metrics clock request-logging:
        // loop {
        //     if let Ok(clock) = clock.try_lock(){
        //         clock.log_request();
        //         break;
        //     }
        // }
        //
        // let passed = Instant::now().duration_since(start).as_millis() as u64;
        // if freq_ms > passed {
        //     let wait = time::Duration::from_millis(freq_ms - passed);
        //     tokio::time::sleep(wait).await;
        //
        //     // - TODO: set the start time HERE to now...
        // } else {
        //     // - TODO: we need to retroactively set the start time to now - passed
        // }

    }
}

pub async fn candlestick_feed_subscription_example(mut subscription: CandlestickChannelSubscription) {

    loop {
        let subscription_result = subscription.recv().await;
        match subscription_result {
            Ok(query_result) => {
                match query_result {
                    Ok(pricing) => println!("QueryResult: {:?}", pricing),
                    Err(err) => println!("QueryResult Error: {:?}", err),
                }
            },
            Err(err) => println!("SubscriptionResult Error: {:?}", err),
        }
    }
}