# Simple blocking HTTP POST request using Rust

The reqwest crate provides all the capabilities to send HTTP requests.

In this example we are using the [reqwest](https://crates.io/crates/reqwest) crate to send an HTTP POST request to [https://httpbin.org/](https://httpbin.org/).


## The curl command


This is the same command as executed using `curl`.

```
curl -X POST -d "text=Hello World!" http://httpbin.org/post
```

This is the result:

```json
{
  "args": {},
  "data": "",
  "files": {},
  "form": {
    "text": "Hello World!"
  },
  "headers": {
    "Accept": "*/*",
    "Content-Length": "17",
    "Content-Type": "application/x-www-form-urlencoded",
    "Host": "httpbin.org",
    "User-Agent": "curl/8.2.1",
    "X-Amzn-Trace-Id": "Root=1-65b22590-3ba351816c46408426023f1b"
  },
  "json": null,
  "origin": "46.120.9.250",
  "url": "http://httpbin.org/post"
}
```


## Dependencies

In order to be able to send a `blocking` request we need to add that feature, and in order to be able to parse the returned [JSON](/json) data we had to add the `json` feature.

{% embed include file="src/examples/simple-blocking-http-post-reqwest/Cargo.toml" %}


## The code

We can pass the parameters in the `form` method as a bunch of key-value pairs from a HashMap.

{% embed include file="src/examples/simple-blocking-http-post-reqwest/src/main.rs" %}

## The result

This is the resulting data from the request.

```rust
Object {
    "args": Object {},
    "data": String(""),
    "files": Object {},
    "form": Object {
        "text": String("Hello World!"),
    },
    "headers": Object {
        "Accept": String("*/*"),
        "Content-Length": String("19"),
        "Content-Type": String("application/x-www-form-urlencoded"),
        "Host": String("httpbin.org"),
        "X-Amzn-Trace-Id": String("Root=1-65b225c3-1cfb5c6440c0d3014b818197"),
    },
    "json": Null,
    "origin": String("46.120.9.250"),
    "url": String("http://httpbin.org/post"),
}
```

---

- reqwest
- HTTP
- POST
- form


