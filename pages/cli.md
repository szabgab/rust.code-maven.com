---
title: CLI - Command Line Interface in Rust
timestamp: 2024-01-05T08:30:01
author: szabgab
published: true
description: Command Line Applications or just tools that run on the command line.
tags:
    - cli
---

Rust provides access to the values on the command line via the [std::env::args](https://doc.rust-lang.org/std/env/fn.args.html) function. We can use this in its raw form:

* [ARGV - command line parameters for simple CLI program](/argv-simple-command-line-parameters)
* [Expect one command line parameter in Rust](/expect-one-command-line-parameter)

There are also crates that provide a much nicer and more comprehensive interface to parse the content of `args` that were provided on the command line. We cover:

* [Clap](/clap) - Command Line Argument Parser for Rust


We also maintain a list of [Open Source Applications](/applications) with links to many command line applications written in Rust.
