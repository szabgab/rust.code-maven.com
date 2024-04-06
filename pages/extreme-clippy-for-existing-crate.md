---
title: Extreme Clippy for existing Crate
timestamp: 2024-04-06T14:30:01
author: szabgab
published: true
description: Enable every clippy lint and then turn off the ones that are violated.
tags:
    - clippy
---

See how we [get started with Extreme Clippy on a new Crate](/extreme-clippy).

{% youtube id="jO-oQH_gK4M" %}

Enabled all the lints in **Cargo.toml**:

```
[lints.clippy]
cargo        = { priority = -1, level = "deny" }
complexity   = { priority = -1, level = "deny" }
correctness  = { priority = -1, level = "deny" }
nursery      = { priority = -1, level = "deny" }
pedantic     = { priority = -1, level = "deny" }
perf         = { priority = -1, level = "deny" }
restriction  = { priority = -1, level = "deny" }
style        = { priority = -1, level = "deny" }
suspicious   = { priority = -1, level = "deny" }
```


I had to run this command 3 time to disable all the lints that were violated by the [Rust Digger](https://rust-digger.code-maven.com/) code-base that can be found [here](https://github.com/szabgab/rust-digger).

```
cargo clippy 2>&1 | grep https://rust-lang.github.io/rust-clippy/master/index.html | sed 's/^.*#//' | sort | uniq | xargs -I % echo '% = "allow"' >> Cargo.toml
```

