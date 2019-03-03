// Global configuration values, loaded from the environment
use log::info;
use std::collections::HashMap;
use std::env;
use std::sync::Mutex;
use yansi::Paint;

lazy_static! {
    pub static ref GITHUB_WEBHOOK_SECRET: String =
        env::var("GITHUB_WEBHOOK_SECRET").unwrap_or_else(|_| "github_webhook_secret".to_string());
}

lazy_static! {
    pub static ref GITHUB_USERNAME: String =
        env::var("GITHUB_USERNAME").unwrap_or_else(|_| "labhub".to_string());
}

lazy_static! {
    pub static ref GITHUB_SSH_KEY: String =
        env::var("GITHUB_SSH_KEY").unwrap_or_else(|_| "/path/to/id_rsa".to_string());
}

lazy_static! {
    pub static ref GITLAB_WEBHOOK_SECRET: String =
        env::var("GITLAB_WEBHOOK_SECRET").unwrap_or_else(|_| "gitlab_webhook_secret".to_string());
}

lazy_static! {
    pub static ref GITLAB_USERNAME: String =
        env::var("GITLAB_USERNAME").unwrap_or_else(|_| "labhub".to_string());
}

lazy_static! {
    pub static ref GITLAB_SSH_KEY: String =
        env::var("GITLAB_SSH_KEY").unwrap_or_else(|_| "/path/to/id_rsa".to_string());
}

fn insert_mappings_into_map<F>(insert: F) -> Mutex<HashMap<String, String>>
where
    F: Fn(&mut HashMap<String, String>, String, String),
{
    let mut m: HashMap<String, String> = HashMap::new();
    let mappings = env::var("LABHUB_MAPPINGS").unwrap_or_else(|_| "".to_string());
    let mapping_pairs: Vec<&str> = mappings.split(',').collect();
    for pair in mapping_pairs {
        let parts: Vec<&str> = pair.split('=').collect();
        if let 2 = parts.len() {
            insert(&mut m, parts[0].to_string(), parts[1].to_string());
        }
    }
    Mutex::new(m)
}

lazy_static! {
    pub static ref HUB_TO_LAB: Mutex<HashMap<String, String>> = {
        insert_mappings_into_map(|m, part1, part2| {
            m.insert(part1, part2);
        })
    };
}

lazy_static! {
    pub static ref LAB_TO_HUB: Mutex<HashMap<String, String>> = {
        insert_mappings_into_map(|m, part1, part2| {
            m.insert(part2, part1);
        })
    };
}

pub fn print() {
    info!("Loading LabHub configuration values from environment");
    info!(
        "❓ GITHUB_WEBHOOK_SECRET ====> {}",
        Paint::red(&*GITHUB_WEBHOOK_SECRET)
    );
    info!(
        "❓ GITHUB_USERNAME ==========> {}",
        Paint::red(&*GITHUB_USERNAME)
    );
    info!(
        "❓ GITHUB_SSH_KEY ===========> {}",
        Paint::red(&*GITHUB_SSH_KEY)
    );
    info!(
        "❓ GITLAB_WEBHOOK_SECRET ====> {}",
        Paint::red(&*GITLAB_WEBHOOK_SECRET)
    );
    info!(
        "❓ GITLAB_USERNAME ==========> {}",
        Paint::red(&*GITLAB_USERNAME)
    );
    info!(
        "❓ GITLAB_SSH_KEY ===========> {}",
        Paint::red(&*GITLAB_SSH_KEY)
    );
    info!(
        "❓ HUB_TO_LAB => {:#?}",
        Paint::red(&*HUB_TO_LAB.lock().unwrap())
    );
    info!(
        "❓ LAB_TO_HUB => {:#?}",
        Paint::red(&*LAB_TO_HUB.lock().unwrap())
    );
}
