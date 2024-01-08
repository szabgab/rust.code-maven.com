---
title: Rocket - multi counter using secure cookies
timestamp: 2024-01-03T14:30:01
published: false
description:
tags:
    - Rust
todo:
    - TODO
---


* Include the `secrets` feature.
* Set the `secret_key` in the `Rocket.toml` file. See [explanation about the secret_key](https://rocket.rs/v0.5/guide/requests/#secret-key) and the about the [Rocket configuration](https://rocket.rs/v0.5/guide/configuration/) for more options.
* Use `get_private` and `add_private` instead of `get` and `add`.

![](examples/rocket/multi-counter-using-secure-cookies/Cargo.toml)
![](examples/rocket/multi-counter-using-secure-cookies/src/main.rs)
![](examples/rocket/multi-counter-using-secure-cookies/src/tests.rs)
![](https://rocket.rs/v0.5/guide/requests/#secret-key)

