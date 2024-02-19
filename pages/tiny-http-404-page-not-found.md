---
title: Tiny HTTP - Setting status to 404 page not found
timestamp: 2023-12-20T10:22:01
author: szabgab
published: true
description: It is not enough to display the text page not found, one also has to set the HTTP status code returned by the server to 404.
tags:
    - tiny_http
    - web
    - with_status_code
    - status code
    - 404
---

Part of the [Tiny HTTP series](/tiny-http).

When a user arrives to a page that does not exist we would like to display some nice text, but we should also send the famous **404 Not Found** HTTP status code.

This is how we set the status code to any number:

```rust
request
    .respond(
        Response::from_string(html)
            .with_status_code(StatusCode::from(status_code))
            .with_header(header),
    )
    .unwrap();
```

We have a function, boringly called `default` that would return the "nice" HTML page and the status code.

```rust
fn default(_request: &tiny_http::Request) -> (String, u32) {
    (String::from("Page not found"), 404)
}

```


## Try it

You can run `cargo run` that will launch the server. Then you can visit **http://localhost:5000/** and follow the "broken link".
In the browser you will see the HTML content.

If you use `curl` with the `-i` flag you will see (in the first line of the result) that the status code was indeed set to **404**.
It automatically comes with the text "Not Found". At the bottom of the response you can see the text we sent back.


```
$ curl -i http://localhost:5000/other

HTTP/1.1 404 Not Found
Server: tiny-http (Rust)
Date: Wed, 20 Dec 2023 08:09:16 GMT
Content-Type: text/html
Content-Length: 14

Page not found
```

## The full source code

{% include file="examples/tiny-http/page-not-found-404/src/main.rs" %}


