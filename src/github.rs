use crate::api::models::github;
use crate::api::{github_client, gitlab_client};
use crate::commands;
use crate::config;
use crate::errors::{GitError, RequestErrorResult};

use git2::build::RepoBuilder;
use git2::{FetchOptions, PushOptions, RemoteCallbacks, Repository};
use log::{debug, error, info, warn};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;
use std::thread;
use tempfile::{tempdir, TempDir};

#[cfg(test)]
use mockers_derive::mocked;

struct RepoData {
    repo: Repository,
    #[allow(dead_code)]
    dir: TempDir,
}

lazy_static! {
    static ref REPOS: Mutex<HashMap<String, RepoData>> = {
        #[allow(unused_mut)]
        let mut m: HashMap<String, RepoData> = HashMap::new();
        Mutex::new(m)
    };
}

fn get_gitlab_repo_name(github_repo_full_name: &str) -> String {
    let hub_to_lab_lock = config::HUB_TO_LAB.lock().unwrap();
    let hub_to_lab = &*hub_to_lab_lock;
    if hub_to_lab.contains_key(github_repo_full_name) {
        hub_to_lab.get(github_repo_full_name).unwrap().to_string()
    } else {
        github_repo_full_name.to_string()
    }
}

fn get_remote_callbacks(site: &config::Site) -> RemoteCallbacks {
    let mut remote_callbacks = RemoteCallbacks::new();
    let ssh_key = site.ssh_key.clone();
    remote_callbacks.credentials(move |_user, _user_from_url, cred| {
        debug!("Entered Git credential callback, cred={:?}", cred);
        if cred.contains(git2::CredentialType::USERNAME) {
            git2::Cred::username(&"git".to_string())
        } else {
            let path = Path::new(&ssh_key);
            git2::Cred::ssh_key(&"git".to_string(), None, &path, None)
        }
    });
    remote_callbacks.push_update_reference(|reference, status_option| {
        match status_option {
            Some(status) => error!(
                "Failed to update remote ref {} message={:?}",
                reference, status
            ),
            _ => info!("Updated remote ref {}", reference),
        };
        Ok(())
    });
    remote_callbacks.update_tips(|reference, oid1, oid2| {
        debug!(
            "Updated tips, ref={} oid1={} oid2={}",
            reference, oid1, oid2
        );
        true
    });
    remote_callbacks
}

#[cfg_attr(test, mocked)]
trait RepositoryExt {
    fn add_remotes(&mut self, pr_handle: &PrHandle) -> Result<(), GitError>;
    fn fetch_github_remote(&self, pr_handle: &PrHandle) -> Result<(), GitError>;
    fn create_ref_for_pr(&self, pr_handle: &PrHandle) -> Result<(), GitError>;
    fn push_pr_ref(&self, pr_handle: &PrHandle) -> Result<(), GitError>;
    fn delete_pr_ref(&self, pr_handle: &PrHandle) -> Result<(), GitError>;
}

#[derive(Debug, Eq, PartialEq)]
pub struct PrHandle {
    base_full_name: String,
    head_full_name: String,
    github_remote: String,
    gitlab_remote: String,
    gitref: String,
    github_clone_url: String,
    pr_number: i64,
}

impl PrHandle {
    fn new(pr: &github::PullRequest) -> Result<PrHandle, std::option::NoneError> {
        let pr_handle = PrHandle {
            gitref: pr
                .pull_request
                .as_ref()?
                .head
                .as_ref()?
                .ref_key
                .as_ref()?
                .clone(),
            pr_number: pr.pull_request.as_ref()?.number?,
            github_clone_url: pr
                .pull_request
                .as_ref()?
                .head
                .as_ref()?
                .repo
                .as_ref()?
                .ssh_url
                .as_ref()?
                .clone(),
            github_remote: format!("github-{}", pr.pull_request.as_ref()?.number?,),
            gitlab_remote: "gitlab".to_string(),
            base_full_name: pr
                .pull_request
                .as_ref()?
                .base
                .as_ref()?
                .repo
                .as_ref()?
                .full_name
                .as_ref()?
                .clone(),
            head_full_name: pr
                .pull_request
                .as_ref()?
                .head
                .as_ref()?
                .repo
                .as_ref()?
                .full_name
                .as_ref()?
                .clone(),
        };
        Ok(pr_handle)
    }
}

