use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub passwords: Vec<u64>,
}

impl AppState {
    pub fn from(config: Config, passwords: Vec<u64>) -> Self {
        Self { config, passwords }
    }
}
