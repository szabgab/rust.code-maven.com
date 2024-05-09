---
title: Rocket - web development with Rust
timestamp: 2023-12-22T12:30:01
author: szabgab
published: true
description: Examples with the Rocket web framework for Rustlang
tags:
    - Rocket
    - web
todo:
    - Hello World! with Rocket, tests in a separate file.
    - Set `content-type` `text/html`
    - Accept dynamic pathes (path parameters)
    - Accept GET parameter
    - Accept POST parameter
    - Cookies
    - Deployment
---

[Rocket](https://rocket.rs/) is one of the web development frameworks for the Rust programming language. It has a nice web site that comes with a nice
tutorial, but for me, to undertand it I needed my own, stand-alone examples. On this page you'll find the list of examples. You can get the full source
code of each one of them from [Rust Maven GitHub repo](https://github.com/szabgab/rust.code-maven.com/) or by following the links above the files.

[Rocket on Crates](https://crates.io/crates/rocket).


* [Hello World! with Rocket](/rocket-hello-world) - `text/plain`
* [Hello World with tests in separate file](/rocket-hello-world-separating-tests) - `test`.
* [Hello World! in HTML](/rocket-hello-world-html) - `RawHtml`, `text/html`, `headers`, `get_one`.
* [Hello World with Tera Templates](/rocket-hello-world-tera-template) - `rocket_dyn_templates`, `Template`, `context!`, `render`, `attach`, `fairing`.
* [Echo using HTTP POST - form handling](/rocket-echo-post) - `POST`, `Form`, `Template`, `header`, `body`, `ContentType::Form`, `Status::UnprocessableEntity`, `422`.
* [Echo using HTTP GET](/rocket-echo-get) = `GET`.
* [Single counter in a plain text file](/rocket-single-counter-in-text-file).
* [Multi-counter using cookies](/rocket-multi-counter-using-cookies) - `CookieJar`, `add`, `get`.
* [Multi counter using encrypted cookies](/rocket-multi-counter-using-encrypted-cookies) - `add_private`, `get_private`, `secret_key`,`Rocket.toml`, `private_cookie`.
* [Liniting issues with Rocket](/rocket-linting) - Clippy
* [Logging in a Rocket-based web application](/rocket-logging) - `log`, `debug!`, `info!`, `warn!`,  `error!`.
* [Access custom configuration in the routes](/rocket-access-custom-configuration) - `rocket::Config`, `figment`, `extract_inner`.
* [Get, set (add), delete cookies - pending cookies](/rocket-set-get-delete-cookies) - `get`, `add`, `delete`, `get_pending`, `CookieJar`.
* [Process Query String of a GET request](/rocket-query-string)
* [Early return from routes](/rocket-early-return)
* [404 page with static content](/rocket-404-page-with-static-content) - `404`, `content`, `RawHtml`, `routes!`, `catchers!`, `mount`, `register`, `include_str!`.
* [In memory hit counter using state](/rocket-in-memory-hit-counter-state) - `AtomicUsize`, `Ordering`, `State`, `manage`.

## Rocket-based Open Source projects

* The [Pastebin tutorial](https://rocket.rs/guide/v0.5/pastebin/#pastebin-tutorial)
* The list of [showcases](https://rocket.rs/guide/v0.5/faq/#showcase).


* [Linked List - A personal knowledge base](https://linkedlist.org/) - [source](https://github.com/wezm/pkb)
* [Dew Point weather Forecast](https://dewpoint.7bit.org/) - [source](https://github.com/wezm/dewpoint.7bit.org)
* [Quotes between friends using UI I designed in 2003](https://quotes.randome.net/) - [source](https://github.com/wezm/Quotes)
* [Tempfiles](https://tempfiles.ninja/) A simple file sharing service that encrypts files at rest and deletes them automatically after 24 hours. - [source](https://github.com/olback/tempfiles-rs)


* A lichess clone of SHOBU powered by Rust. (in development) [source](https://github.com/Kapsyloffer/RUSTBU)

