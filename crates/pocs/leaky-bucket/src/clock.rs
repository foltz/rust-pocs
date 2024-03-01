use std::sync::RwLock;
use std::time::Instant;

pub struct Clock {
    start: Instant,
    tock: Instant,
    cur_sec: i32,
    reqs: RwLock<i32>,
}

impl Clock {

    pub fn new() -> Self {

        let start = Instant::now();
        let tock = start.clone();
        let cur_sec = 0;
        let reqs = RwLock::new(0);
        Self {
            start,
            tock,
            cur_sec,
            reqs,
        }
    }

    pub fn req(&self) {
        let reqs = *self.reqs.read().unwrap();
        *self.reqs.write().unwrap() = reqs + 1;
    }
    pub fn tick(&mut self) {
        let passed = Instant::now().duration_since(self.tock);
        if passed.as_millis() >= 1000 {
            self.cur_sec += 1;
            let reqs = *self.reqs.read().unwrap();
            println!("secs: {} - reqs: {}", self.cur_sec, reqs);
            self.tock = Instant::now();
            *self.reqs.write().unwrap() = 0;
        }
    }
}