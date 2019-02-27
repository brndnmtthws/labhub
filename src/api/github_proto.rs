use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;

#[derive(Debug)]
pub enum RequestError {
    BadCount,
    Missing,
}

pub struct XGitHubEvent(pub String);

impl<'a, 'r> FromRequest<'a, 'r> for XGitHubEvent {
    type Error = RequestError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let events: Vec<_> = request.headers().get("X-GitHub-Event").collect();

        match events.len() {
            0 => Outcome::Failure((Status::BadRequest, RequestError::Missing)),
            1 => Outcome::Success(XGitHubEvent(events[0].to_string())),
            _ => Outcome::Failure((Status::BadRequest, RequestError::BadCount)),
        }
    }
}

pub struct XHubSignature(pub String);

impl<'a, 'r> FromRequest<'a, 'r> for XHubSignature {
    type Error = RequestError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let signatures: Vec<_> = request.headers().get("X-Hub-Signature").collect();

        match signatures.len() {
            0 => Outcome::Failure((Status::BadRequest, RequestError::Missing)),
            1 => Outcome::Success(XHubSignature(signatures[0].to_string())),
            _ => Outcome::Failure((Status::BadRequest, RequestError::BadCount)),
        }
    }
}
