---
title: "Clippy: Multiple crate versions"
timestamp: 2026-04-18T09:30:01
author: szabgab
published: true
show_related: true
description: Should we invest energy avoiding the same crate twice as our dependency?
tags:
    - Clippy
---

Adding

```toml
[lints.clippy]
cargo        = { priority = -1, level = "deny" }
```

to `Cargo.toml` will also turn on the [multiple_crate_versions](https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#multiple_crate_versions) rule.

It will them complain if the same create was added more than once (with different version numbers) to `Cargo.lock`.

## How could this happen?


## What to do about it?

1. We can decide to disregard this rule. Add the following line to `Cargo.toml`

```
[lints.clippy]
multiple_crate_versions = "allow"
```

2. Tell Clippy about the specific crates for which we allow the duplication:

Create the `clippy.toml` file and add the following entry, with the names of the crates we allow to be duplicated:

```toml
allowed-duplicate-crates = [
    "async-channel",
    "event-listener",
    "foldhash",
    "hashbrown",
    "hashlink",
    "wit-bindgen",
]
```

See [allowed-duplicate-crates](https://doc.rust-lang.org/nightly/clippy/lint_configuration.html#allowed-duplicate-crates).


3. Figure out which of our dependencies brought in these dependencies and either replace our own dependencies, or send a Pull-Request to our dependencies to allow for more flexibility.



