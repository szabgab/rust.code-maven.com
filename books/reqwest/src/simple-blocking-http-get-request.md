# Simple blocking HTTP GET request in Rust

The [reqwest](https://crates.io/crates/reqwest) crate provides functions for both asynchronous and blocking http requests.
Although in most cases you'd probably want to use the asynch calls, using the blocking calls is simpler, so we start with that.

This is what we add to the `Cargo.toml`

{% embed include file="src/examples/reqwest/simple-blocking-http-get-request/Cargo.toml" %}

And this is the code:

{% embed include file="src/examples/reqwest/simple-blocking-http-get-request/src/main.rs" %}

This is the output (slightly reformatted to make it easier to read).


```
Response {
    url: Url {
        scheme: "https",
        cannot_be_a_base: false,
        username: "",
        password: None,
        host: Some(Domain("httpbin.org")),
        port: None,
        path: "/ip",
        query: None,
        fragment: None
    },
    status: 200,
    headers: {
        "date": "Tue, 03 Oct 2023 13:12:58 GMT",
        "content-type": "application/json",
        "content-length": "31",
        "connection": "keep-alive",
        "server": "gunicorn/19.9.0",
        "access-control-allow-origin": "*",
        "access-control-allow-credentials": "true"
    }
}

status: 200

server: "gunicorn/19.9.0"

{
  "origin": "46.120.9.250"
}
```

---

- reqwest
- blocking
- HTTP
- GET


