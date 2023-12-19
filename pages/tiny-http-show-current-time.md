---
title: "Tiny HTTP: generate dynamic response, show the current time ⏰"
timestamp: 2023-12-15T08:30:01
published: true
description: Returning the same static content every time the user visits is boring. Generating dynamic content is a bit more interesting.
tags:
    - tiny_http
    - web
    - chrono
    - UTC
    - Local
    - date
    - time
    - HTML
---

Part of the [Tiny HTTP](/tiny-http) series. Previously we saw how to [show static HTML content](/tiny-http-hello-world) now let's go a tiny step further and show how to
display dynamic content. Every time the user access the page we'll show the current time as it is available on the server.

For this we are going to use the [chrono](https://crates.io/crates/chrono) crate.

First let's see the dependencies in the `Cargo.toml` file:

![](examples/tiny-http/show-time/Cargo.toml)


## The new code

First we load the tools we are going to use:

```
use chrono::{DateTime, Local, Utc};
```

Then we fetch the UTC time and the localtime. Finally we create an HTML string.

```rust
let utc: DateTime<Utc> = Utc::now();
let local: DateTime<Local> = Local::now();
let html = format!("<table><tr><td>UTC: </td><td>{}</td></tr><tr><td>Localtime: </td><td>{}</td></tr></table>", utc, local);
```

## The Full source code

![](examples/tiny-http/show-time/src/main.rs)



