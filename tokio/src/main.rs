use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // This variable is owned by main function, so we can move it, move it to heap, or whatever we
    // want to do on its lifetime.
    // If this is a reference variable, then we cannot do it. We cannot use tricky way, like mem
    // operations, but it is notr recommanded.
    // Also, if it is a reference, it means the design of this function does want to limit its
    // lifetime and constraint some operations(danagerous operations).
    let owned_variable = 36;
    tokio::spawn(async move {
        loop {
            println!(
                "Hello from tokio spawned task! Moved owned_variable: {}",
                owned_variable
            );
            sleep(Duration::from_secs(1)).await;
        }
    });

    sleep(Duration::from_secs(5)).await;
    println!("Main task finished!");
}
