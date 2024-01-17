---
title: Web development in Rust
timestamp: 2023-12-20T11:00:01
author: szabgab
published: true
description: Web application development tools and frameworks in Rust
tags:
    - web
todo:
    - experiment with each one of them
---

A collection of various web frameworks and other web-related tools in Rust.  The ones I have started to review are in **bold**.

Based on the replies to the question I asked on Reddit: [Which web framework do you use in Rust?](https://www.reddit.com/r/rust/comments/18ogwtl/which_web_framework_do_you_use_in_rust/)
I can say that most of these were mentioned, but I think axum and actix got the most comments.

I will keep updating this page as I find more Crates that are strongly related to web development. You can suggest more thing in issues or Pull-Request!

## Backend Web frameworks

* **[Tiny HTTP](/tiny-http)** is a very low-level library. I've used it for the [Rustatic](https://rustatic.code-maven.com/) project, and you can also experiement with it, but you'll probably find better alternatives.
* **[Rouille](/rouille)** is a web micro-framework.
* **[Loco](/loco)** as in Locomotive is a Rails-inspired framework.
* **[Rocket](/rocket)** is a bit like Pyhton Flask, or Perl Dancer.

* [axum](https://crates.io/crates/axum)
* [actix](https://crates.io/crates/actix-web) - [Actix](https://actix.rs/)
* [Tungstenite](https://crates.io/crates/tungstenite) - Lightweight stream-based WebSocket implementation.
* [warp](https://crates.io/crates/warp)
* [Gotham](https://crates.io/crates/gotham) - [Gotham](https://gotham.rs/)
* [salvo](https://crates.io/crates/salvo) - [Salvo](https://salvo.rs/)
* [picoserve](https://crates.io/crates/picoserve) - An async no_std HTTP server suitable for bare-metal environments.
* [ntex](https://crates.io/crates/ntex) Framework for composable network services.
* [poem](https://crates.io/crates/poem)
* [viz](https://crates.io/crates/viz) - [viz](https://viz.rs/)
* [dioxus](https://crates.io/crates/dioxus) - [Dioxus](https://dioxuslabs.com/)

## Frontend web tools

* [Stdweb](https://crates.io/crates/stdweb)
* [Yew](https://crates.io/crates/yew) - [Yew](https://yew.rs/)


## Full stack

* [Leptos](https://crates.io/crates/leptos) - [Leptos](https://www.leptos.dev/) - Full stack

## Templating systems

* **[liquid](/slides/rust/liquid)** - I liked this, I could understand how to use it, though there were some issues.
* **[Handlebars](/slides/rust/handlebars)** - I found it too complex for my understanding.
* [askama](https://crates.io/crates/askama) - like Python jinja
* [Tera](https://crates.io/crates/tera) used with [Rocket](/rocket)

* [maud](https://crates.io/crates/maud)  [maud](https://maud.lambda.xyz/) Compile-time HTML templates. A macro for writing HTML.

## Other

* [htmx](https://crates.io/crates/htmx) See also [htmx](https://htmx.org/)
* [routerify](https://crates.io/crates/routerify)
* [okapi](https://crates.io/crates/okapi)


## Projects

* [axum-tutorial](https://github.com/GeauxEric/axum-tutorial) and educational project serving static files. [video](https://www.youtube.com/watch?v=WR0N9rfNC-Y) explaining it.
* [Rustatic - Rust statci server](https://rustatic.code-maven.com/) serving static files using Tiny HTTP


## Source

Some of the above were collected from these posts, but they have more information.

* [Exploring the top Rust web frameworks](https://blog.logrocket.com/top-rust-web-frameworks/) lists several other options with some level of analyzis of them.
* [Rust web framework comparison](https://github.com/flosse/rust-web-framework-comparison)


