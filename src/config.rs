use super::api::grpc;

#[derive(Debug)]
pub struct Config {
    pub grpc: grpc::config::Config,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        Self {
            grpc: grpc::config::Config::new(),
        }
    }
}
