use crate::api::models::gitlab;
use crate::config;
use crate::errors::GitError;

use log::error;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use reqwest;

fn headers(token: &str) -> reqwest::header::HeaderMap {
    let token_header = reqwest::header::HeaderName::from_static("private-token");
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        token_header,
        reqwest::header::HeaderValue::from_str(token).unwrap(),
    );
    headers.insert(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
    );
    headers.insert(
        reqwest::header::ACCEPT_ENCODING,
        reqwest::header::HeaderValue::from_static("Accept-Encoding: deflate, gzip"),
    );
    headers
}

fn make_api_url(project: &str) -> String {
    let hostname = match config::CONFIG.gitlab.hostname.as_ref() {
        Some(hostname) => hostname.clone(),
        _ => "gitlab.com".to_string(),
    };
    const FRAGMENT: &AsciiSet = &CONTROLS.add(b'/').add(b'%');
    let project = utf8_percent_encode(project, FRAGMENT).to_string();
    format!("https://{}/api/v4/projects/{}", hostname, project)
}

pub fn make_ext_url(project: &str) -> String {
    let hostname = match config::CONFIG.gitlab.hostname.as_ref() {
        Some(hostname) => hostname.clone(),
        _ => "gitlab.com".to_string(),
    };
    format!("https://{}/{}", hostname, project)
}

pub fn get_pipelines(
    client: &reqwest::Client,
    project: &str,
    page: i64,
    per_page: i64,
) -> Result<Vec<gitlab::Pipeline>, GitError> {
    let res: Vec<gitlab::Pipeline> = client
        .get(&format!(
            "{}/pipelines?page={}&per_page={}",
            make_api_url(project),
            page,
            per_page
        ))
        .headers(headers(&config::CONFIG.gitlab.api_token))
        .send()?
        .json()?;
    Ok(res)
}

pub fn retry_pipeline(
    client: &reqwest::Client,
    project: &str,
    pipeline_id: i64,
) -> Result<(), GitError> {
    let res = client
        .post(&format!(
            "{}/pipelines/{}/retry",
            make_api_url(project),
            pipeline_id
        ))
        .headers(headers(&config::CONFIG.gitlab.api_token))
        .send()?;

    match res.status() {
        reqwest::StatusCode::CREATED => Ok(()),
        _ => {
            let msg = format!("Error retrying pipeline: {:#?}", res);
            error!("{}", msg);
            Err(GitError { message: msg })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_make_ext_url() {
        assert_eq!(
            make_ext_url("brndnmtthws-oss/conky"),
            "https://gitlab.com/brndnmtthws-oss/conky"
        );
    }

    #[test]
    fn test_make_api_url() {
        assert_eq!(
            make_api_url("brndnmtthws-oss/conky"),
            "https://gitlab.com/api/v4/projects/brndnmtthws-oss%2Fconky"
        );
    }
}
