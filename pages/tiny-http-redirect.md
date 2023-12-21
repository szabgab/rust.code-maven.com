---
title: Tiny HTTP redirect URL
timestamp: 2023-12-21T11:30:01
published: true
description: HTTP Redirecting a page to another URL.
tags:
    - tiny_http
    - web
    - Header
    - HeaderField
    - AsciiString
    - Request
    - Response
---

Part of the [Tiny HTTP series](/tiny-http).

Sometimes you'd like to have a URL that, instead of showing content would redirect the browser to some other page on the same web site or
on a different web site. For example I have a path on the [Code Maven](https://code-maven.com/) web site that will redirect the users to
my YouTube channel: [https://code-maven.com/youtube](https://code-maven.com/youtube). I created this redirection so it will be easy for me
to link to the channel and even to type it in when I need to do that.

I often need redirection behind a login page, so when someone visits a page and then wants to login to the website I can send the person
back to the same page after s/he logged in. This can be implemented in the front-end without redirection, but if I'd like to do it in the
back-end then redirection can be useful.

In technical terms a redirection is created by setting the status code to one of the appropriate numbers in the 300-399 range
and adding a field called `Location` to the header with the target URL as the value.

There is a page listing all the [HTTP response status codes](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status)
and a separate page describing [Redirections in HTTP](https://developer.mozilla.org/en-US/docs/Web/HTTP/Redirections) in more details.


## Rust code to redirect URL in Tiny HTTP

This is the core of the solution.

First we create a [Header](https://docs.rs/tiny_http/latest/tiny_http/struct.Header.html) with a field
called `Location` and a value supplied by the caller that is the target URL  where w would like to redirect the user to.

Then we create a response. The content of the response can be an empty string. The status code must be set correctly
and we need to add the header.

```rust
fn redirect(request: tiny_http::Request, site: &str, status_code: u16) {
    let header = Header {
        field: HeaderField::from_str("Location").unwrap(),
        value: AsciiString::from_ascii(site).unwrap(),
    };
    request
        .respond(
            Response::from_string("")
                .with_status_code(StatusCode::from(status_code))
                .with_header(header),
        )
        .unwrap();
}
```

This is how we would call it:

```rust
redirect(request, "https://rust.code-maven.com/", 302);
```

Where 302 means "temporary redirect".

## full source code

![](examples/tiny-http/redirect/Cargo.toml)

![](examples/tiny-http/redirect/src/main.rs)


