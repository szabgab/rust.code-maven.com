---
title: Rouille - Hello World in text/plain
timestamp: 2024-01-01T14:30:02
author: szabgab
published: true
description: The most simple web application using Rouille showing some plain text message but also returning 404 Not Found when necessary.
tags:
    - web
    - Rouille
    - start_server
    - router!
    - GET
    - text
    - 404 Not Found
    - empty_404
---

Part of the series about the [Rouille](/rouille) micro-web framework in Rust.


## Create a new crate

```
cargo new hello-world-text
cd hello-world-text
```

## Dependencies

Add Rouille as a dependency to the `Cargo.toml` file:

![](examples/rouille/hello-world-text/Cargo.toml)

## The code

![](examples/rouille/hello-world-text/src/main.rs)


* We call [rouille::start_server](https://docs.rs/rouille/latest/rouille/fn.start_server.html) providing it the hostname and port number. Giving "localhost" or "127.0.0.1" will ensure that the web server won't be accessible from outside of your computer. This is a good idea during development.

* The [router!](https://docs.rs/rouille/latest/rouille/macro.router.html) macro helps us define the routes.

* `(GET) (/) => {}`  is the mapping that say what to do when an HTTP GET request arrives to the root (`/`) path on the server.

* The [rouille::Response::text](https://docs.rs/rouille/latest/rouille/struct.Response.html#method.text) method send a response setting the `Content-Type` to `text/plain`.

* We also have a default route-handle: `_ => rouille::Response::empty_404()`  calling the [empty_404](https://docs.rs/rouille/latest/rouille/struct.Response.html#method.empty_404) method.

## Running the code

```
cargo run
```

Will print `Now listening on localhost:8000` on the console and we can visit http://localhost:8000/ with our browser and see

`Hello <b>world!</b>`

We won't see the text in bold, we'll see the HTML tag.


On Linux and probably also on macOs we can use another terminal to run the following command.


```
curl -i http://localhost:8000/
```

The `-i` flag of `curl` will make it show the header along the content.

```
HTTP/1.1 200 OK
Server: tiny-http (Rust)
Date: Mon, 01 Jan 2024 12:40:58 GMT
Content-Type: text/plain; charset=utf-8
Content-Length: 11

Hello <b>world!</b>
```

Here we can see clearly the `Content-Type` being `text/plain`. That's the reason the browser shows us the text and does not try to interpret it as HTML.

## 404 Not found

If we visit any other URL (e.g. http://localhost:8000/qqrq ) we'll get a blank page.

With `curl` we can verify that the HTTP response code was 404 as expected.

```
$ curl -i http://localhost:8000/qqrq
```

```
HTTP/1.1 404 Not Found
Server: tiny-http (Rust)
Date: Mon, 01 Jan 2024 12:45:42 GMT
Content-Length: 0
```


