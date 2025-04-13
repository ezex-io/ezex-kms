use std::env;

#[derive(Debug)]
pub struct Config {
    pub address: String,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        Self {
            address: env::var("EZEX_KMS_GRPC_ADDRESS").unwrap_or("0.0.0.0:39577".to_string()),
        }
    }
}
