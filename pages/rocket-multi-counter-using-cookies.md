---
title: Rocket - multi-counter using cookies
timestamp: 2024-01-03T14:30:01
published: false
description:
tags:
    - Rust
todo:
    - TODO
---

This is a [Counter example](https://code-maven.com/counter), part of the series about the [Rocket web development framework](/rocket) of Rust.

Unlike the [Single counter in text file](/rocket-single-counter-in-text-file) example, here we don't use a file on the server to store the counter.
Instead we store the counter a **cookie in the browser**. This means that each brower that access this page will have its own counter.


## Dependencies

For this example to work we only need the Rocket crate:

![](examples/rocket/multi-counter-using-cookies/Cargo.toml)


## The code

![](examples/rocket/multi-counter-using-cookies/src/main.rs)


We use the [rocket::http::CookieJar](https://api.rocket.rs/v0.5/rocket/http/struct.CookieJar.html).


We only have one route and there we expect a parameter `cookies: &CookieJar<'_>`. This tells Rocket to parse the header of the request and
extract the cookie from the header into the `cookies` variable.


## The tests

![](examples/rocket/multi-counter-using-cookies/src/tests.rs)

[cookie](https://api.rocket.rs/v0.5/rocket/local/blocking/struct.LocalRequest.html#method.cookie)
