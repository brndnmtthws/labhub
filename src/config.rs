// Global configuration values, loaded from the environment

lazy_static! {
    static GITHUB_WEBHOOK_SECRET: String = var("GITHUB_WEBHOOK_SECRET").unwrap();
}
