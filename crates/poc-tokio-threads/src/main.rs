
mod safe_clock;
mod safe_client;
mod safe_endpoint;

use std::sync::{Arc, Mutex};
use rand::prelude::*;
use tokio::time;
use crate::safe_client::{ArcClient, SafeClient};
use crate::safe_clock::{ArcMutexClock, SafeClock};
use crate::safe_endpoint::SafeEndpoint;

#[tokio::main]
async fn main() {

    let clock: ArcMutexClock = {
        Arc::new(Mutex::new(SafeClock::new()))
    };

    let api_key = "".to_string();
    let api_uri = "".to_string();
    let client = Arc::new(SafeClient::new(api_uri, api_key));

    tokio::spawn(run_clock(clock.clone()));
    tokio::spawn(foo(client.clone(), clock.clone()));
    tokio::spawn(bar(client.clone(), clock.clone()));
    // tokio::spawn(bar(clock.clone() ));

    // let duration = time::Duration::from_millis(10);
    loop {
        // tokio::time::sleep(duration).await;
        // println!("-");
    }
}

async fn run_clock(clock: ArcMutexClock) {

    let duration = time::Duration::from_millis(1);
    loop {
        {
            tokio::time::sleep(duration).await;
        }
        if let Ok(mut clock) = clock.try_lock() {
            clock.tick();
        }
    }
}

async fn foo(client: ArcClient, clock: ArcMutexClock) {

    let mut rng: StdRng = SeedableRng::from_entropy();

    let endpoint = SafeEndpoint::new(client.clone());

    loop {

        let res = endpoint.get_some().await;
        // client.get().await;

        let sleep = rng.gen_range(1000..3000);
        // println!("bar: sleep: {sleep}");
        let duration = time::Duration::from_micros(sleep);
        tokio::time::sleep(duration).await;

        if let Ok(clock) = clock.try_lock(){
            clock.req();
        }
    }
}

async fn bar(client: ArcClient, clock: ArcMutexClock) {

    let mut rng: StdRng = SeedableRng::from_entropy();

    loop {

        client.get().await;

        let sleep = rng.gen_range(1000..3000);
        // println!("bar: sleep: {sleep}");
        let duration = time::Duration::from_micros(sleep);
        tokio::time::sleep(duration).await;

        if let Ok(clock) = clock.try_lock(){
            clock.req();
        }
    }
}