use std::error::Error;
use std::sync::Arc;
use std::time;
use chrono::Utc;
use crate::models::TestModel;
use crate::queue::QueueWrapper;

pub async fn write_data(
    mut queue: Arc<QueueWrapper>,
    count: i32,
) {

    let duration = time::Duration::from_nanos(1);
    for n in 1..=count {

        let m = TestModel {
            value: format!("{n}"),
            created: Utc::now(),
            received: None,
        };

        let msg = serde_json::to_string(&m).unwrap();

        let latency = (Utc::now() - m.created).num_milliseconds();
        println!("write-start: {} - latency: {}", m.value, latency);
        queue.write(msg.as_bytes()); //.expect("add failed");

        let latency = (Utc::now() - m.created).num_milliseconds();
        println!("write-complete: {} - latency: {}", m.value, latency);

        tokio::time::sleep(duration).await;
    }
}

pub async fn read_data(
    queue: Arc<QueueWrapper>,
) {

    let duration = time::Duration::from_nanos(1);
    loop {
        match queue.read() {
            Err(e) => {
                println!("Read Error: {:?}", e);
            }
            Ok(d) => {
                if d.is_none() {
                    // break;
                }
                else {

                    let d = d.unwrap();
                    let msg = std::str::from_utf8(&d).unwrap_or("<invalid>");

                    match serde_json::from_str::<TestModel>(&msg) {

                        Err(e) => println!("Deserialize Error: {:?}", e),

                        Ok(m) => {
                            let latency = (Utc::now() - m.created).num_microseconds().unwrap_or_default();
                            println!("read: {} - latency: {}", m.value, latency);
                        }
                    };

                    queue.remove();
                }
            }
        }

        tokio::time::sleep(duration).await;
    }
}