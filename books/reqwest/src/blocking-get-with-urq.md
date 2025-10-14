# Blocking HTTP GET request with ureq

[ureq](https://crates.io/crates/ureq) was recommended as a better alternative to reqwest for blocking requests.

In this example we'll see how to use it to send a simple GET request.


## Dependencies

{% embed include file="src/examples/ureq/get/Cargo.toml" %}

## Code

`get_url` just gets the url from the command line

We get a response that can be either a good response or an error. In case of a good response we can print the content of the response.


## Success

```
$ cargo run http://localhost/get

GET request to: http://localhost/get
Status: 200 OK
------- Headers --------
server: "gunicorn/19.9.0"
date: "Tue, 14 Oct 2025 10:07:54 GMT"
connection: "keep-alive"
content-type: "application/json"
content-length: "210"
access-control-allow-origin: "*"
access-control-allow-credentials: "true"
------- Body --------
Body: {
  "args": {}, 
  "headers": {
    "Accept": "*/*", 
    "Accept-Encoding": "gzip", 
    "Host": "localhost", 
    "User-Agent": "ureq/3.1.2"
  }, 
  "origin": "172.17.0.1", 
  "url": "http://localhost/get"
}
```


## 404 status code

```
$ cargo run -q http://localhost/status/404

GET request to: http://localhost/status/404
Error: http status: 404
```

## 500 status code

```
$ cargo run -q http://localhost/status/500

GET request to: http://localhost/status/500
Error: http status: 500
```


## Redirect:

This will follow the redirect and thus it will print out the content of the target web site.

```
$ cargo run "http://localhost/redirect-to?url=https://rust.code-maven.com/&status_code=301"
```

{% embed include file="src/examples/ureq/get/src/main.rs" %}

