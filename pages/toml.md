---
title: Toml
timestamp: 2024-04-11T19:45:01
author: szabgab
published: true
description:
tags:
    - toml
todo:
    - Use structs to parse the toml files
---

TOML stands for [Tom's Obvious Minimal Language](https://toml.io/en/). It is an alternative to INI files and [YAML](/yaml) and [JSON](/json) files.

The [toml](https://crates.io/crates/toml) crate allows us to read TOML files.

Every Rust Crate comes with a file called `Cargo.toml` which, surprise, surprise, is in this format. So you, a Rust developer, are probably quite familiar with this format.


* [Read rustfmt.toml TOML file](/read-rustfmt-toml). A simple example.
* [Read and parse the Cargo.toml manifes file of a Rust crate](/read-cargo-toml) using the `cargo_toml` crate.
* [Read and parse Cargo.toml using the toml crate defining our own structs](/read-and-parse-cargo-toml) using the `toml` crate and `serde`.
