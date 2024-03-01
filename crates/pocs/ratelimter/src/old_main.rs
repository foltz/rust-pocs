mod safe_client;

use std::time;
use std::time::Duration;
use rand::prelude::*;
use ratelimit::Ratelimiter;

#[tokio::main]
async fn old_main() {
    init_logging();

    let interval = Duration::from_micros(8500);
    let rate_limiter = Ratelimiter::builder(1, interval)
        // .initial_available(60)
        // .max_tokens(60)
        .build()
        .unwrap();

    let start = time::Instant::now();
    let mut tick = start.clone();
    let mut cur_sec = 0;
    let mut reqs = 0;

    let mut rng = thread_rng();

    loop {
        {

            if let Err(duration) = rate_limiter.try_wait() {
                tokio::time::sleep(duration).await;
            } else {
                let rnd_delay = rng.gen_range(2000..3000);
                tokio::time::sleep(time::Duration::from_micros(rnd_delay)).await;
                reqs += 1;
            }

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