impl RepositoryExt for Repository {
    fn add_remotes(&mut self, pr_handle: &PrHandle) -> Result<(), GitError> {
        let github_refspec = format!("+refs/heads/*:refs/remotes/{}/*", pr_handle.github_remote);
        self.remote_add_fetch(&pr_handle.github_remote, &github_refspec)?;
        self.remote_set_url(&pr_handle.github_remote, &pr_handle.github_clone_url)?;
        let hostname = match config::CONFIG.gitlab.hostname.as_ref() {
            Some(hostname) => hostname.clone(),
            _ => "gitlab.com".to_string(),
        };
        let gitlab_url = format!(
            "git@{}:{}.git",
            hostname,
            get_gitlab_repo_name(&pr_handle.base_full_name)
        );
        let gitlab_refspec = "refs/heads/master:refs/heads/master".to_string();
        self.remote_add_push(&pr_handle.gitlab_remote, &gitlab_refspec)?;
        self.remote_set_url(&pr_handle.gitlab_remote, &gitlab_url)?;
        Ok(())
    }

    fn fetch_github_remote(&self, pr_handle: &PrHandle) -> Result<(), GitError> {
        info!(
            "Fetching remote={} ref={}",
            pr_handle.github_remote, pr_handle.gitref
        );
        let mut remote = self.find_remote(&pr_handle.github_remote)?;

        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(get_remote_callbacks(&config::CONFIG.github));

        remote.fetch(&[&pr_handle.gitref], Some(&mut fetch_options), None)?;

        info!("Successfully fetched remote");
        Ok(())
    }

    fn create_ref_for_pr(&self, pr_handle: &PrHandle) -> Result<(), GitError> {
        let github_ref = format!(
            "refs/remotes/{}/{}",
            pr_handle.github_remote, pr_handle.gitref
        );
        let gitlab_ref = format!(
            "refs/heads/pr-{}/{}/{}",
            pr_handle.pr_number, pr_handle.head_full_name, pr_handle.gitref
        );
        let id = self.refname_to_id(&github_ref)?;
        debug!("Creating ref {} from {}, id={}", gitlab_ref, github_ref, id);
        self.reference(&gitlab_ref, id, true, "new ref")?;
        Ok(())
    }

    fn push_pr_ref(&self, pr_handle: &PrHandle) -> Result<(), GitError> {
        info!(
            "Pushing PR remote={} ref={} number={} base_full_name={}",
            pr_handle.gitlab_remote,
            pr_handle.gitref,
            pr_handle.pr_number,
            pr_handle.base_full_name
        );
        let mut gitremote = self.find_remote(&pr_handle.gitlab_remote)?;
        let mut push_options = PushOptions::new();
        push_options.remote_callbacks(get_remote_callbacks(&config::CONFIG.gitlab));

        let refspec = format!(
            "+refs/heads/pr-{}/{}/{}:refs/heads/pr-{}/{}/{}",
            pr_handle.pr_number,
            pr_handle.head_full_name,
            pr_handle.gitref,
            pr_handle.pr_number,
            pr_handle.head_full_name,
            pr_handle.gitref
        );
        gitremote.push(&[&refspec], Some(&mut push_options))?;

        info!("Successfully pushed");
        Ok(())
    }

    fn delete_pr_ref(&self, pr_handle: &PrHandle) -> Result<(), GitError> {
        info!(
            "Deleting PR remote={} ref={} number={} base_full_name={}",
            pr_handle.gitlab_remote,
            pr_handle.gitref,
            pr_handle.pr_number,
            pr_handle.base_full_name
        );
        let mut gitremote = self.find_remote(&pr_handle.gitlab_remote)?;
        let mut push_options = PushOptions::new();
        push_options.remote_callbacks(get_remote_callbacks(&config::CONFIG.gitlab));

        let refspec = format!(
            ":refs/heads/pr-{}/{}/{}",
            pr_handle.pr_number, pr_handle.head_full_name, pr_handle.gitref,
        );
        gitremote.push(&[&refspec], Some(&mut push_options))?;

        info!("Successfully pushed");
        Ok(())
    }
}

