---
title: Set the User-Agent in a HTTP request using Rust reqwest
timestamp: 2024-01-25T13:40:01
author: szabgab
published: true
description:
tags:
    - User Agent
    - reqwest
    - header
---


## Dependencies

{% include file="examples/reqwest-set-user-agent/Cargo.toml" %}

## The code

{% include file="examples/reqwest-set-user-agent/src/main.rs" %}

## The output

```
{
  "headers": {
    "Accept": "*/*",
    "Host": "httpbin.org",
    "X-Amzn-Trace-Id": "Root=1-65b23d5c-7291d26c5121b8d160a837f9"
  }
}

{
  "headers": {
    "Accept": "*/*",
    "Host": "httpbin.org",
    "User-Agent": "Rust Maven 1.42",
    "X-Amzn-Trace-Id": "Root=1-65b23d5c-1376c1c654589f201d8f958c"
  }
}
```
