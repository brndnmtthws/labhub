use crate::api::{github_proto, github_signature};
use crate::config;
use crate::errors;
use crate::github;

use log::{debug, info};
use rocket::data::Data;
use rocket::response::content;
use rocket_contrib::json::Json;
use std::io::Read;

const MAX_BODY_LENGTH: u64 = 10 * 1024 * 1024;

#[get("/check")]
pub fn check() -> &'static str {
    "ok"
}

#[post("/events", format = "json", data = "<body_data>")]
pub fn github_event(
    body_data: Data,
    event_type: github_proto::XGitHubEvent,
    signature: github_proto::XHubSignature,
) -> Result<content::Json<String>, errors::RequestErrorResult> {
    info!("Received GitHub webhook, type={}", event_type.0);

    // Read request body
    let mut body = String::with_capacity(MAX_BODY_LENGTH as usize);
    body_data
        .open()
        .take(MAX_BODY_LENGTH)
        .read_to_string(&mut body)?;

    // Check X-Hub-Signature
    github_signature::check_signature(
        &config::CONFIG.github.webhook_secret.clone(),
        &signature.0,
        &body,
    )?;

    debug!("body={}", body);

    // Handle the event
    Ok(content::Json(github::handle_event_body(
        &event_type.0.as_ref(),
        &body,
    )?))
}

#[post("/events", format = "json", data = "<event>")]
pub fn gitlab_event(
    event: Json<serde_json::Value>,
) -> Result<content::Json<String>, errors::RequestErrorResult> {
    info!("{:?}", event.0);
    Ok(content::Json(json!({"hello":"hi"}).to_string()))
}
