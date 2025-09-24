# Simple blocking http client with reqwest sending a GET request

Create a new crate

```
cargo new demo
cd demo
```

Add the [reqwest](https://crates.io/crates/reqwest) crate as a dependency with the `blocking` feature:

```
cargo add reqwest --features blocking
```


## Cargo.toml

Or manually edit the `Cargo.toml` file to add it as a dependency.

{% embed include file="src/examples/reqwest/simple-http-client/Cargo.toml" %}


## Code

{% embed include file="src/examples/reqwest/simple-http-client/src/main.rs" %}


## Run the code

```
cargo run
```

If everything works fine then you'll get back something like this:

```
200
Response {
   url: "https://httpbin.org/get",
   status: 200,
   headers: {
     "date": "Wed, 24 Sep 2025 18:09:11 GMT",
     "content-type": "application/json",
     "content-length": "219",
     "connection": "keep-alive",
     "server": "gunicorn/19.9.0",
     "access-control-allow-origin": "*",
     "access-control-allow-credentials": "true"
  }
}
```

If the service does not work we'll see an error message:

```
Error error sending request for url (https://httpbin.org/get)
```

In that case we can also run the httpbin service locally using Docker.

Start as:

```
docker run --rm -p 80:80 --name httpbin kennethreitz/httpbin
```

Then run the code:

```
cargo run localhost
```

To stop the Docker container open another terminal and execute

```
docker container stop -t0 httpbin
```



---

* reqwest
* blocking
* get
* status


