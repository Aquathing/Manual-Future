use std::time::Duration;

use testone::ReadIntoBuf;
/*
#[tokio::main]
async fn main() {
    let mut manual_future = manualfuture::FutureCompletion::<String>::new();
    let future = manual_future.get_future();
    tokio::spawn(async {
        let string = future.await;
        println!("{}", string);
    });
    
    tokio::time::sleep(Duration::from_secs(10)).await;
    manual_future.set_result("Hello!".to_string());
    manual_future.get_future().await;
}
*/
