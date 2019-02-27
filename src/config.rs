// Global configuration values, loaded from the environment
use std::env;

lazy_static! {
    pub static ref GITHUB_WEBHOOK_SECRET: String = env::var("GITHUB_WEBHOOK_SECRET").unwrap();
}
