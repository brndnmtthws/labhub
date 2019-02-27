use crate::api::{github_proto, github_signature};
use crate::config;
use log::info;
use rocket::data::Data;
use rocket::request::Request;
use rocket::response::content;
use rocket_contrib::json::Json;
use std::io::{self, Read};

#[derive(Debug, Responder)]
#[response(status = 500, content_type = "json")]
pub struct ResponseError {
    response: content::Json<String>,
}

impl From<io::Error> for ResponseError {
    fn from(error: io::Error) -> Self {
        ResponseError {
            response: content::Json(json!({ "error": format!("{:?}", error) }).to_string()),
        }
    }
}

impl From<github_signature::SignatureError> for ResponseError {
    fn from(error: github_signature::SignatureError) -> Self {
        ResponseError {
            response: content::Json(json!({ "error": format!("{:?}", error) }).to_string()),
        }
    }
}

const MAX_BODY_LENGTH: u64 = 10 * 1024 * 1024;

#[post("/events", format = "json", data = "<body_data>")]
pub fn github_event(
    body_data: Data,
    event_type: github_proto::XGitHubEvent,
    signature: github_proto::XHubSignature,
) -> Result<content::Json<String>, ResponseError> {
    info!("Received GitHub webhook, type={}", event_type.0);
    let mut body = String::with_capacity(MAX_BODY_LENGTH as usize);
    body_data
        .open()
        .take(MAX_BODY_LENGTH)
        .read_to_string(&mut body)?;
    github_signature::check_signature(&config::GITHUB_WEBHOOK_SECRET, &signature.0, &body)?;
    // match event_type.0.as_ref() {
    //     "push" => {
    //         let push: github::Push = serde_json::from_value(event.0).unwrap();
    //         info!("Push ref={}", push.r#ref.unwrap());
    //     }
    //     _ => println!("delp)"),
    // }
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
