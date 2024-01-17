---
title: Rocket - Hello World returning HTML
timestamp: 2024-01-02T16:25:01
author: szabgab
published: true
description: Set the content-type to text/html
tags:
    - content::RawHtml
    - Content-Type
    - headers
    - get_one
---

Part of the series about the [Rocket web framework](/rocket).


## Dependencies

![](examples/rocket/hello-world-html/Cargo.toml)


## Code

![](examples/rocket/hello-world-html/src/main.rs)

The key is that the route function is defined to return `content::RawHtml<&'static str>`
and then we call `content::RawHtml` to return the HTML.

This will set the `Content-type` to `text/html`.


## Testing

We can verify it in our test using the `response.headers().get_one("Content-Type")` call and running

```
cargo test
```

![](examples/rocket/hello-world-html/src/tests.rs)


## Running

We can also launch the web service using

```
cargo run
```

And verify the output in our browser or by using `curl` in a separate terminal:

```
$ curl -i http://localhost:8000

HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
server: Rocket
permissions-policy: interest-cohort=()
x-content-type-options: nosniff
x-frame-options: SAMEORIGIN
content-length: 20
date: Tue, 02 Jan 2024 14:11:04 GMT

Hello, <b>world!</b>
```