fn clone_repo(url: &str) -> Result<RepoData, GitError> {
    // Setup fetch options
    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(get_remote_callbacks(&config::CONFIG.github));

    // Initialize & clone repo
    let mut builder = RepoBuilder::new();
    builder.fetch_options(fetch_options);
    let dir = tempdir()?;
    match builder.clone(url, dir.as_ref()) {
        Ok(repo) => {
            info!(
                "Cloned new repo {} into {}",
                url,
                dir.as_ref().to_str().unwrap()
            );

            Ok(RepoData { repo, dir })
        }
        Err(err) => {
            let msg = format!("Error cloning repo: {:?}", err);
            error!("{}", &msg);
            Err(GitError { message: msg })
        }
    }
}

fn handle_pr_closed_with_repo(
    repo: &mut dyn RepositoryExt,
    pr: &github::PullRequest,
) -> Result<String, GitError> {
    let pr_handle = PrHandle::new(pr)?;

    info!("pr_handle={:#?}", pr_handle);

    repo.add_remotes(&pr_handle)?;
    repo.delete_pr_ref(&pr_handle)?;

    Ok(String::from("deleted :D"))
}

fn handle_pr_closed(pr: &github::PullRequest) -> Result<String, GitError> {
    info!("Handling closed PR");
    let url = pr.repository.as_ref()?.ssh_url.as_ref()?;
    let mut repos = REPOS.lock();
    let repo_data = repos
        .as_mut()
        .unwrap()
        .entry(url.clone())
        .or_insert(clone_repo(url)?);

    handle_pr_closed_with_repo(&mut repo_data.repo, pr)
}

fn handle_pr_updated(pr: &github::PullRequest) -> Result<String, GitError> {
    info!("Handling open PR");
    let url = pr.repository.as_ref()?.ssh_url.as_ref()?;
    let mut repos = REPOS.lock();
    let repo_data = repos
        .as_mut()
        .unwrap()
        .entry(url.clone())
        .or_insert(clone_repo(url)?);

    handle_pr_updated_with_repo(&mut repo_data.repo, pr)
}

fn handle_pr_updated_with_repo(
    repo: &mut dyn RepositoryExt,
    pr: &github::PullRequest,
) -> Result<String, GitError> {
    info!("handle_pr_updated_with_repo");
    let pr_handle = PrHandle::new(pr)?;

    info!("pr_handle={:#?}", pr_handle);

    repo.add_remotes(&pr_handle)?;
    repo.fetch_github_remote(&pr_handle)?;
    repo.create_ref_for_pr(&pr_handle)?;
    repo.push_pr_ref(&pr_handle)?;

    Ok(String::from(":)"))
}

impl github::PullRequest {
    fn is_fork(&self) -> Result<bool, std::option::NoneError> {
        Ok(self
            .pull_request
            .as_ref()?
            .head
            .as_ref()?
            .repo
            .as_ref()?
            .fork?)
    }
}

fn handle_pr(pr: github::PullRequest) -> Result<(), RequestErrorResult> {
    if pr.is_fork()? {
        info!("PR is a fork");
        let result = match pr.action.as_ref()?.as_ref() {
            "closed" => handle_pr_closed(&pr),
            _ => handle_pr_updated(&pr),
        };
        match result {
            Ok(ok) => info!("Handled PR: {}", ok),
            Err(err) => error!("Caught error handling PR: {:?}", err),
        }
    } else {
        info!("Skipping PR because it's not a fork, cya 👋");
    }
    Ok(())
}

