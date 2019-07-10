// This file is auto-generated, do not edit.

#[derive(Serialize, Deserialize, Debug)]
pub struct Ping {
    pub zen: Option<String>,
    pub hook_id: Option<i64>,
    pub hook: Option<PingHook>,
    pub repository: Option<PingRepository>,
    pub sender: Option<PingSender>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PingHook {
    #[serde(rename = "type")]
    pub type_key: Option<String>,
    pub id: Option<i64>,
    pub name: Option<String>,
    pub active: Option<bool>,
    pub events: Option<Vec<String>>,
    pub config: Option<PingHookConfig>,
    pub updated_at: Option<serde_json::value::Value>,
    pub created_at: Option<serde_json::value::Value>,
    pub url: Option<String>,
    pub test_url: Option<String>,
    pub ping_url: Option<String>,
    pub last_response: Option<PingHookLastResponse>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PingHookConfig {
    pub content_type: Option<String>,
    pub insecure_ssl: Option<String>,
    pub secret: Option<String>,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PingHookLastResponse {
    pub code: Option<String>,
    pub status: Option<String>,
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PingRepository {
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub name: Option<String>,
    pub full_name: Option<String>,
    pub private: Option<bool>,
    pub owner: Option<PingRepositoryOwner>,
    pub html_url: Option<String>,
    pub description: Option<String>,
    pub fork: Option<bool>,
    pub url: Option<String>,
    pub forks_url: Option<String>,
    pub keys_url: Option<String>,
    pub collaborators_url: Option<String>,
    pub teams_url: Option<String>,
    pub hooks_url: Option<String>,
    pub issue_events_url: Option<String>,
    pub events_url: Option<String>,
    pub assignees_url: Option<String>,
    pub branches_url: Option<String>,
    pub tags_url: Option<String>,
    pub blobs_url: Option<String>,
    pub git_tags_url: Option<String>,
    pub git_refs_url: Option<String>,
    pub trees_url: Option<String>,
    pub statuses_url: Option<String>,
    pub languages_url: Option<String>,
    pub stargazers_url: Option<String>,
    pub contributors_url: Option<String>,
    pub subscribers_url: Option<String>,
    pub subscription_url: Option<String>,
    pub commits_url: Option<String>,
    pub git_commits_url: Option<String>,
    pub comments_url: Option<String>,
    pub issue_comment_url: Option<String>,
    pub contents_url: Option<String>,
    pub compare_url: Option<String>,
    pub merges_url: Option<String>,
    pub archive_url: Option<String>,
    pub downloads_url: Option<String>,
    pub issues_url: Option<String>,
    pub pulls_url: Option<String>,
    pub milestones_url: Option<String>,
    pub notifications_url: Option<String>,
    pub labels_url: Option<String>,
    pub releases_url: Option<String>,
    pub deployments_url: Option<String>,
    pub created_at: Option<serde_json::value::Value>,
    pub updated_at: Option<serde_json::value::Value>,
    pub pushed_at: Option<serde_json::value::Value>,
    pub git_url: Option<String>,
    pub ssh_url: Option<String>,
    pub clone_url: Option<String>,
    pub svn_url: Option<String>,
    pub homepage: Option<String>,
    pub size: Option<i64>,
    pub stargazers_count: Option<i64>,
    pub watchers_count: Option<i64>,
    pub language: Option<String>,
    pub has_issues: Option<bool>,
    pub has_projects: Option<bool>,
    pub has_downloads: Option<bool>,
    pub has_wiki: Option<bool>,
    pub has_pages: Option<bool>,
    pub forks_count: Option<i64>,
    pub mirror_url: Option<String>,
    pub archived: Option<bool>,
    pub open_issues_count: Option<i64>,
    pub license: Option<PingRepositoryLicense>,
    pub forks: Option<i64>,
    pub open_issues: Option<i64>,
    pub watchers: Option<i64>,
    pub default_branch: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PingRepositoryOwner {
    pub login: Option<String>,
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<String>,
    pub organizations_url: Option<String>,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub type_key: Option<String>,
    pub site_admin: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PingRepositoryLicense {
    pub key: Option<String>,
    pub name: Option<String>,
    pub spdx_id: Option<String>,
    pub url: Option<String>,
    pub node_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PingSender {
    pub login: Option<String>,
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<String>,
    pub organizations_url: Option<String>,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub type_key: Option<String>,
    pub site_admin: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Push {
    #[serde(rename = "ref")]
    pub ref_key: Option<String>,
    pub before: Option<String>,
    pub after: Option<String>,
    pub created: Option<bool>,
    pub deleted: Option<bool>,
    pub forced: Option<bool>,
    pub base_ref: Option<String>,
    pub compare: Option<String>,
    pub commits: Option<Vec<PushCommitsItem>>,
    pub head_commit: Option<PushHeadCommit>,
    pub repository: Option<PushRepository>,
    pub pusher: Option<PushPusher>,
    pub sender: Option<PushSender>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PushCommitsItem {
    pub id: Option<String>,
    pub tree_id: Option<String>,
    pub distinct: Option<bool>,
    pub message: Option<String>,
    pub timestamp: Option<String>,
    pub url: Option<String>,
    pub author: Option<PushCommitsItemAuthor>,
    pub committer: Option<PushCommitsItemCommitter>,
    pub modified: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PushCommitsItemAuthor {
    pub name: Option<String>,
    pub email: Option<String>,
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PushCommitsItemCommitter {
    pub name: Option<String>,
    pub email: Option<String>,
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PushHeadCommit {
    pub id: Option<String>,
    pub tree_id: Option<String>,
    pub distinct: Option<bool>,
    pub message: Option<String>,
    pub timestamp: Option<String>,
    pub url: Option<String>,
    pub author: Option<PushHeadCommitAuthor>,
    pub committer: Option<PushHeadCommitCommitter>,
    pub modified: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PushHeadCommitAuthor {
    pub name: Option<String>,
    pub email: Option<String>,
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PushHeadCommitCommitter {
    pub name: Option<String>,
    pub email: Option<String>,
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PushRepository {
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub name: Option<String>,
    pub full_name: Option<String>,
    pub private: Option<bool>,
    pub owner: Option<PushRepositoryOwner>,
    pub html_url: Option<String>,
    pub description: Option<String>,
    pub fork: Option<bool>,
    pub url: Option<String>,
    pub forks_url: Option<String>,
    pub keys_url: Option<String>,
    pub collaborators_url: Option<String>,
    pub teams_url: Option<String>,
    pub hooks_url: Option<String>,
    pub issue_events_url: Option<String>,
    pub events_url: Option<String>,
    pub assignees_url: Option<String>,
    pub branches_url: Option<String>,
    pub tags_url: Option<String>,
    pub blobs_url: Option<String>,
    pub git_tags_url: Option<String>,
    pub git_refs_url: Option<String>,
    pub trees_url: Option<String>,
    pub statuses_url: Option<String>,
    pub languages_url: Option<String>,
    pub stargazers_url: Option<String>,
    pub contributors_url: Option<String>,
    pub subscribers_url: Option<String>,
    pub subscription_url: Option<String>,
    pub commits_url: Option<String>,
    pub git_commits_url: Option<String>,
    pub comments_url: Option<String>,
    pub issue_comment_url: Option<String>,
    pub contents_url: Option<String>,
    pub compare_url: Option<String>,
    pub merges_url: Option<String>,
    pub archive_url: Option<String>,
    pub downloads_url: Option<String>,
    pub issues_url: Option<String>,
    pub pulls_url: Option<String>,
    pub milestones_url: Option<String>,
    pub notifications_url: Option<String>,
    pub labels_url: Option<String>,
    pub releases_url: Option<String>,
    pub deployments_url: Option<String>,
    pub created_at: Option<serde_json::value::Value>,
    pub updated_at: Option<serde_json::value::Value>,
    pub pushed_at: Option<serde_json::value::Value>,
    pub git_url: Option<String>,
    pub ssh_url: Option<String>,
    pub clone_url: Option<String>,
    pub svn_url: Option<String>,
    pub homepage: Option<String>,
    pub size: Option<i64>,
    pub stargazers_count: Option<i64>,
    pub watchers_count: Option<i64>,
    pub language: Option<String>,
    pub has_issues: Option<bool>,
    pub has_projects: Option<bool>,
    pub has_downloads: Option<bool>,
    pub has_wiki: Option<bool>,
    pub has_pages: Option<bool>,
    pub forks_count: Option<i64>,
    pub mirror_url: Option<String>,
    pub archived: Option<bool>,
    pub open_issues_count: Option<i64>,
    pub license: Option<PushRepositoryLicense>,
    pub forks: Option<i64>,
    pub open_issues: Option<i64>,
    pub watchers: Option<i64>,
    pub default_branch: Option<String>,
    pub stargazers: Option<i64>,
    pub master_branch: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PushRepositoryOwner {
    pub name: Option<String>,
    pub email: Option<String>,
    pub login: Option<String>,
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<String>,
    pub organizations_url: Option<String>,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub type_key: Option<String>,
    pub site_admin: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PushRepositoryLicense {
    pub key: Option<String>,
    pub name: Option<String>,
    pub spdx_id: Option<String>,
    pub url: Option<String>,
    pub node_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PushPusher {
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PushSender {
    pub login: Option<String>,
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<String>,
    pub organizations_url: Option<String>,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub type_key: Option<String>,
    pub site_admin: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequest {
    pub action: Option<String>,
    pub number: Option<i64>,
    pub pull_request: Option<PullRequestPullRequest>,
    pub repository: Option<PullRequestRepository>,
    pub sender: Option<PullRequestSender>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequestPullRequest {
    pub url: Option<String>,
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub html_url: Option<String>,
    pub diff_url: Option<String>,
    pub patch_url: Option<String>,
    pub issue_url: Option<String>,
    pub number: Option<i64>,
    pub state: Option<String>,
    pub locked: Option<bool>,
    pub title: Option<String>,
    pub user: Option<PullRequestPullRequestUser>,
    pub body: Option<String>,
    pub created_at: Option<serde_json::value::Value>,
    pub updated_at: Option<serde_json::value::Value>,
    pub closed_at: Option<serde_json::value::Value>,
    pub merged_at: Option<String>,
    pub merge_commit_sha: Option<String>,
    pub assignee: Option<String>,
    pub milestone: Option<PullRequestPullRequestMilestone>,
    pub commits_url: Option<String>,
    pub review_comments_url: Option<String>,
    pub review_comment_url: Option<String>,
    pub comments_url: Option<String>,
    pub statuses_url: Option<String>,
    pub head: Option<PullRequestPullRequestHead>,
    pub base: Option<PullRequestPullRequestBase>,
    pub _links: Option<PullRequestPullRequestLinks>,
    pub author_association: Option<String>,
    pub draft: Option<bool>,
    pub merged: Option<bool>,
    pub mergeable: Option<bool>,
    pub rebaseable: Option<bool>,
    pub mergeable_state: Option<String>,
    pub merged_by: Option<PullRequestPullRequestMergedBy>,
    pub comments: Option<i64>,
    pub review_comments: Option<i64>,
    pub maintainer_can_modify: Option<bool>,
    pub commits: Option<i64>,
    pub additions: Option<i64>,
    pub deletions: Option<i64>,
    pub changed_files: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequestPullRequestUser {
    pub login: Option<String>,
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<String>,
    pub organizations_url: Option<String>,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub type_key: Option<String>,
    pub site_admin: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequestPullRequestMilestone {
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub labels_url: Option<String>,
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub number: Option<i64>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub creator: Option<PullRequestPullRequestMilestoneCreator>,
    pub open_issues: Option<i64>,
    pub closed_issues: Option<i64>,
    pub state: Option<String>,
    pub created_at: Option<serde_json::value::Value>,
    pub updated_at: Option<serde_json::value::Value>,
    pub due_on: Option<serde_json::value::Value>,
    pub closed_at: Option<serde_json::value::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequestPullRequestMilestoneCreator {
    pub login: Option<String>,
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<String>,
    pub organizations_url: Option<String>,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub type_key: Option<String>,
    pub site_admin: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequestPullRequestHead {
    pub label: Option<String>,
    #[serde(rename = "ref")]
    pub ref_key: Option<String>,
    pub sha: Option<String>,
    pub user: Option<PullRequestPullRequestHeadUser>,
    pub repo: Option<PullRequestPullRequestHeadRepo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequestPullRequestHeadUser {
    pub login: Option<String>,
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<String>,
    pub organizations_url: Option<String>,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub type_key: Option<String>,
    pub site_admin: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequestPullRequestHeadRepo {
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub name: Option<String>,
    pub full_name: Option<String>,
    pub private: Option<bool>,
    pub owner: Option<PullRequestPullRequestHeadRepoOwner>,
    pub html_url: Option<String>,
    pub description: Option<String>,
    pub fork: Option<bool>,
    pub url: Option<String>,
    pub forks_url: Option<String>,
    pub keys_url: Option<String>,
    pub collaborators_url: Option<String>,
    pub teams_url: Option<String>,
    pub hooks_url: Option<String>,
    pub issue_events_url: Option<String>,
    pub events_url: Option<String>,
    pub assignees_url: Option<String>,
    pub branches_url: Option<String>,
    pub tags_url: Option<String>,
    pub blobs_url: Option<String>,
    pub git_tags_url: Option<String>,
    pub git_refs_url: Option<String>,
    pub trees_url: Option<String>,
    pub statuses_url: Option<String>,
    pub languages_url: Option<String>,
    pub stargazers_url: Option<String>,
    pub contributors_url: Option<String>,
    pub subscribers_url: Option<String>,
    pub subscription_url: Option<String>,
    pub commits_url: Option<String>,
    pub git_commits_url: Option<String>,
    pub comments_url: Option<String>,
    pub issue_comment_url: Option<String>,
    pub contents_url: Option<String>,
    pub compare_url: Option<String>,
    pub merges_url: Option<String>,
    pub archive_url: Option<String>,
    pub downloads_url: Option<String>,
    pub issues_url: Option<String>,
    pub pulls_url: Option<String>,
    pub milestones_url: Option<String>,
    pub notifications_url: Option<String>,
    pub labels_url: Option<String>,
    pub releases_url: Option<String>,
    pub deployments_url: Option<String>,
    pub created_at: Option<serde_json::value::Value>,
    pub updated_at: Option<serde_json::value::Value>,
    pub pushed_at: Option<serde_json::value::Value>,
    pub git_url: Option<String>,
    pub ssh_url: Option<String>,
    pub clone_url: Option<String>,
    pub svn_url: Option<String>,
    pub homepage: Option<String>,
    pub size: Option<i64>,
    pub stargazers_count: Option<i64>,
    pub watchers_count: Option<i64>,
    pub language: Option<String>,
    pub has_issues: Option<bool>,
    pub has_projects: Option<bool>,
    pub has_downloads: Option<bool>,
    pub has_wiki: Option<bool>,
    pub has_pages: Option<bool>,
    pub forks_count: Option<i64>,
    pub mirror_url: Option<String>,
    pub archived: Option<bool>,
    pub open_issues_count: Option<i64>,
    pub license: Option<PullRequestPullRequestHeadRepoLicense>,
    pub forks: Option<i64>,
    pub open_issues: Option<i64>,
    pub watchers: Option<i64>,
    pub default_branch: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequestPullRequestHeadRepoOwner {
    pub login: Option<String>,
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<String>,
    pub organizations_url: Option<String>,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub type_key: Option<String>,
    pub site_admin: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequestPullRequestHeadRepoLicense {
    pub key: Option<String>,
    pub name: Option<String>,
    pub spdx_id: Option<String>,
    pub url: Option<String>,
    pub node_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequestPullRequestBase {
    pub label: Option<String>,
    #[serde(rename = "ref")]
    pub ref_key: Option<String>,
    pub sha: Option<String>,
    pub user: Option<PullRequestPullRequestBaseUser>,
    pub repo: Option<PullRequestPullRequestBaseRepo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequestPullRequestBaseUser {
    pub login: Option<String>,
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<String>,
    pub organizations_url: Option<String>,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub type_key: Option<String>,
    pub site_admin: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequestPullRequestBaseRepo {
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub name: Option<String>,
    pub full_name: Option<String>,
    pub private: Option<bool>,
    pub owner: Option<PullRequestPullRequestBaseRepoOwner>,
    pub html_url: Option<String>,
    pub description: Option<String>,
    pub fork: Option<bool>,
    pub url: Option<String>,
    pub forks_url: Option<String>,
    pub keys_url: Option<String>,
    pub collaborators_url: Option<String>,
    pub teams_url: Option<String>,
    pub hooks_url: Option<String>,
    pub issue_events_url: Option<String>,
    pub events_url: Option<String>,
    pub assignees_url: Option<String>,
    pub branches_url: Option<String>,
    pub tags_url: Option<String>,
    pub blobs_url: Option<String>,
    pub git_tags_url: Option<String>,
    pub git_refs_url: Option<String>,
    pub trees_url: Option<String>,
    pub statuses_url: Option<String>,
    pub languages_url: Option<String>,
    pub stargazers_url: Option<String>,
    pub contributors_url: Option<String>,
    pub subscribers_url: Option<String>,
    pub subscription_url: Option<String>,
    pub commits_url: Option<String>,
    pub git_commits_url: Option<String>,
    pub comments_url: Option<String>,
    pub issue_comment_url: Option<String>,
    pub contents_url: Option<String>,
    pub compare_url: Option<String>,
    pub merges_url: Option<String>,
    pub archive_url: Option<String>,
    pub downloads_url: Option<String>,
    pub issues_url: Option<String>,
    pub pulls_url: Option<String>,
    pub milestones_url: Option<String>,
    pub notifications_url: Option<String>,
    pub labels_url: Option<String>,
    pub releases_url: Option<String>,
    pub deployments_url: Option<String>,
    pub created_at: Option<serde_json::value::Value>,
    pub updated_at: Option<serde_json::value::Value>,
    pub pushed_at: Option<serde_json::value::Value>,
    pub git_url: Option<String>,
    pub ssh_url: Option<String>,
    pub clone_url: Option<String>,
    pub svn_url: Option<String>,
    pub homepage: Option<String>,
    pub size: Option<i64>,
    pub stargazers_count: Option<i64>,
    pub watchers_count: Option<i64>,
    pub language: Option<String>,
    pub has_issues: Option<bool>,
    pub has_projects: Option<bool>,
    pub has_downloads: Option<bool>,
    pub has_wiki: Option<bool>,
    pub has_pages: Option<bool>,
    pub forks_count: Option<i64>,
    pub mirror_url: Option<String>,
    pub archived: Option<bool>,
    pub open_issues_count: Option<i64>,
    pub license: Option<PullRequestPullRequestBaseRepoLicense>,
    pub forks: Option<i64>,
    pub open_issues: Option<i64>,
    pub watchers: Option<i64>,
    pub default_branch: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequestPullRequestBaseRepoOwner {
    pub login: Option<String>,
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<String>,
    pub organizations_url: Option<String>,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub type_key: Option<String>,
    pub site_admin: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequestPullRequestBaseRepoLicense {
    pub key: Option<String>,
    pub name: Option<String>,
    pub spdx_id: Option<String>,
    pub url: Option<String>,
    pub node_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequestPullRequestLinks {
    #[serde(rename = "self")]
    pub self_key: Option<PullRequestPullRequestLinksSelfKey>,
    pub html: Option<PullRequestPullRequestLinksHtml>,
    pub issue: Option<PullRequestPullRequestLinksIssue>,
    pub comments: Option<PullRequestPullRequestLinksComments>,
    pub review_comments: Option<PullRequestPullRequestLinksReviewComments>,
    pub review_comment: Option<PullRequestPullRequestLinksReviewComment>,
    pub commits: Option<PullRequestPullRequestLinksCommits>,
    pub statuses: Option<PullRequestPullRequestLinksStatuses>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequestPullRequestLinksSelfKey {
    pub href: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequestPullRequestLinksHtml {
    pub href: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequestPullRequestLinksIssue {
    pub href: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequestPullRequestLinksComments {
    pub href: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequestPullRequestLinksReviewComments {
    pub href: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequestPullRequestLinksReviewComment {
    pub href: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequestPullRequestLinksCommits {
    pub href: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequestPullRequestLinksStatuses {
    pub href: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequestPullRequestMergedBy {
    pub login: Option<String>,
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<String>,
    pub organizations_url: Option<String>,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub type_key: Option<String>,
    pub site_admin: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequestRepository {
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub name: Option<String>,
    pub full_name: Option<String>,
    pub private: Option<bool>,
    pub owner: Option<PullRequestRepositoryOwner>,
    pub html_url: Option<String>,
    pub description: Option<String>,
    pub fork: Option<bool>,
    pub url: Option<String>,
    pub forks_url: Option<String>,
    pub keys_url: Option<String>,
    pub collaborators_url: Option<String>,
    pub teams_url: Option<String>,
    pub hooks_url: Option<String>,
    pub issue_events_url: Option<String>,
    pub events_url: Option<String>,
    pub assignees_url: Option<String>,
    pub branches_url: Option<String>,
    pub tags_url: Option<String>,
    pub blobs_url: Option<String>,
    pub git_tags_url: Option<String>,
    pub git_refs_url: Option<String>,
    pub trees_url: Option<String>,
    pub statuses_url: Option<String>,
    pub languages_url: Option<String>,
    pub stargazers_url: Option<String>,
    pub contributors_url: Option<String>,
    pub subscribers_url: Option<String>,
    pub subscription_url: Option<String>,
    pub commits_url: Option<String>,
    pub git_commits_url: Option<String>,
    pub comments_url: Option<String>,
    pub issue_comment_url: Option<String>,
    pub contents_url: Option<String>,
    pub compare_url: Option<String>,
    pub merges_url: Option<String>,
    pub archive_url: Option<String>,
    pub downloads_url: Option<String>,
    pub issues_url: Option<String>,
    pub pulls_url: Option<String>,
    pub milestones_url: Option<String>,
    pub notifications_url: Option<String>,
    pub labels_url: Option<String>,
    pub releases_url: Option<String>,
    pub deployments_url: Option<String>,
    pub created_at: Option<serde_json::value::Value>,
    pub updated_at: Option<serde_json::value::Value>,
    pub pushed_at: Option<serde_json::value::Value>,
    pub git_url: Option<String>,
    pub ssh_url: Option<String>,
    pub clone_url: Option<String>,
    pub svn_url: Option<String>,
    pub homepage: Option<String>,
    pub size: Option<i64>,
    pub stargazers_count: Option<i64>,
    pub watchers_count: Option<i64>,
    pub language: Option<String>,
    pub has_issues: Option<bool>,
    pub has_projects: Option<bool>,
    pub has_downloads: Option<bool>,
    pub has_wiki: Option<bool>,
    pub has_pages: Option<bool>,
    pub forks_count: Option<i64>,
    pub mirror_url: Option<String>,
    pub archived: Option<bool>,
    pub open_issues_count: Option<i64>,
    pub license: Option<PullRequestRepositoryLicense>,
    pub forks: Option<i64>,
    pub open_issues: Option<i64>,
    pub watchers: Option<i64>,
    pub default_branch: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequestRepositoryOwner {
    pub login: Option<String>,
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<String>,
    pub organizations_url: Option<String>,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub type_key: Option<String>,
    pub site_admin: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequestRepositoryLicense {
    pub key: Option<String>,
    pub name: Option<String>,
    pub spdx_id: Option<String>,
    pub url: Option<String>,
    pub node_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequestSender {
    pub login: Option<String>,
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<String>,
    pub organizations_url: Option<String>,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub type_key: Option<String>,
    pub site_admin: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueComment {
    pub action: Option<String>,
    pub issue: Option<IssueCommentIssue>,
    pub comment: Option<IssueCommentComment>,
    pub repository: Option<IssueCommentRepository>,
    pub sender: Option<IssueCommentSender>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueCommentIssue {
    pub url: Option<String>,
    pub repository_url: Option<String>,
    pub labels_url: Option<String>,
    pub comments_url: Option<String>,
    pub events_url: Option<String>,
    pub html_url: Option<String>,
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub number: Option<i64>,
    pub title: Option<String>,
    pub user: Option<IssueCommentIssueUser>,
    pub state: Option<String>,
    pub locked: Option<bool>,
    pub assignee: Option<String>,
    pub milestone: Option<String>,
    pub comments: Option<i64>,
    pub created_at: Option<serde_json::value::Value>,
    pub updated_at: Option<serde_json::value::Value>,
    pub closed_at: Option<serde_json::value::Value>,
    pub author_association: Option<String>,
    pub pull_request: Option<IssueCommentIssuePullRequest>,
    pub body: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueCommentIssueUser {
    pub login: Option<String>,
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<String>,
    pub organizations_url: Option<String>,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub type_key: Option<String>,
    pub site_admin: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueCommentIssuePullRequest {
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub diff_url: Option<String>,
    pub patch_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueCommentComment {
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub issue_url: Option<String>,
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub user: Option<IssueCommentCommentUser>,
    pub created_at: Option<serde_json::value::Value>,
    pub updated_at: Option<serde_json::value::Value>,
    pub author_association: Option<String>,
    pub body: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueCommentCommentUser {
    pub login: Option<String>,
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<String>,
    pub organizations_url: Option<String>,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub type_key: Option<String>,
    pub site_admin: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueCommentRepository {
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub name: Option<String>,
    pub full_name: Option<String>,
    pub private: Option<bool>,
    pub owner: Option<IssueCommentRepositoryOwner>,
    pub html_url: Option<String>,
    pub description: Option<String>,
    pub fork: Option<bool>,
    pub url: Option<String>,
    pub forks_url: Option<String>,
    pub keys_url: Option<String>,
    pub collaborators_url: Option<String>,
    pub teams_url: Option<String>,
    pub hooks_url: Option<String>,
    pub issue_events_url: Option<String>,
    pub events_url: Option<String>,
    pub assignees_url: Option<String>,
    pub branches_url: Option<String>,
    pub tags_url: Option<String>,
    pub blobs_url: Option<String>,
    pub git_tags_url: Option<String>,
    pub git_refs_url: Option<String>,
    pub trees_url: Option<String>,
    pub statuses_url: Option<String>,
    pub languages_url: Option<String>,
    pub stargazers_url: Option<String>,
    pub contributors_url: Option<String>,
    pub subscribers_url: Option<String>,
    pub subscription_url: Option<String>,
    pub commits_url: Option<String>,
    pub git_commits_url: Option<String>,
    pub comments_url: Option<String>,
    pub issue_comment_url: Option<String>,
    pub contents_url: Option<String>,
    pub compare_url: Option<String>,
    pub merges_url: Option<String>,
    pub archive_url: Option<String>,
    pub downloads_url: Option<String>,
    pub issues_url: Option<String>,
    pub pulls_url: Option<String>,
    pub milestones_url: Option<String>,
    pub notifications_url: Option<String>,
    pub labels_url: Option<String>,
    pub releases_url: Option<String>,
    pub deployments_url: Option<String>,
    pub created_at: Option<serde_json::value::Value>,
    pub updated_at: Option<serde_json::value::Value>,
    pub pushed_at: Option<serde_json::value::Value>,
    pub git_url: Option<String>,
    pub ssh_url: Option<String>,
    pub clone_url: Option<String>,
    pub svn_url: Option<String>,
    pub homepage: Option<String>,
    pub size: Option<i64>,
    pub stargazers_count: Option<i64>,
    pub watchers_count: Option<i64>,
    pub language: Option<String>,
    pub has_issues: Option<bool>,
    pub has_projects: Option<bool>,
    pub has_downloads: Option<bool>,
    pub has_wiki: Option<bool>,
    pub has_pages: Option<bool>,
    pub forks_count: Option<i64>,
    pub mirror_url: Option<String>,
    pub archived: Option<bool>,
    pub open_issues_count: Option<i64>,
    pub license: Option<IssueCommentRepositoryLicense>,
    pub forks: Option<i64>,
    pub open_issues: Option<i64>,
    pub watchers: Option<i64>,
    pub default_branch: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueCommentRepositoryOwner {
    pub login: Option<String>,
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<String>,
    pub organizations_url: Option<String>,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub type_key: Option<String>,
    pub site_admin: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueCommentRepositoryLicense {
    pub key: Option<String>,
    pub name: Option<String>,
    pub spdx_id: Option<String>,
    pub url: Option<String>,
    pub node_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueCommentSender {
    pub login: Option<String>,
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<String>,
    pub organizations_url: Option<String>,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub type_key: Option<String>,
    pub site_admin: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoPr {
    pub url: Option<String>,
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub html_url: Option<String>,
    pub diff_url: Option<String>,
    pub patch_url: Option<String>,
    pub issue_url: Option<String>,
    pub commits_url: Option<String>,
    pub review_comments_url: Option<String>,
    pub review_comment_url: Option<String>,
    pub comments_url: Option<String>,
    pub statuses_url: Option<String>,
    pub number: Option<i64>,
    pub state: Option<String>,
    pub locked: Option<bool>,
    pub title: Option<String>,
    pub user: Option<RepoPrUser>,
    pub body: Option<String>,
    pub labels: Option<Vec<RepoPrLabelsItem>>,
    pub milestone: Option<RepoPrMilestone>,
    pub active_lock_reason: Option<String>,
    pub created_at: Option<serde_json::value::Value>,
    pub updated_at: Option<serde_json::value::Value>,
    pub closed_at: Option<serde_json::value::Value>,
    pub merged_at: Option<String>,
    pub merge_commit_sha: Option<String>,
    pub assignee: Option<RepoPrAssignee>,
    pub assignees: Option<Vec<RepoPrAssigneesItem>>,
    pub requested_reviewers: Option<Vec<RepoPrRequestedReviewersItem>>,
    pub requested_teams: Option<Vec<RepoPrRequestedTeamsItem>>,
    pub head: Option<RepoPrHead>,
    pub base: Option<RepoPrBase>,
    pub _links: Option<RepoPrLinks>,
    pub author_association: Option<String>,
    pub draft: Option<bool>,
    pub merged: Option<bool>,
    pub mergeable: Option<bool>,
    pub rebaseable: Option<bool>,
    pub mergeable_state: Option<String>,
    pub merged_by: Option<RepoPrMergedBy>,
    pub comments: Option<i64>,
    pub review_comments: Option<i64>,
    pub maintainer_can_modify: Option<bool>,
    pub commits: Option<i64>,
    pub additions: Option<i64>,
    pub deletions: Option<i64>,
    pub changed_files: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoPrUser {
    pub login: Option<String>,
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<String>,
    pub organizations_url: Option<String>,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub type_key: Option<String>,
    pub site_admin: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoPrLabelsItem {
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
    pub default: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoPrMilestone {
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub labels_url: Option<String>,
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub number: Option<i64>,
    pub state: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub creator: Option<RepoPrMilestoneCreator>,
    pub open_issues: Option<i64>,
    pub closed_issues: Option<i64>,
    pub created_at: Option<serde_json::value::Value>,
    pub updated_at: Option<serde_json::value::Value>,
    pub closed_at: Option<serde_json::value::Value>,
    pub due_on: Option<serde_json::value::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoPrMilestoneCreator {
    pub login: Option<String>,
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<String>,
    pub organizations_url: Option<String>,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub type_key: Option<String>,
    pub site_admin: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoPrAssignee {
    pub login: Option<String>,
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<String>,
    pub organizations_url: Option<String>,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub type_key: Option<String>,
    pub site_admin: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoPrAssigneesItem {
    pub login: Option<String>,
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<String>,
    pub organizations_url: Option<String>,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub type_key: Option<String>,
    pub site_admin: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoPrRequestedReviewersItem {
    pub login: Option<String>,
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<String>,
    pub organizations_url: Option<String>,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub type_key: Option<String>,
    pub site_admin: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoPrRequestedTeamsItem {
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub privacy: Option<String>,
    pub permission: Option<String>,
    pub members_url: Option<String>,
    pub repositories_url: Option<String>,
    pub parent: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoPrHead {
    pub label: Option<String>,
    #[serde(rename = "ref")]
    pub ref_key: Option<String>,
    pub sha: Option<String>,
    pub user: Option<RepoPrHeadUser>,
    pub repo: Option<RepoPrHeadRepo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoPrHeadUser {
    pub login: Option<String>,
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<String>,
    pub organizations_url: Option<String>,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub type_key: Option<String>,
    pub site_admin: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoPrHeadRepo {
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub name: Option<String>,
    pub full_name: Option<String>,
    pub owner: Option<RepoPrHeadRepoOwner>,
    pub private: Option<bool>,
    pub html_url: Option<String>,
    pub description: Option<String>,
    pub fork: Option<bool>,
    pub url: Option<String>,
    pub archive_url: Option<String>,
    pub assignees_url: Option<String>,
    pub blobs_url: Option<String>,
    pub branches_url: Option<String>,
    pub collaborators_url: Option<String>,
    pub comments_url: Option<String>,
    pub commits_url: Option<String>,
    pub compare_url: Option<String>,
    pub contents_url: Option<String>,
    pub contributors_url: Option<String>,
    pub deployments_url: Option<String>,
    pub downloads_url: Option<String>,
    pub events_url: Option<String>,
    pub forks_url: Option<String>,
    pub git_commits_url: Option<String>,
    pub git_refs_url: Option<String>,
    pub git_tags_url: Option<String>,
    pub git_url: Option<String>,
    pub issue_comment_url: Option<String>,
    pub issue_events_url: Option<String>,
    pub issues_url: Option<String>,
    pub keys_url: Option<String>,
    pub labels_url: Option<String>,
    pub languages_url: Option<String>,
    pub merges_url: Option<String>,
    pub milestones_url: Option<String>,
    pub notifications_url: Option<String>,
    pub pulls_url: Option<String>,
    pub releases_url: Option<String>,
    pub ssh_url: Option<String>,
    pub stargazers_url: Option<String>,
    pub statuses_url: Option<String>,
    pub subscribers_url: Option<String>,
    pub subscription_url: Option<String>,
    pub tags_url: Option<String>,
    pub teams_url: Option<String>,
    pub trees_url: Option<String>,
    pub clone_url: Option<String>,
    pub mirror_url: Option<String>,
    pub hooks_url: Option<String>,
    pub svn_url: Option<String>,
    pub homepage: Option<String>,
    pub language: Option<String>,
    pub forks_count: Option<i64>,
    pub stargazers_count: Option<i64>,
    pub watchers_count: Option<i64>,
    pub size: Option<i64>,
    pub default_branch: Option<String>,
    pub open_issues_count: Option<i64>,
    pub topics: Option<Vec<String>>,
    pub has_issues: Option<bool>,
    pub has_projects: Option<bool>,
    pub has_wiki: Option<bool>,
    pub has_pages: Option<bool>,
    pub has_downloads: Option<bool>,
    pub archived: Option<bool>,
    pub pushed_at: Option<serde_json::value::Value>,
    pub created_at: Option<serde_json::value::Value>,
    pub updated_at: Option<serde_json::value::Value>,
    pub permissions: Option<RepoPrHeadRepoPermissions>,
    pub allow_rebase_merge: Option<bool>,
    pub allow_squash_merge: Option<bool>,
    pub allow_merge_commit: Option<bool>,
    pub subscribers_count: Option<i64>,
    pub network_count: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoPrHeadRepoOwner {
    pub login: Option<String>,
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<String>,
    pub organizations_url: Option<String>,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub type_key: Option<String>,
    pub site_admin: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoPrHeadRepoPermissions {
    pub admin: Option<bool>,
    pub push: Option<bool>,
    pub pull: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoPrBase {
    pub label: Option<String>,
    #[serde(rename = "ref")]
    pub ref_key: Option<String>,
    pub sha: Option<String>,
    pub user: Option<RepoPrBaseUser>,
    pub repo: Option<RepoPrBaseRepo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoPrBaseUser {
    pub login: Option<String>,
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<String>,
    pub organizations_url: Option<String>,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub type_key: Option<String>,
    pub site_admin: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoPrBaseRepo {
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub name: Option<String>,
    pub full_name: Option<String>,
    pub owner: Option<RepoPrBaseRepoOwner>,
    pub private: Option<bool>,
    pub html_url: Option<String>,
    pub description: Option<String>,
    pub fork: Option<bool>,
    pub url: Option<String>,
    pub archive_url: Option<String>,
    pub assignees_url: Option<String>,
    pub blobs_url: Option<String>,
    pub branches_url: Option<String>,
    pub collaborators_url: Option<String>,
    pub comments_url: Option<String>,
    pub commits_url: Option<String>,
    pub compare_url: Option<String>,
    pub contents_url: Option<String>,
    pub contributors_url: Option<String>,
    pub deployments_url: Option<String>,
    pub downloads_url: Option<String>,
    pub events_url: Option<String>,
    pub forks_url: Option<String>,
    pub git_commits_url: Option<String>,
    pub git_refs_url: Option<String>,
    pub git_tags_url: Option<String>,
    pub git_url: Option<String>,
    pub issue_comment_url: Option<String>,
    pub issue_events_url: Option<String>,
    pub issues_url: Option<String>,
    pub keys_url: Option<String>,
    pub labels_url: Option<String>,
    pub languages_url: Option<String>,
    pub merges_url: Option<String>,
    pub milestones_url: Option<String>,
    pub notifications_url: Option<String>,
    pub pulls_url: Option<String>,
    pub releases_url: Option<String>,
    pub ssh_url: Option<String>,
    pub stargazers_url: Option<String>,
    pub statuses_url: Option<String>,
    pub subscribers_url: Option<String>,
    pub subscription_url: Option<String>,
    pub tags_url: Option<String>,
    pub teams_url: Option<String>,
    pub trees_url: Option<String>,
    pub clone_url: Option<String>,
    pub mirror_url: Option<String>,
    pub hooks_url: Option<String>,
    pub svn_url: Option<String>,
    pub homepage: Option<String>,
    pub language: Option<String>,
    pub forks_count: Option<i64>,
    pub stargazers_count: Option<i64>,
    pub watchers_count: Option<i64>,
    pub size: Option<i64>,
    pub default_branch: Option<String>,
    pub open_issues_count: Option<i64>,
    pub topics: Option<Vec<String>>,
    pub has_issues: Option<bool>,
    pub has_projects: Option<bool>,
    pub has_wiki: Option<bool>,
    pub has_pages: Option<bool>,
    pub has_downloads: Option<bool>,
    pub archived: Option<bool>,
    pub pushed_at: Option<serde_json::value::Value>,
    pub created_at: Option<serde_json::value::Value>,
    pub updated_at: Option<serde_json::value::Value>,
    pub permissions: Option<RepoPrBaseRepoPermissions>,
    pub allow_rebase_merge: Option<bool>,
    pub allow_squash_merge: Option<bool>,
    pub allow_merge_commit: Option<bool>,
    pub subscribers_count: Option<i64>,
    pub network_count: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoPrBaseRepoOwner {
    pub login: Option<String>,
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<String>,
    pub organizations_url: Option<String>,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub type_key: Option<String>,
    pub site_admin: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoPrBaseRepoPermissions {
    pub admin: Option<bool>,
    pub push: Option<bool>,
    pub pull: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoPrLinks {
    #[serde(rename = "self")]
    pub self_key: Option<RepoPrLinksSelfKey>,
    pub html: Option<RepoPrLinksHtml>,
    pub issue: Option<RepoPrLinksIssue>,
    pub comments: Option<RepoPrLinksComments>,
    pub review_comments: Option<RepoPrLinksReviewComments>,
    pub review_comment: Option<RepoPrLinksReviewComment>,
    pub commits: Option<RepoPrLinksCommits>,
    pub statuses: Option<RepoPrLinksStatuses>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoPrLinksSelfKey {
    pub href: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoPrLinksHtml {
    pub href: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoPrLinksIssue {
    pub href: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoPrLinksComments {
    pub href: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoPrLinksReviewComments {
    pub href: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoPrLinksReviewComment {
    pub href: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoPrLinksCommits {
    pub href: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoPrLinksStatuses {
    pub href: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoPrMergedBy {
    pub login: Option<String>,
    pub id: Option<i64>,
    pub node_id: Option<String>,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<String>,
    pub organizations_url: Option<String>,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub type_key: Option<String>,
    pub site_admin: Option<bool>,
}
