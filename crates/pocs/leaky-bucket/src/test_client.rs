use std::cell::RefCell;
use std::sync::RwLock;

use std::time::Duration;
use leaky_bucket::RateLimiter;
use rand::prelude::*;
use tokio::time;
use crate::clock::Clock;

pub struct TestClient<'a> {
    clock: &'a Clock,
    limiter: RateLimiter,
    rng: RwLock<ThreadRng>,
}

impl<'a> TestClient<'a> {
    pub fn new(clock: &'a Clock) -> Self {

        let interval = Duration::from_micros(8000);
        let limiter = RateLimiter::builder()
            // .max(120)
            .interval(interval)
            // .initial(60)
            .refill(1)
            .build()
            ;

        let rng = RwLock::new(thread_rng());
        Self {
            clock,
            limiter,
            rng,
        }
    }

    pub async fn get(&self) -> u64 {

        self.limiter.acquire_one().await;
        self.clock.req();

        let micros = self.rng.write().unwrap().gen_range(2000..3000);
        time::sleep(Duration::from_micros(micros)).await;
        micros
    }
}