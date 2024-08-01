---
title: Default GitHub Action Workflow for Rust project running on Ubuntu Linux
timestamp: 2024-08-01T12:00:01
author: szabgab
published: true
description: You don't need much to set up Continuous Integration for your Rust project hosted on GitHub Actions
tags:
    - GitHub Actions
    - GitHub Workflow
todo:
    - pages with the other examples
---

We really don't need to do much to set up Continuous Integration (CI) for our Rust project if it is hosted on GitHub.
Accroding to the [stats](https://rust-digger.code-maven.com/stats) if the [Rust Digger](https://rust-digger.code-maven.com/)
approximately 75.96% of the crates use GitHub, but 38.83% are [on GitHub, but don't have CI](https://rust-digger.code-maven.com/github-but-no-ci).
It's a pity as it is very easy to set it up.


Probably the simplests version would be creating a folder called `.github/workflows` in your repository and saving the following file
as `ci.yml`. (The actual name does not matter, but the extension has to be either `yml` or `yaml`.


{% include file="examples/github-workflows/default-ubuntu.yml" %}

This will run every time you push out code and every time someone sends a Pull-Request.
Actually, as a precaution, if someone sends a Pull-Request you need to approve the job for them. Only once you accepted a pull-request from a person only then will this job start automatically on any subsequent PRs from the same person.

This will set up a job on an Ubuntu Linux machine. See the [options for other runners](https://docs.github.com/en/actions/writing-workflows/workflow-syntax-for-github-actions#standard-github-hosted-runners-for-public-repositories).

It will then check out your repository to to this machine using the [checkout action](https://github.com/actions/checkout/).


## Checking the version

Then we execute two command to show the version of `rusup` and that of `rustc`. We only do this so later we can look at the output and see where did our process run.
As in this simple setup we did not configure the version of Rust, it can be especially important to check which version did we get.

Here are the results on 2024.08.01 when I am publishing this:

```
rustup 1.27.1 (54dd3d00f 2024-04-24)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.79.0 (129f3b996 2024-06-10)`
```

and

```
Run rustc -vV
rustc 1.79.0 (129f3b996 2024-06-10)
binary: rustc
commit-hash: 129f3b9964af4d4a709d1383930ade12dfe7c081
commit-date: 2024-06-10
host: x86_64-unknown-linux-gnu
release: 1.79.0
LLVM version: 18.1.7
```

## Testing

Then we have 3 cargo commands to check the code in various ways. You can change these as you like and/or you can add more such commands.


I maintain a [repository with several more examples](https://github.com/szabgab/github-actions-rust).


