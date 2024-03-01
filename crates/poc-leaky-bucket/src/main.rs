mod test_client;
mod test_agent;
mod clock;

use leaky_bucket::RateLimiter;
use std::{thread, time};
use std::time::Duration;
use rand::prelude::*;

use crate::clock::Clock;
use crate::test_agent::TestAgent;
use crate::test_client::TestClient;

#[tokio::main]
// async fn main() {
//
//     init_logging();
//
//     let mut clock = Clock::new();
//     let client = TestClient::new(&clock);
//
//     let mut agents = Vec::new();
//     for i in 1..=10 {
//         let mut agent = TestAgent::new(&client, i);
//         tokio::spawn(async {
//             agent.run();
//         });
//         agents.push(agent);
//     }
//
//     loop {
//         clock.tick();
//     }
// }


async fn main() {

    init_logging();

    let interval = Duration::from_micros(8000);
    let limiter = RateLimiter::builder()
        // .max(120)
        .interval(interval)
        // .initial(60)
        .refill(1)
        .build()
        ;

    let start = time::Instant::now();
    let mut tick = start.clone();
    let mut cur_sec = 0;
    let mut reqs = 0;

    let mut rng = thread_rng();

    loop {

        {
            limiter.acquire_one().await;
            let sleep = rng.gen_range(2000..3000);
            thread::sleep(time::Duration::from_micros(sleep));
            reqs += 1;
        }
        // if reqs % 10 == 0 {
        //     println!("reqs: {reqs}")
        // }

        let passed = time::Instant::now().duration_since(tick);
        if passed.as_millis() >= 1000 {
            cur_sec+= 1;
            println!("tick: {cur_sec} - reqs: {reqs}");
            if cur_sec >= 10 { break; }

            tick = time::Instant::now();
            reqs = 0;
        }
        // println!(
        //     "I made it in {:?}!",
        //     time::Instant::now().duration_since(start)
        // );
    }


    // println!("Waiting for permit...");
    //
    // // Should take ~400 ms to acquire in total.
    // let a = limiter.acquire(7);
    // let b = limiter.acquire(3);
    // let c = limiter.acquire(10);
    //
    // let ((), (), ()) = tokio::join!(a, b, c);
    //
    // println!(
    //     "I made it in {:?}!",
    //     time::Instant::now().duration_since(start)
    // );
}

/// Initialize logging for a given example.
pub fn init_logging() {

    use tracing_subscriber::prelude::*;
    use tracing_subscriber::EnvFilter;

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_level(true)
                .compact(),
        )
        .init();
}