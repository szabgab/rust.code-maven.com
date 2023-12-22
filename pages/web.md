---
title: Web development in Rust
timestamp: 2023-12-20T11:00:01
published: true
description: Web application development tools and frameworks in Rust
tags:
    - web
todo:
    - experiment with each one of them
---

This is basically a placeholder page where I am going to collect the various web-related tools in Rust that I have reviewed. (in **bold**).


## Backend Web frameworks

* **[Tiny HTTP](/tiny-http)** is a very low-level library. I've used it for the [Rustatic](https://rustatic.code-maven.com/) project, and you can also experiement with it, but you'll probably find better alternatives.
* **[Rouille](/rouille)** is a web micro-framework.
* **[Loco](/loco)** (as in Locomotive)
* **[Rocket](/rocket)**

* [axum](https://crates.io/crates/axum)
* [actix](https://crates.io/crates/actix-web) - [Actix](https://actix.rs/)
* [Tungstenite](https://crates.io/crates/tungstenite) - Lightweight stream-based WebSocket implementation.
* [warp](https://crates.io/crates/warp)
* [Gotham](https://crates.io/crates/gotham) - [Gotham](https://gotham.rs/)


## Frontend web tools

* [Stdweb](https://crates.io/crates/stdweb)
* [Yew](https://crates.io/crates/yew) - [Yew](https://yew.rs/)


## Full stack

* [Leptos](https://crates.io/crates/leptos) - [Leptos](https://www.leptos.dev/) - Full stack

## Templating systems

* **[liquid](/slides/rust/liquid)** - I liked this, I could understand how to use it, though there were some issues.
* **[Handlebars](/slides/rust/handlebars)** - I found it too complex for my understanding.

## Source

Some of the above were collected from these posts, but they have more information.

* [Exploring the top Rust web frameworks](https://blog.logrocket.com/top-rust-web-frameworks/) lists several other options with some level of analyzis of them.
* [Rust web framework comparison](https://github.com/flosse/rust-web-framework-comparison)
