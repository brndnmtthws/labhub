use crate::api::models::github;
use crate::config;
use crate::errors::{GitError, RequestErrorResult};

use git2::build::RepoBuilder;
use git2::{FetchOptions, PushOptions, RemoteCallbacks, Repository};
use log::{debug, error, info};
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

fn get_remote_callbacks<'a>() -> RemoteCallbacks<'a> {
    let mut remote_callbacks = RemoteCallbacks::new();
    remote_callbacks.credentials(|_user, _user_from_url, cred| {
        debug!("Entered Git credential callback, cred={:?}", cred);
        if cred.contains(git2::CredentialType::USERNAME) {
            git2::Cred::username(&"git".to_string())
        } else {
            let github_ssh_key = config::GITHUB_SSH_KEY.to_string();
            let path = Path::new(&github_ssh_key);
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

        let gitlab_url = format!(
            "git@gitlab.com:{}.git",
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
        fetch_options.remote_callbacks(get_remote_callbacks());

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
        push_options.remote_callbacks(get_remote_callbacks());

        let refspec = format!(
            "refs/heads/pr-{}/{}/{}:refs/heads/pr-{}/{}/{}",
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
        push_options.remote_callbacks(get_remote_callbacks());

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
    fetch_options.remote_callbacks(get_remote_callbacks());

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
    repo: &mut RepositoryExt,
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
    repo: &mut RepositoryExt,
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

fn handle_pr(pr: github::PullRequest) -> Result<(), RequestErrorResult> {
    if pr
        .pull_request
        .as_ref()?
        .head
        .as_ref()?
        .repo
        .as_ref()?
        .fork?
    {
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

pub fn handle_event_body(event_type: &str, body: &str) -> Result<String, RequestErrorResult> {
    match event_type {
        "push" => {
            let push: github::Push = serde_json::from_str(body)?;
            info!("Push ref={}", push.ref_key.as_ref()?);
            Ok(String::from("Push received 😘"))
        }
        "pull_request" => {
            let pr: github::PullRequest = serde_json::from_str(body)?;
            info!("PullRequest action={}", pr.action.as_ref()?);
            thread::spawn(move || handle_pr(pr));
            Ok(String::from("Thanks buddy bro 😍"))
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
            let _pr_handle = PrHandle::new(&pr).unwrap();
        });
    }
}
