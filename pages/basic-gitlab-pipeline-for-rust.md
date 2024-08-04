---
title: Basic GitLab pipeline to setup CI for a Rust project
timestamp: 2024-08-03T11:30:01
author: szabgab
published: true
description: When using GitLab it is very easy to setup Continuous Integration using the GitLab pipelines.
tags:
    - GitLab
    - CI
todo:
    - more examples
---

Just as we can use [GitHub Actions](/default-github-workflow-for-rust-on-linux) to provide Continuous Integration for a rust project
hosted on GitHub, we can use the [GitLab pipelines](https://docs.gitlab.com/ee/ci/) to the same for projects hosted on GitLab.

according to the [VCS report](https://rust-digger.code-maven.com/vcs/) of [Rust Digger](https://rust-digger.code-maven.com/) 2.45% of all the
crates are hosted on [GitLab.com](https://gitlab.com/), and there are a few hundred other project that are hosted on the GitLab platform elsewhere.

In order to setup a pipeline you need to have a "runner" that will run a Docker container in which the code will run. GitLab.com provides runners
free of charge for a limited number of minutes every minute. If you run GitLab on your own server you might also need to set up a Virtual Private Server
to be your runner.

Once you have a runner all you need to do is add a configuration file called `.gitlab-ci.yml` to the root of your git repository.

This is the first example I have that you can find in [this project](https://gitlab.com/szabgab/gitlab-pipeline-for-rust/). The results can be found under [jobs](https://gitlab.com/szabgab/gitlab-pipeline-for-rust/-/jobs).


{% include file="examples/gitlab-simple/.gitlab-ci.yml" %}

First we define the Docker image we'll use to run the tests in. You can pick any image from the [Docker Hub](https://hub.docker.com/). In this example I used the "latest"
that already contains [rust](https://hub.docker.com/_/rust).

Then I have a number of commands in the `before_script` section that help me see exactly which version of Rust we are using and that also installs clippy and rustfmt.

It is customary to put all the configuration commands in the `before_script` section so if something fails it will be easier for us to see if the failure happened
while setting up the environment or in the actual tests and verifications.

In the `script` section we have 3 commands that are running some simple verifications of the code and all the tests that come with the crate.

One can, of course, do a lot more with the GitLab pipelines, but setting this up will already be a good start.

Rust Digger reports that 1.23% of all the crates are on GitLab without a pipeline. That is, more than half of the Rust projects hosted on GitLab do NOT have a pipeline yet.
See the [stats](https://rust-digger.code-maven.com/stats) and the list of [crates on GitLab without a pipeline](https://rust-digger.code-maven.com/gitlab-but-no-ci).


## Other

See also the [example in the Cargo Guide](https://doc.rust-lang.org/cargo/guide/continuous-integration.html#gitlab-ci).
