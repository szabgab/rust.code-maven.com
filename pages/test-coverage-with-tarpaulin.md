---
title: Test coverage with Tarpaulin
timestamp: 2025-04-08T09:30:01
author: szabgab
published: true
show_related: true
description:
tags:
    - tests
---

[cargo-tarpaulin](https://crates.io/crates/cargo-tarpaulin) is a code coverage reporting tool for the Cargo build system, named for a waterproof cloth used to cover cargo on a ship.


## Install Tarpaulin

```
cargo install cargo-tarpaulin
```


## Run Tarpaulin generate HTML report

```
cargo tarpaulin --out Html
```

## Run Tarpaulin - print minimalistic text report on the screen

```
cargo tarpaulin --out Stdout
```

