use std::time::Duration;

use tokio::task;

#[tokio::main]
async fn main() {

    let handle = tokio::spawn(async {
        for _ in 0..4 {
            tokio::time::sleep(Duration::from_secs(2)).await;
            println!("hi");
            task::yield_now().await;
        }
    });
    
    let handle2 =  tokio::spawn(async {
        for _ in 0..4 {
            tokio::time::sleep(Duration::from_secs(4)).await;
            println!("bye");
            task::yield_now().await;
        }
    });

    handle.await;
    handle2.await;

    println!("Hello, world!");
}
