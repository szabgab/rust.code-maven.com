---
title: Accept cookise in an HTTP request sent by the server
timestamp: 2024-01-03T14:30:01
author: szabgab
published: false
description:
tags:
    - Rust
todo:
    - TODO
---

```
curl -i "http://httpbin.org/cookies/set?name=Foo+Bar"
```

In this request we asked the httpbin server to send us a cookie (normally this is the decision of the server, but the httpbin server is here to help us).

Output (showing only the header part) where we can see the row `Set-Cookie`, that is setting a cookie in our "broswer".

```
HTTP/1.1 302 FOUND
Date: Thu, 25 Jan 2024 13:22:52 GMT
Content-Type: text/html; charset=utf-8
Content-Length: 223
Connection: keep-alive
Server: gunicorn/19.9.0
Location: /cookies
Set-Cookie: name="Foo Bar"; Path=/
Access-Control-Allow-Origin: *
Access-Control-Allow-Credentials: true
```


## Dependencies

{% include file="examples/reqwest-accept-cookies/Cargo.toml" %}

## The code

{% include file="examples/reqwest-accept-cookies/src/main.rs" %}

## The output


