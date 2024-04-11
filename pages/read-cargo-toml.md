---
title: Read and parse the Cargo.toml manifes file of a Rust crate
timestamp: 2024-04-01T13:30:01
author: szabgab
published: true
description: How to read the Cargo.toml file of a Crate?
tags:
    - Cargo.toml
    - toml
todo:
    - TODO
---

As part of the [Rust Digger](https://rust-digger.code-maven.com/) project I wanted to read the Cargo.toml file that comes with the project.
At first I tried the [read_manifes](https://docs.rs/cargo/latest/cargo/util/toml/fn.read_manifest.html) method of the
[Cargo](https://crates.io/crates/cargo) crate, but I could not figure out how to use it.

Luckily then I encountered the [cargo_toml](https://crates.io/crates/cargo_toml) crate that provides a much easier way to get started.

## Dependencies

{% include file="examples/read-cargo-toml/Cargo.toml" %}

## The code:

{% include file="examples/read-cargo-toml/src/main.rs" %}

My current interests are the `edition` and the `rust-version` fields as I am working on the
[issue of MSRV](https://github.com/szabgab/rust-digger/issues/53) (Minimum Supported Rust Version) indication.


Two observations:

* The `Edition` field will default to 2015.
* IF there are invalid fields they are disregarded.



## The various sample input files:

{% include file="examples/read-cargo-toml/Cargo_only_edition.toml" %}

{% include file="examples/read-cargo-toml/Cargo_only_version.toml" %}

{% include file="examples/read-cargo-toml/Cargo_both_edition_and_version.toml" %}

{% include file="examples/read-cargo-toml/Cargo_neither.toml" %}

{% include file="examples/read-cargo-toml/Cargo_invalid_field.toml" %}


## The output:

```
Cargo_only_edition.toml
No rust-version
edition 2021
------
Cargo_only_version.toml
1.77
edition 2015
------
Cargo_both_edition_and_version.toml
1.77
edition 2021
------
Cargo_neither.toml
No rust-version
edition 2015
------

Manifest {
    package: Some(
        Package {
            name: "read-cargo-toml",
            edition: Set(
                E2021,
            ),
            rust_version: None,
            version: Set(
                "0.1.0",
            ),
            build: None,
            workspace: None,
            authors: Set(
                [],
            ),
            links: None,
            description: None,
            homepage: None,
            documentation: None,
            readme: Set(
                Flag(
                    true,
                ),
            ),
            keywords: Set(
                [],
            ),
            categories: Set(
                [],
            ),
            exclude: Set(
                [],
            ),
            include: Set(
                [],
            ),
            license: None,
            license_file: None,
            repository: None,
            default_run: None,
            autobins: true,
            autoexamples: true,
            autotests: true,
            autobenches: true,
            publish: Set(
                Flag(
                    true,
                ),
            ),
            resolver: None,
            metadata: None,
        },
    ),
    workspace: None,
    dependencies: {
        "cargo_toml": Simple(
            "0.19.2",
        ),
    },
    dev_dependencies: {},
    build_dependencies: {},
    target: {},
    features: {},
    replace: {},
    patch: {},
    lib: None,
    profile: Profiles {
        release: None,
        dev: None,
        test: None,
        bench: None,
        doc: None,
        custom: {},
    },
    badges: Badges {
        appveyor: None,
        circle_ci: None,
        gitlab: None,
        travis_ci: None,
        codecov: None,
        coveralls: None,
        is_it_maintained_issue_resolution: None,
        is_it_maintained_open_issues: None,
        maintenance: Maintenance {
            status: None,
        },
    },
    bin: [],
    bench: [],
    test: [],
    example: [],
    lints: None,
}

```
