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

Probably the simplests version would be creating a folder called `.github/workflows` in your repository and saving the following file
as `ci.yml`. (The actual name does not matter, but the extension has to be either `yml` or `yaml`.


{% include file="examples/github-workflows/default-ubuntu.yml" %}

This will run every time you push out code and every time someone sends a Pull-Request.
Actually, as a precaution, if someone sends a Pull-Request you need to approve the job for them. Only once you accepted a pull-request from a person only then will this job start automatically on any subsequent PRs from the same person.

This will set up a job on an Ubuntu Linux machine. See the [options for other runners](https://docs.github.com/en/actions/writing-workflows/workflow-syntax-for-github-actions#standard-github-hosted-runners-for-public-repositories).

It will then check out your repository to to this machine using the [checkout action](https://github.com/actions/checkout/).

Then we execute two command to show the version of `rusup` and that of `rustc`. We only do this so later we can look at the output and see where did our process run.

Then we have 3 cargo commands to check the code in various ways. You can change these as you like and/or you can add more such commands.


I maintain a [repository with several more examples](https://github.com/szabgab/github-actions-rust).


