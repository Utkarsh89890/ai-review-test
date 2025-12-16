pub struct AppConfig {
    pub interest_treshhold: f64,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        AppConfig {
            interest_treshhold: 0.0,
        }
    }
}
