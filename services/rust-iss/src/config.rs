use std::env;
use std::time::Duration;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub iss_api_url: String,
    pub iss_api_key: String,
    pub http_timeout: Duration,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL missing"),
            iss_api_url: env::var("ISS_API_URL").expect("ISS_API_URL missing"),
            iss_api_key: env::var("ISS_API_KEY").expect("ISS_API_KEY missing"),
            http_timeout: Duration::from_secs(
                env::var("HTTP_TIMEOUT")
                    .unwrap_or("10".into())
                    .parse()
                    .unwrap(),
            ),
        }
    }
}
