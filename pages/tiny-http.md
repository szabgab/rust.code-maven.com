---
title: Getting start with Tiny HTTP building a web application in Rust
timestamp: 2023-11-11T18:15:01
description: tiny_http is a very simple, dear to say tiny web server
tags:
    - tiny_http
---

[tiny_http](https://crates.io/crates/tiny_http) is, well, a tiny http server.

In this article we'll see a couple of basic examples using it.

## Hello World

We need created a Crate using the `cargo new hello-world` command that will create a folder called `hello-world`.

Then change the directory to the newly created folder: `cd hello-world` and then add the `tiny_http` crate by running:

```
cargo add tiny_http
```

This will update the `Cargo.toml` file to something like this:

![](examples/tiny-http/hello-world/Cargo.toml)


![](examples/tiny-http/hello-world/src/main.rs)

* First we need to create a "server" by calling `Server::http`. I've added a call to `expect` at the end that will `panic!` if we operation is not successful. For example if the port is already in use.
* Then we go into an infinite look waiting for clients (borwsers) to access the server: `for request in server.incoming_requests() {`.
* Then we attach our response text to the request object.

We can run the application by executing the `cargo run` command and then we can visit **http://localhost:5000/** with our browser. It will show the text "Hello World!".

If you have [curl](https://curl.se/) installed you can also check the response with it:

```
$ curl http://127.0.0.1:5000/

Hello World
```

We can also use the `-i` flag of `curl` to display the head that the server sent.

```
$ curl -i http://127.0.0.1:5000/

HTTP/1.1 200 OK
Server: tiny-http (Rust)
Date: Mon, 11 Dec 2023 15:39:47 GMT
Content-Type: text/plain; charset=UTF-8
Content-Length: 11

Hello World
```

You might notice it identifies itself as `tiny-http (Rust)` and that the `Content-Type` is set to `text/plain`.


## Hello World as HTML

Stop the server by pressing `Crtl-C`, then change the respons text from

```
"Hello World!"
```

to

```html
"<h1>Hello World</h1>"
```

restart the server by running `cargo run` again and reload the page in your browser.
Instead of showing the text

```
Hello World!
```

in large letters it will also show the HTML tags:

```html
<h1>Hello World</h1>
```

We'll have to change the `Content-type` if we want to tell the browser to display the content as HTML.

For this I had to first add another crate called [ascii](https://crates.io/crates/ascii)

![](examples/tiny-http/hello-world-as-html/Cargo.toml)


![](examples/tiny-http/hello-world-as-html/src/main.rs)

We had to add a few `use`-statements, create a header:

```rust
let header = Header {
    field: HeaderField::from_str("Content-type").unwrap(),
    value: AsciiString::from_ascii("text/html").unwrap(),
};
```
and then add it to the response by calling `with_header(header)`.

If we now restart the server (stop by pressing `Ctrl-C` and then run `cargo run`) and then we visit the web site we'll see
"Hello World!", but this time in large letters as required by the `h1` HTML elements.

Running `curl -i` reveals tha the `Content-type` in the header was changed to `text/html` that told the browser to interpret the
HTML tags.

```
$ curl -i http://127.0.0.1:5000/

HTTP/1.1 200 OK
Server: tiny-http (Rust)
Date: Mon, 11 Dec 2023 16:12:40 GMT
Content-Type: text/html
Content-Length: 20

<h1>Hello World</h1>
```


