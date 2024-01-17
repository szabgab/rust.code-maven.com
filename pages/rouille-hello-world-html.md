---
title: Rouille - Hello World in text/html
timestamp: 2024-01-01T15:10:02
author: szabgab
published: true
description:
tags:
    - web
    - Rouille
    - start_server
    - router!
    - GET
    - html
    - 404 Not Found
    - with_status_code
---

Part of the series about the [Rouille](/rouille) micro-web framework in Rust.

## Dependency

![](examples/rouille/hello-world-html/Cargo.toml)


## The code

![](examples/rouille/hello-world-html/src/main.rs)


## Running the web application



```
$ curl -i http://localhost:8000/
```

The response is:

```
HTTP/1.1 200 OK
Server: tiny-http (Rust)
Date: Mon, 01 Jan 2024 13:00:06 GMT
Content-Type: text/html; charset=utf-8
Content-Length: 19

Hello <b>world!</b>
```

Here we can see the `Content-Type` being `text/html`.

If we visit the URL http://localhost:8000/ we'll see the word `world!` in bold.


## 404 Not Found

This time, instead of using the `empty_404` call we used the `html` call and changed the status code manually to be 404.

```
_ => rouille::Response::html("This page does <b>not</b> exist.").with_status_code(404)
```

If we visit any page other than the root page we'll see the special message "This page does **not** exist." and the word not is being bold.

We can use `curl` to verify it:

```
$ curl -i http://localhost:8000/qqrq
```

```
HTTP/1.1 404 Not Found
Server: tiny-http (Rust)
Date: Mon, 01 Jan 2024 13:07:17 GMT
Content-Type: text/html; charset=utf-8
Content-Length: 32

This page does <b>not</b> exist.
```

