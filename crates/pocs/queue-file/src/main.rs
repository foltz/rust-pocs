mod models;
mod tasks;
mod queue;

use std::sync::Arc;
use queue_file::QueueFile;
use crate::queue::QueueWrapper;
use crate::tasks::{read_data, write_data};

// #[tokio::main(flavor = "current_thread")]
#[tokio::main(flavor = "current_thread")]
async fn main() {

    queue_tasks().await;
    // queue_wrapper();
    // two_queues();
    // basic_example();
}

async fn queue_tasks() {

    let queue = Arc::new(QueueWrapper::new("example.qf"));

    queue.clear();

    let write_task = tokio::spawn(write_data(queue.clone(), 10));
    let read_task = tokio::spawn(read_data(queue.clone()));

    let _ = tokio::join!(
        write_task,
        read_task,
    );
}

fn queue_wrapper() {

    let qw = QueueWrapper::new("example.qf");

    qw.write("ele 1".as_bytes());
    qw.write("ele 2".as_bytes());
    qw.write("ele 3".as_bytes());

    qw.remove();

    match qw.get_reader() {
        Err(e) => println!("get-reader err: {:?}", e),
        Ok(mut reader) => {

            for (index, elem) in reader.iter().enumerate() {
                println!(
                    "{}: {} bytes -> {}",
                    index,
                    elem.len(),
                    std::str::from_utf8(&elem).unwrap_or("<invalid>")
                );
            }
        }
    }

}

fn two_queues() {

    let mut qf_write = QueueFile::open("example.qf")
        .expect("cannot open queue file");

    let mut qf_read = QueueFile::open("example.qf")
        .expect("cannot open queue file");

    qf_write.add("ELEMENT #1".as_bytes()).expect("add failed");
    qf_write.add("ELEMENT #2".as_bytes()).expect("add failed");
    qf_write.add("ELEMENT #3".as_bytes()).expect("add failed");

    qf_write.remove().expect("remove failed");


    for (index, elem) in qf_read.iter().enumerate() {
        println!(
            "{}: {} bytes -> {}",
            index,
            elem.len(),
            std::str::from_utf8(&elem).unwrap_or("<invalid>")
        );
    }

    // qf.clear().expect("clear failed");
}


fn basic_example() {
    let mut qf = QueueFile::open("example.qf")
        .expect("cannot open queue file");

    qf.add("ELEMENT #1".as_bytes()).expect("add failed");
    qf.add("ELEMENT #2".as_bytes()).expect("add failed");
    qf.add("ELEMENT #3".as_bytes()).expect("add failed");

    qf.remove().expect("remove failed");

    for (index, elem) in qf.iter().enumerate() {
        println!(
            "{}: {} bytes -> {}",
            index,
            elem.len(),
            std::str::from_utf8(&elem).unwrap_or("<invalid>")
        );
    }

    // qf.clear().expect("clear failed");
}