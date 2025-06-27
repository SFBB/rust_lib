async fn Hello_World_Async() {
    std::thread::sleep(std::time::Duration::from_secs(6));
    println!("Hello, world! From Async!\n");
}

#[tokio::main]
async fn main() {
    tokio::spawn(Hello_World_Async());
    println!("Hello, world! From main!\n");
}
