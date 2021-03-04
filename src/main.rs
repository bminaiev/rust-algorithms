use std::time::{SystemTime, UNIX_EPOCH};

mod lib;

fn main() {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros();
    println!("Hello: {}", now);
}