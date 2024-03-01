use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::time;
use std::time::Instant;

pub type ArcMutexClock = Arc<Mutex<MetricsClock>>;

pub async fn metrics_clock_task(clock: ArcMutexClock) {

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

pub struct MetricsClock {
    start: Instant,
    tock: Instant,
    cur_sec: i32,
    reqs: RefCell<i32>,
}

impl MetricsClock {
    pub fn new() -> Self {
        let start = Instant::now();
        let tock = start.clone();
        let cur_sec = 0;
        let reqs = RefCell::new(0);
        Self {
            start,
            tock,
            cur_sec,
            reqs,
        }
    }

    pub fn log_request(&self) {
        *self.reqs.borrow_mut() += 1;
    }

    pub fn count_requests(&self) -> i32 {
        *self.reqs.borrow()
    }

    pub fn tick(&mut self) {
        let passed = Instant::now().duration_since(self.tock);
        if passed.as_millis() >= 1000 {
            self.cur_sec += 1;
            let reqs = self.count_requests();
            println!("secs: {} - reqs: {}", self.cur_sec, reqs);
            self.tock = Instant::now();
            *self.reqs.borrow_mut() = 0;
        }
    }
}