fn write_issue_comment(
    client: &reqwest::Client,
    ic: &github::IssueComment,
    body: &str,
) -> Result<(), GitError> {
    let repo_full_name = ic.repository.as_ref()?.full_name.as_ref()?;
    let repo_full_name_parts: Vec<String> = repo_full_name
        .split('/')
        .map(std::string::ToString::to_string)
        .collect();
    if repo_full_name_parts.len() != 2 {
        return Err(GitError {
            message: format!("Invalid repo name {}", repo_full_name),
        });
    }
    github_client::create_issue_comment(
        client,
        &repo_full_name_parts[0],
        &repo_full_name_parts[1],
        ic.issue.as_ref()?.number?,
        body,
    )
}

fn get_sha(client: &reqwest::Client, ic: &github::IssueComment) -> Result<String, GitError> {
    let repo_full_name = ic.repository.as_ref()?.full_name.as_ref()?;
    let repo_full_name_parts: Vec<String> = repo_full_name
        .split('/')
        .map(std::string::ToString::to_string)
        .collect();
    if repo_full_name_parts.len() != 2 {
        return Err(GitError {
            message: format!("Invalid repo name {}", repo_full_name),
        });
    }
    let pr = github_client::get_pull(
        client,
        &repo_full_name_parts[0],
        &repo_full_name_parts[1],
        ic.issue.as_ref()?.number?,
    )?;
    Ok(pr.head.as_ref()?.sha.as_ref()?.clone())
}

impl github::IssueComment {
    fn is_from_pr(&self) -> Result<bool, std::option::NoneError> {
        Ok(self.issue.as_ref()?.pull_request.is_some())
    }
}

fn find_pipeline_id(client: &reqwest::Client, project: &str, sha: &str) -> Result<i64, GitError> {
    let mut result_len = 100;
    let mut page = 1;
    while result_len == 100 {
        let pipelines = gitlab_client::get_pipelines(client, project, page, 100)?;
        let pipeline = pipelines
            .iter()
            .filter(|p| p.sha.is_some() && p.id.is_some())
            .find(|p| p.sha.as_ref().unwrap() == sha);
        if let Some(pipeline) = pipeline {
            return Ok(pipeline.id.unwrap());
        }
        result_len = pipelines.len();
        page += 1;
    }
    Err(GitError {
        message: format!(
            "Unable to find pipeline for project={} sha={}",
            project, sha
        ),
    })
}

fn handle_retry_command(
    client: &reqwest::Client,
    ic: &github::IssueComment,
) -> Result<(), GitError> {
    let repo_full_name = ic.repository.as_ref()?.full_name.as_ref()?;
    let sha = get_sha(&client, ic)?;
    let project = get_gitlab_repo_name(&repo_full_name);
    info!("Got retry command for project={} sha={}", project, sha);
    let pipeline_id = find_pipeline_id(&client, &get_gitlab_repo_name(&project), &sha)?;
    gitlab_client::retry_pipeline(&client, &project, pipeline_id)?;

    let comment_body = format!(
        "Sent **retry** command for pipeline [**{}**]({}/pipelines/{}) on [**GitLab**]({})

Have a great day! 😄",
        pipeline_id,
        gitlab_client::make_ext_url(&project),
        pipeline_id,
        gitlab_client::make_ext_url(&project),
    );

    write_issue_comment(&client, ic, &comment_body)
}

fn handle_pr_ic(ic: github::IssueComment) -> Result<(), GitError> {
    let client = reqwest::Client::new();
    info!(
        "Issue comment received for issue number={} action={}",
        ic.issue.as_ref()?.number?,
        ic.action.as_ref()?,
    );

    if ic.sender.as_ref()?.login.as_ref()? == &config::CONFIG.github.username {
        info!("Hey this is my comment :D Skipping");
        return Ok(());
    }

    let command_res = commands::parse_body(
        ic.comment.as_ref()?.body.as_ref()?,
        &*config::CONFIG.github.username,
    );

    match command_res {
        Err(commands::CommandError::UnknownCommand) => {
            // Write a comment on the PR
            let comment_body = "Sorry, but I don't know what that command means.

Thanks for asking 🥰"
                .to_string();

            write_issue_comment(&client, &ic, &comment_body)?;
            Ok(())
        }
        _ => {
            let command = command_res.unwrap();

            if !config::command_enabled(&command.command) {
                warn!("Command {:#?} is not enabled.", command.command);
                Ok(())
            } else {
                match command.command {
                    commands::CommandAction::Retry => handle_retry_command(&client, &ic),
                }
            }
        }
    }
}

