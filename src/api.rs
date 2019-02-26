use crate::api::models::github;
use rocket::response::content;
use rocket::Request;
use rocket_contrib::json::Json;
mod models;
use log::{info, trace, warn};

#[derive(Debug, Responder)]
#[response(status = 500, content_type = "json")]
pub struct ResponseError {
    response: content::Json<String>,
}

#[post("/events", format = "json", data = "<event>")]
pub fn github_event(
    event: Json<github::WebhookEvent>,
) -> Result<content::Json<String>, ResponseError> {
    info!("{:?}", event.0);
    Ok(content::Json(json!({"hello":"hi"}).to_string()))
}

#[post("/events", format = "json", data = "<event>")]
pub fn gitlab_event(
    event: Json<github::WebhookEvent>,
) -> Result<content::Json<String>, ResponseError> {
    info!("{:?}", event.0);
    Ok(content::Json(json!({"hello":"hi"}).to_string()))
}

#[catch(404)]
pub fn not_found(req: &Request) -> content::Json<String> {
    content::Json(
        json!({
            "error":
                format!(
                    "Look elsewhere, perhaps? No matching route for uri={}",
                    req.uri()
                )
        })
        .to_string(),
    )
}

#[catch(500)]
pub fn internal_server_error(_req: &Request) -> content::Json<String> {
    content::Json(
        json!({
            "error":"Internal server error 🤖"
        })
        .to_string(),
    )
}

#[catch(422)]
pub fn unprocessable_entity(_req: &Request) -> content::Json<String> {
    content::Json(
        json!({
            "error":"The request was well-formed but was unable to be followed due to semantic errors."
        })
        .to_string(),
    )
}
