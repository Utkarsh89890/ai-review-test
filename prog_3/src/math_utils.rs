// this function divides numbers
pub fn divide(x: i32, y: i32) -> i32 {
    x / y
}
pub fn calculate_tax(amount: f64) -> f64 {
    let tax_rate = 0.18;
    amount * tax_rate
}

pub fn calculate_tax_helper(amount: f64) -> f64 {
    calculate_tax(amount)
}
