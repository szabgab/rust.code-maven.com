---
title: Getting started with Tiny HTTP building a web application in Rust
timestamp: 2023-12-13T15:30:01
published: true
description: tiny_http is a very simple, dear to say tiny web server
tags:
    - tiny_http
    - web
todo:
    - process path, create flexible routes e.g. /users/<username>
    - send back the content of a file (html / image)
    - Set Status code to 301, to 500
    - show the request type GET/POST/HEADER/etc.
    - process the POST parameters
    - return JSON
    - return HTML content from template
---

[tiny_http](https://crates.io/crates/tiny_http) is, well, a tiny crate to create web applications.

In this series of articles we are going to see how to use it to build a simple web application.

* [Hello World](/tiny-http-hello-world) - set Content-type to `text/html`. `Server`, `http`, `incoming_requests`, `Response`, `HeaderField`, `Header`, `AsciiString`
* [Generate dynamic response, show the current time](/tiny-http-show-current-time).  `chrono`, `UTC`, `Local`, `date`, `time`, `HTML`
* [Echo using GET request](/tiny-http-echo-get) - parse the `QUERY_STRING.`, `GET`, `url`, `query_pairs`, `parse`
* [Path based routing](/tiny-http-path-based-routing). `path`, `match`
* [Setting status to 404 page not found](/tiny-http-404-page-not-found). `with_status_code`, `with_header`, `path`.
* [Tiny HTTP redirect URL](/tiny-http-redirect). `with_header`, `Header`, `HeaderField`, `Request`, `Response`.
