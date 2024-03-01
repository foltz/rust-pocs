use std::time::Duration;
use tokio::sync::broadcast;
use tokio::time::sleep;

#[tokio::main]
async fn main() {

    // let (tx, mut rx1) = broadcast::channel(16);
    let (tx, _) = broadcast::channel::<String>(16);

    for i in 0..2 {

        let mut rx = tx.subscribe();
        tokio::spawn(async move {

            loop {
                let evt_val = rx.recv().await.unwrap();
                println!("evt_val: {i:?} - {evt_val}");
            }

        });
    };

    let mut i = 1;
    loop {
        sleep(Duration::from_millis(1000)).await;
        tx.send(i.to_string()).unwrap();
        i = i+1;
    }
}