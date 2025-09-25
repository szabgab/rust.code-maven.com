# Reqwest the HTTP client library of Rust

Using the [reqwest](https://crates.io/crates/reqwest) crate.

Reqwest is a Rust crate to handle HTTP requests.

* We use [httpbin](https://httpbin.org/) for checking examples.


* [Simple blocking HTTP GET request in Rust](./simple-blocking-http-get-request.md) - `GET`.
* [Simple blocking HTTP POST request using Rust ](./simple-blocking-http-post-request.md) - `POST`, `Client`.
* [Set the User-Agent in a HTTP request using Rust reqwest](./reqwest-set-user-agent.md) - `header`.
* [HTTP reqwest sending cookie](./reqwest-send-cookie.md) - `header`, `Cookie`.


In order to handle secure HTTPS request I had to install the following packages on Ubuntu:

```
apt-get install pkg-config
apt-get install libssl-dev
```


