mod math_utils;
mod finance;
mod config;

use math_utils::divide;
use finance::calculate_interest;
use config::AppConfig;

fn main() {
    let p = 10000.0;
    let r = 5.0;
    let t = 2.0;

    let interest1 = finance::calculate_simple_interest(p, r, t);
    let interest2 = finance::calculate_interest_v2(p, r, t);

    let tax1 = math_utils::calculate_tax(interest1);
    let tax2 = math_utils::calculate_tax(interest2);

    println!("Interest1: {}, Tax1: {}", interest1, tax1);
    println!("Interest2: {}, Tax2: {}", interest2, tax2);

    let a = 10;
    let b = 0;
    let result = divide(a, b);
    println!("Result {}", result);

    let cfg = AppConfig::new();
    println!("Rate threshold {}", cfg.interest_treshhold); 
}
