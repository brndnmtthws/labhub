use crate::api::models::github;
use crate::config;
use crate::errors::GitError;

use log::error;
use reqwest;

fn headers(token: &str) -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::AUTHORIZATION,
        reqwest::header::HeaderValue::from_str(&format!("token {}", token)).unwrap(),
    );
    headers.insert(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/vnd.github.v3+json"),
    );
    headers.insert(
        reqwest::header::ACCEPT_ENCODING,
        reqwest::header::HeaderValue::from_static("Accept-Encoding: deflate, gzip"),
    );
    headers
}

fn make_repo_url(org: &str, repo: &str) -> String {
    let hostname = match config::CONFIG.github.hostname.as_ref() {
        Some(hostname) => hostname.clone(),
        _ => "github.com".to_string(),
    };
    format!("https://api.{}/repos/{}/{}", hostname, org, repo)
}

pub fn get_pull(
    client: &reqwest::Client,
    org: &str,
    repo: &str,
    number: i64,
) -> Result<github::RepoPr, GitError> {
    let res: github::RepoPr = client
        .get(&format!("{}/pulls/{}", make_repo_url(org, repo), number))
        .headers(headers(&config::CONFIG.github.api_token))
        .send()?
        .json()?;
    Ok(res)
}

pub fn create_issue_comment(
    client: &reqwest::Client,
    org: &str,
    repo: &str,
    number: i64,
    body: &str,
) -> Result<(), GitError> {
    let mut res = client
        .post(&format!(
            "{}/issues/{}/comments",
            make_repo_url(org, repo),
            number
        ))
        .headers(headers(&config::CONFIG.github.api_token))
        .body(serde_json::json!({"body":body.to_string()}).to_string())
        .send()?;
    let body = res.text()?;
    match res.status() {
        reqwest::StatusCode::CREATED => Ok(()),
        _ => {
            let msg = format!("Error creating issue comment: res={:#?} body={}", res, body);
            error!("{}", msg);
            Err(GitError { message: msg })
        }
    }
}
