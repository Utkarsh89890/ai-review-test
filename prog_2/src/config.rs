// config.rs

/// Global configuration values
pub struct AppConfig {
    pub temp: i32,
}

impl AppConfig {
    pub fn new() -> Self {
        Self {
            temp: 0, // ⚠️ dangerous default
        }
    }
}
