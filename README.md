[![Build Status](https://travis-ci.org/brndnmtthws/labhub.svg?branch=master)](https://travis-ci.org/brndnmtthws/labhub) [![Coverage Status](https://coveralls.io/repos/github/brndnmtthws/labhub/badge.svg)](https://coveralls.io/github/brndnmtthws/labhub)

# LabHub 🤖

A GitHub bot written in Rust for using GitLab CI in OSS projects.

## The Problem

GitLab has a great CI system, however it's not suitable for open source projects 😧 (at the time of writing) because it won't build external PRs by default.

For more details on the issue, [please take a look at this GitLab discussion](https://gitlab.com/gitlab-org/gitlab-ee/issues/5667).

## The Solution

LabHub listens for webhooks from GitHub to notify for new pull requests. If the PR is from a forked repo, it will push a branch to GitLab (for the corresponding PR) to test the build.

## In Action

LabHub is currently being used by the [Conky](https://github.com/brndnmtthws/conky) project.

## Configuration

Configuration is accomplished using environment variables. For details, see [src/config.rs](src/config.rs). An example configuration might look like this:

```shell
# A pair of repo mappings from GitHub to GitLab. Format is `github1=gitlab1,github2=gitlab2,...`
export LABHUB_MAPPINGS=brndnmtthws/labhub=brndnmtthws-oss/labhub,brndnmtthws/conky=brndnmtthws-oss/conky,brndnmtthws/labhub-test=brndnmtthws-oss/labhub-test

# GitHub settings
export GITHUB_WEBHOOK_SECRET=secret
export GITHUB_USERNAME=conky-ci
export GITHUB_SSH_KEY=/home/conky-ci/.ssh/labhub-key.ecdsa

# GitLab settings
export GITLAB_WEBHOOK_SECRET=secret
export GITLAB_USERNAME=conky-ci
export GITLAB_SSH_KEY=/home/conky-ci/.ssh/labhub-key.ecdsa

# Logger settings
export RUST_LOG=labhub=info
```

## Deployment

### Setup Webhooks

You'll need to set up webhooks for any repo you wish to enable LabHub for. Currently, only GitHub webhooks are required. To get started, go to `github.com/<org>/<repo>/settings/hooks` and add a new webhook.

Configure the webhook to send PR and push events. Set the payload URL such that it ends in `/github/events`, which is the path LabHub is expecting. Create a secrete (ex: `cat /dev/urandom | LC_CTYPE=C tr -dc 'a-zA-Z0-9' | fold -w 32 | head -n 1`) and set the same value in the webhook config as in LabHub.

Example:

<a href="docs/github-webhook-config.png"><img src="docs/github-webhook-config.png" alt="GitHub webhook config" height="400px"></a>

### Create SSH keys

You'll need a CI user with SSH keys for both GitHub and GitLab. Create an account on both sites (if you don't already have a CI user), and create an SSH key for LabHub:

```ShellSession
$ ssh-keygen -f labhub-key.ecdsa -t ecdsa -b 521
```

Keep `labhub-key.ecdsa` safe, and upload `labhub-key.ecdsa.pub` to both GitHub and GitLab for the CI user.

### Deploy with Helm

There's a Helm chart included in this repo, which is the preferred method of deployment. To use you, you must first create the SSH key secrets with kubectl. Assuming your SSH private key is `labhub-key.ecdsa`:

```ShellSession
$ kubectl create secret generic ssh-keys --from-file=github=labhub-key.ecdsa --from-file=gitlab=labhub-key.ecdsa
```

You may use separate keys for GitHub and GitLab if you choose, respectively.

Once you have the secrets, install the helm chart from [helm/labhub/](helm/labhub/):

```ShellSession
$ cd helm/labhub/
$ cp values.yaml myvalues.yaml
# Edit myvalues.yaml to your liking
$ helm upgrade --install labhub . -f myvalues.yaml
```
