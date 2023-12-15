---
title: Getting start with Tiny HTTP building a web application in Rust
timestamp: 2023-12-13T15:30:01
description: tiny_http is a very simple, dear to say tiny web server
tags:
    - tiny_http
    - web
todo:
    - process path, create "routes"
    - send back the content of a file (html / image)
    - Set Status code to 404, 301, to 500
    - show the request type GET/POST/HEADER/etc.
    - process the POST parameters
---

[tiny_http](https://crates.io/crates/tiny_http) is, well, a tiny http server.

In this series of articles we are going to see how to use it to build a simple web application.

* [Hello World](/tiny-http-hello-world) - set Content-type to `text/html`.
* [Generate dynamic response, show the current time](/tiny-http-show-current-time).
* [Echo using GET request](/tiny-http-echo-get) - parse the QUERY_STRING.


In this article we'll see a couple of basic examples using it.


