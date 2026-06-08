use std::env;

#[derive(Clone)]
pub struct Config {
    pub dev: bool,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let dev = env::var("DEV")
            .unwrap_or_else(|_| "false".into())
            .to_lowercase()
            == "true";

        Config { dev }
    }
}
