// main.rs

mod math;
mod config;

use math::{divide, square};
use config::AppConfig;

fn main() {
    let config = AppConfig::new();

    let a = 10;
    let result = divide(a, config.temp);

    println!("Result: {}", result);
    println!("Square: {}", square(result));
}
