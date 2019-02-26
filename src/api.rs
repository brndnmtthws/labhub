use crate::api::models::github;
use log::{info, trace, warn};
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::response::content;
use rocket::Outcome;
use rocket_contrib::json::Json;
mod models;

#[derive(Debug, Responder)]
#[response(status = 500, content_type = "json")]
pub struct ResponseError {
    response: content::Json<String>,
}

pub struct XGitHubEvent(String);

impl<'a, 'r> FromRequest<'a, 'r> for XGitHubEvent {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<XGitHubEvent, ()> {
        let events: Vec<_> = request.headers().get("X-GitHub-Event").collect();
        if events.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }
        let event = events[0];

        Outcome::Success(XGitHubEvent(event.to_string()))
    }
}

#[post("/events", format = "json", data = "<event>")]
pub fn github_event(
    event: Json<serde_json::Value>,
    github_event_type: XGitHubEvent,
) -> Result<content::Json<String>, ResponseError> {
    info!("Received GitHub webhook, type={}", github_event_type.0);
    match github_event_type.0.as_ref() {
        "push" => {
            let push: github::Push = serde_json::from_value(event.0).unwrap();
            info!("Push ref={}", push.r#ref.unwrap());
        }
        _ => println!("delp)"),
    }
    Ok(content::Json(json!({"hello":"hi"}).to_string()))
}

#[post("/events", format = "json", data = "<event>")]
pub fn gitlab_event(
    event: Json<serde_json::Value>,
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
