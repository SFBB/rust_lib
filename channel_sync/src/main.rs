use std::sync::{
    Arc, Mutex,
    mpsc::{Receiver, Sender, channel},
};
use std::thread;
use std::thread::JoinHandle;

#[derive(Clone)]
struct Test {
    a: String,
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
                match rx.lock().unwrap().recv() {
                    Ok(received_test) => {
                        println!("received test.a: {}", received_test.a);
                        if counter >= 1 {
                            return 0;
                        }
                    }
                    Err(_) => {
                        return 1;
                    }
                }
                counter += 1;
            }
        }));
    }

    pub fn block(&mut self) {
        if let Some(handle) = self.t.take() {
            // Take the JoinHandle
            handle.join().unwrap(); // Handle potential error
        }
    }
}

impl Target {
    pub fn new(new_b: String) -> Self {
        Target { b: new_b }
    }
}

fn main() {
    let t = Test {
        a: String::from("Hello World!"),
    };
    let target = Target::new(t.a);
    // println!("t.a: {}", t.a);
    println!("target.b: {}", target.b);
    println!("Hello, world!");

    let (tx, rx): (Sender<Test>, Receiver<Test>) = channel();

    let mut manager = Manager::new(rx);
    manager.run();
    let tt = Test {
        a: String::from("Hello World! from main thread!"),
    };
    tx.send(tt.clone());
    tx.send(tt);
    manager.block();
}
