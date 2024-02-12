---
title: Clap - Command Line Argument Parser for Rust
timestamp: 2023-12-24T08:30:01
author: szabgab
published: true
description: Clap is a simple to use, efficient, and full-featured Command Line Argument Parser for Rustlang.
tags:
    - CLI
    - args
todo:
    - subcommand the same way we need for the code-maven-web and code-maven-sendgrid that have different options
    - mutually exclusive parameters?
    - positional argument
    - optional values
    - accepting more than one values
---

[Clap](https://crates.io/crates/clap) is a simple to use, efficient, and full-featured Command Line Argument Parser for Rust.

* [Getting started with Clap a single required parameter](/clap-simple) - `arg`, `long`, `derive`, `struct`, `--help`
* [Clap - show version number of the command line application in Rust](/clap-show-version-number) - `version`, `command`
* [Clap - accept string, integer, floating point numbers, booleans, and more on the command line](/clap-strings-numbers-float) - `String`, `i32`, `f32`, `bool`, `PathBuf`
* [Clap - default values for CLI command line parameters in Rust](/clap-default-values) - `default_value`, `default_value_t`
* [Clap - short command line parameters](/clap-short) - `short`
* [Clap - Add help text for each command line parameter in Rust](/clap-help-text) - `help`
* [Clap - positional command line arguments in Rust](/clap-positional-arguments) - `required`, `default_value`
* [Clap - subcommands](/clap-subcommand) - `Subcommand`
* [Clap - Showing the description in the help using the about command](/clap-about-the-application) - `about`, `description`.
* [How to include sha1 from git in the version report using derive interface of Clap?](/clap-include-sha1-from-git-in-the-version-report-using-derive-interface-of-clap) - `version`, `about`, `help`.

Documentation of the [derive interface](https://docs.rs/clap/latest/clap/_derive/).
