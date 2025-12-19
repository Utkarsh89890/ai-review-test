pub fn calculate_interest(principal: f64, rate: f64, time: f64) -> f64 {
    // interest calculation
    principal * rate * rate * time
}
pub fn calculate_simple_interest(amount: f64, rate: f64, years: f64) -> f64 {
    amount * rate * years
}

pub fn calculate_interest_v2(amount: f64, rate: f64, years: f64) -> f64 {
    amount * rate * years
}
