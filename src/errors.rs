use crate::api::github_signature;
use crate::commands;

use rocket::request::Request;
use rocket::response::content;
use std::io;

#[derive(Debug, Responder)]
#[response(status = 500, content_type = "json")]
pub struct ResponseError {
    response: content::Json<String>,
}

#[derive(Debug, Responder)]
#[response(status = 400, content_type = "json")]
pub struct BadRequest {
    response: content::Json<String>,
}

#[derive(Debug, Responder)]
pub enum RequestErrorResult {
    #[response(status = 400, content_type = "json")]
    BadRequest(BadRequest),
    #[response(status = 500, content_type = "json")]
    ResponseError(ResponseError),
}

#[derive(Debug)]
pub struct GitError {
    pub message: String,
}

impl From<io::Error> for RequestErrorResult {
    fn from(error: io::Error) -> Self {
        RequestErrorResult::ResponseError {
            0: ResponseError {
                response: content::Json(json!({ "error": format!("{:?}", error) }).to_string()),
            },
        }
    }
}

impl From<github_signature::SignatureError> for RequestErrorResult {
    fn from(error: github_signature::SignatureError) -> Self {
        RequestErrorResult::BadRequest {
            0: BadRequest {
                response: content::Json(json!({ "error": format!("{:?}", error) }).to_string()),
            },
        }
    }
}

impl From<std::option::NoneError> for RequestErrorResult {
    fn from(error: std::option::NoneError) -> Self {
        RequestErrorResult::BadRequest {
            0: BadRequest {
                response: content::Json(json!({ "error": format!("{:?}", error) }).to_string()),
            },
        }
    }
}

impl From<serde_json::error::Error> for RequestErrorResult {
    fn from(error: serde_json::error::Error) -> Self {
        RequestErrorResult::BadRequest {
            0: BadRequest {
                response: content::Json(json!({ "error": format!("{:?}", error) }).to_string()),
            },
        }
    }
}

impl From<git2::Error> for GitError {
    fn from(error: git2::Error) -> Self {
        GitError {
            message: format!("Git error: {:?}", error.message()),
        }
    }
}

impl From<std::option::NoneError> for GitError {
    fn from(error: std::option::NoneError) -> Self {
        GitError {
            message: format!("Git error: {:?}", error),
        }
    }
}

impl From<io::Error> for GitError {
    fn from(error: io::Error) -> Self {
        GitError {
            message: format!("Git error: {:?}", error),
        }
    }
}

impl From<serde_json::error::Error> for GitError {
    fn from(error: serde_json::error::Error) -> Self {
        GitError {
            message: format!("Github serde error: {:?}", error),
        }
    }
}

impl From<reqwest::Error> for GitError {
    fn from(error: reqwest::Error) -> Self {
        GitError {
            message: format!("Git request error: {:?}", error),
        }
    }
}

impl From<commands::CommandError> for GitError {
    fn from(error: commands::CommandError) -> Self {
        GitError {
            message: format!("Git command error: {:?}", error),
        }
    }
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
