use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
}

impl AppState {
    pub fn from(config: Config) -> Self {
        Self { config }
    }
}
