---
title: Rocket liniting issues
timestamp: 2024-01-18T18:10:01
author: szabgab
published: true
description: Demonstrating and dealing with linting issues in web applications written using Rust Rocket.
tags:
    - Rocket
    - Clippy
    - lint
---

Recently I started to use Clippy extensively and also applied it to my [Rocket](/rocket)-based projects.

On this page I am going to collected the linting issues I've encountered and hope to also provide answers.


![](examples/rocket/lint/Cargo.toml)

![](examples/rocket/lint/src/main.rs)


```
cargo clippy -- -Dclippy::no_effect_underscore_binding
```

complains:

```

    Checking lint v0.1.0 (/home/gabor/work/rust.code-maven.com/lint)
error: binding to `_` prefixed variable with no side-effect
  --> src/main.rs:15:17
   |
15 |                 hello_world
   |                 ^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#no_effect_underscore_binding
   = note: requested on the command line with `-D clippy::no-effect-underscore-binding`

error: binding to `_` prefixed variable with no side-effect
  --> src/main.rs:15:17
   |
15 |                 hello_world
   |                 ^^^^^^^^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#no_effect_underscore_binding

error: binding to `_` prefixed variable with no side-effect
  --> src/main.rs:10:16
   |
10 | fn rocket() -> _ {
   |                ^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#no_effect_underscore_binding

error: could not compile `lint` (bin "lint") due to 3 previous errors
```

Reported as [no_effect_underscore_binding false positive for Rocket routes](https://github.com/rust-lang/rust-clippy/issues/12166)