fn handle_ic(ic: github::IssueComment) {
    if ic.is_from_pr().unwrap() {
        match handle_pr_ic(ic) {
            Ok(()) => info!("Finished handling issue comment"),
            Err(_err) => info!("Ignoring issue comment because it's invalid"),
        }
    }
}

pub fn handle_event_body(event_type: &str, body: &str) -> Result<String, RequestErrorResult> {
    match event_type {
        "push" => {
            let push: github::Push = serde_json::from_str(body)?;
            info!("Push ref={}", push.ref_key.as_ref()?);
            Ok(String::from("Push received 😘"))
        }
        "pull_request" => {
            if config::feature_enabled(&config::Feature::ExternalPr) {
                let pr: github::PullRequest = serde_json::from_str(body)?;
                info!("PullRequest action={}", pr.action.as_ref()?);
                thread::spawn(move || handle_pr(pr));
            } else {
                info!("ExternalPr feature not enabled. Skipping event.");
            }
            Ok(String::from("Thanks buddy bro 😍"))
        }
        "issue_comment" => {
            if config::feature_enabled(&config::Feature::Commands) {
                let ic: github::IssueComment = serde_json::from_str(body)?;
                info!(
                    "Issue comment action={} user={}",
                    ic.action.as_ref()?,
                    ic.issue.as_ref()?.user.as_ref()?.login.as_ref()?
                );
                thread::spawn(move || handle_ic(ic));
            } else {
                info!("Commands feature not enabled. Skipping event.");
            }
            Ok(String::from("Issue comment received 🥳"))
        }
        _ => Ok(format!(
            "Unhandled event_type={}, doing nothing 😀",
            event_type,
        )),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::testing::{read_testdata_to_string, run_test};
    // use mockers::Scenario;
    #[test]
    fn open_pr() {
        run_test(|| {
            info!("open_pr test");
            let pr: github::PullRequest =
                serde_json::from_str(&read_testdata_to_string("github_open_pull_request.json"))
                    .unwrap();
            assert_eq!(pr.is_fork().unwrap(), false);
            let _pr_handle = PrHandle::new(&pr).unwrap();
        });
    }

    #[test]
    fn reopen_pr() {
        run_test(|| {
            info!("reopen_pr test");
            let pr: github::PullRequest =
                serde_json::from_str(&read_testdata_to_string("github_reopen_pull_request.json"))
                    .unwrap();
            assert_eq!(pr.is_fork().unwrap(), false);
            let _pr_handle = PrHandle::new(&pr).unwrap();
        });
    }

    #[test]
    fn open_pr_fork() {
        run_test(|| {
            info!("open_pr_fork test");
            let pr: github::PullRequest =
                serde_json::from_str(&read_testdata_to_string("github_open_pr_forked.json"))
                    .unwrap();
            assert_eq!(pr.is_fork().unwrap(), true);
            let _pr_handle = PrHandle::new(&pr).unwrap();
        });
    }

    #[test]
    fn close_pr_fork() {
        run_test(|| {
            info!("close_pr_fork test");
            let pr: github::PullRequest =
                serde_json::from_str(&read_testdata_to_string("github_close_pr_forked.json"))
                    .unwrap();
            let _pr_handle = PrHandle::new(&pr).unwrap();
        });
    }

    #[test]
    fn get_pr() {
        run_test(|| {
            info!("get_pr test");
            let _pr: github::RepoPr =
                serde_json::from_str(&read_testdata_to_string("github_get_pr.json")).unwrap();
        });
    }

    #[test]
    fn created_issue_comment() {
        run_test(|| {
            info!("created_issue_comment test");
            let _ic: github::IssueComment = serde_json::from_str(&read_testdata_to_string(
                "github_created_issue_comment.json",
            ))
            .unwrap();
        });
    }
}
