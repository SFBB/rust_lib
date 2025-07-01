#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

lazy_static! {
    static ref GLOBAL_VAR: Mutex<i32> = Mutex::new(0);
}
fn main() {
    println!("Hello, world!");
    println!("GLOBAL_VAR: {}", GLOBAL_VAR.lock().unwrap());
}
