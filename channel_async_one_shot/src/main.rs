use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use tokio::sync::mpsc::{Receiver, Sender, channel};
use tokio::sync::oneshot;

struct Test {
    a: String,
    response_tx: Option<oneshot::Sender<()>>,
}

struct Target {
    b: String,
}

struct Manager {
    rx: Arc<Mutex<Receiver<Test>>>,
    t: Option<JoinHandle<i32>>,
}

impl Manager {
    pub fn new(new_rx: Receiver<Test>) -> Self {
        Manager {
            rx: Arc::new(Mutex::new(new_rx)),
            t: None,
        }
    }

    pub fn run(&mut self) {
        let rx = Arc::clone(&self.rx);
        self.t = Some(thread::spawn(move || {
            let mut counter = 0;
            loop {
                match rx.lock().unwrap().blocking_recv() {
                    Some(received_test) => {
                        println!("received test.a: {}", received_test.a);
                        match received_test.response_tx {
                            Some(sender) => {
                                sender.send(());
                            }
                            None => {}
                        }
                        if counter >= 5 {
                            return 0;
                        }
                    }
                    None => {
                        return 1;
                    }
                }
                counter += 1;
            }
        }));
    }

    pub fn block(&mut self) {
        if let handle = std::mem::take(&mut self.t) {
            match handle {
                Some(h) => {
                    h.join().unwrap();
                }
                None => {}
            }
        }
        // or
        // if let Some(handle) = self.t.take() { // Take the JoinHandle
        // handle.join().unwrap(); // Handle potential error
        // }
    }
}

impl Target {
    pub fn new(new_b: String) -> Self {
        Target { b: new_b }
    }
}

async fn send_message(message: String, tx: &Sender<Test>) {
    let (response_tx, response_rx) = oneshot::channel();
    let test = Test {
        a: message,
        response_tx: Some(response_tx),
    };
    tx.send(test).await;
    response_rx.await;
}

#[tokio::main]
async fn main() {
    let t = Test {
        a: String::from("Hello World!"),
        response_tx: None,
    };
    let target = Target::new(t.a);
    // println!("t.a: {}", t.a);
    println!("target.b: {}", target.b);
    println!("Hello, world!");

    let (tx, rx): (Sender<Test>, Receiver<Test>) = channel(32);

    let mut manager = Manager::new(rx);
    manager.run();
    let tt = Test {
        a: String::from("Hello World! from main thread!"),
        response_tx: None,
    };
    // tx.send(tt.clone()).await;
    tx.send(tt);
    send_message(String::from("Hello world! 0 in main thread!"), &tx).await;
    println!("We've sent message 0!");
    send_message(String::from("Hello world! 1 in main thread!"), &tx).await;
    println!("We've sent message 1!");
    send_message(String::from("Hello world! 2 in main thread!"), &tx).await;
    println!("We've sent message 2!");
    send_message(String::from("Hello world! 3 in main thread!"), &tx).await;
    println!("We've sent message 3!");
    send_message(String::from("Hello world! 4 in main thread!"), &tx).await;
    println!("We've sent message 4!");
    send_message(String::from("Hello world! 5 in main thread!"), &tx).await;
    println!("We've sent message 5!");
    println!("We've sent all messages!");
    manager.block();
}
