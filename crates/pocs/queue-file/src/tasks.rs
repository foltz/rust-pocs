use std::error::Error;
use std::sync::Arc;
use crate::queue::QueueWrapper;

pub async fn write_data(
    mut queue: Arc<QueueWrapper>,
    count: i32,
) {

    for n in 1..=count {
        println!("write: {n}");
        queue.write(format!("ELEMENT #{}", n).as_bytes()); //.expect("add failed");
    }
}

pub async fn read_data(
    queue: Arc<QueueWrapper>,
) {

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
                    println!("read: {msg}");
                    queue.remove();
                }
            }
        }
    }
}