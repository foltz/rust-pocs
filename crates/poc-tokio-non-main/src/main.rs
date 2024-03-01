use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::sync::broadcast;
use tokio::time::sleep;


fn main() {

    let rt = Runtime::new().expect("Unable to create Runtime");

    // Enter the runtime so that `tokio::spawn` is available immediately.
    let _ = rt.enter();

    // Execute the runtime in its own thread.
    // The future doesn't have to do anything. In this example, it just sleeps forever.
    std::thread::spawn(move || {
        rt.block_on(async {
            tokio_main().await
        })
    });

    loop {}
}
async fn tokio_main() {

    // let (tx, mut rx1) = broadcast::channel(16);
    let (tx, _) = broadcast::channel(16);

    for i in 0..2 {

        let mut rx = tx.subscribe();
        tokio::spawn(async move {

            loop {
                let evt_val = rx.recv().await;
                // println!("received");
                if evt_val.is_ok() {
                    println!("evt_val: {i:?} - {}", evt_val.unwrap());
                } else {
                    println!("evt_val - ERROR: {}", evt_val.err().unwrap());
                }
                // sleep(Duration::from_millis(2000)).await;

            }

        });
    };


    let tx_1 = tx.clone();
    tokio::spawn(async move {
        let mut i = 1;
        loop {
            println!("send-1: {i:?}");
            tx_1.send(i).unwrap();
            i = i+1;
            sleep(Duration::from_millis(1000)).await;
        }
    });

    // tx.send(0).unwrap();

    let tx_100 = tx.clone();
    tokio::spawn(async move {
        let mut i = 101;
        loop {

            tx_100.send(i).unwrap();
            println!("send-100: {i:?}");
            i = i+1;
            sleep(Duration::from_millis(1000)).await;
        }
    });

    // - keep all channels open...
    loop {}

}