# HTTP reqwest sending cookie

Sending a cookie back to the server using the reqwest crate.

## The curl command

```
curl --cookie counter=42 "https://httpbin.org/get"
```

and the output:

```
{
  "args": {},
  "headers": {
    "Accept": "*/*",
    "Cookie": "counter=42",
    "Host": "httpbin.org",
    "User-Agent": "curl/8.2.1",
    "X-Amzn-Trace-Id": "Root=1-65b2455d-400413860278b6624dc30284"
  },
  "origin": "46.120.9.250",
  "url": "https://httpbin.org/get"
}
```


## The dependencies

{% embed include file="src/examples/simple-blocking-http-reqwest-sending-cookie/Cargo.toml" %}


## The code

{% embed include file="src/examples/simple-blocking-http-reqwest-sending-cookie/src/main.rs" %}

## The output

```
Object {
    "args": Object {},
    "headers": Object {
        "Accept": String("*/*"),
        "Cookie": String("counter=42"),
        "Host": String("httpbin.org"),
        "X-Amzn-Trace-Id": String("Root=1-65b244fe-37f44f2f5385618e52e9397b"),
    },
    "origin": String("46.120.9.250"),
    "url": String("https://httpbin.org/get"),
}
```

---

- reqwest
- header
- Cookie
- Client



