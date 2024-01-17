---
title: "Rocket: Web-based Hello World! with tests"
timestamp: 2023-12-22T12:31:01
author: szabgab
published: true
description: The most simple Hello World web aplication in Rust with embedded tests.
tags:
    - get
    - mount
    - build
    - web
---

Part of the series about the [Rocket web framework](/rocket).


## Create crate

In order to have a Rocket, we need to create a regular Rust crate.

```
cargo new hello-world
cd hello-world
```

## Setup dependencies

Add

```
rocket = "0.5.0"
```

to the Cargo.toml file:

![](examples/rocket/hello-world/Cargo.toml)


## Setup the use of the macros

```rust
#[macro_use]
extern crate rocket;
```

## Route

Define a route and what it should do and return.

```rust
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
```

This create a route that will handle any GET request arriving to the root of the site: `/`.
The function will be executed and the return value of the function will be returned to the
client.


## Map the routes


```
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
```

This maps the "/" request to the index function. At first it is strange that we need to define the mapping of urls twice,
but later we'll see how this can be actually useful.

## run the application

Just type in

```
cargo run
```

It will compile the code and start running the server listening on port 8000 by default.

Visit `http://localhost:8000/` with your browser to see **"Hello world!**.



## Content-type is text/plain

Using the [curl](https://curl.se/) command with the `-i` flag will show both the header and the content of the response.
We can see that the `content-type` returned was `text/plain`.


```
$ curl -i http://localhost:8000
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
server: Rocket
x-content-type-options: nosniff
x-frame-options: SAMEORIGIN
permissions-policy: interest-cohort=()
content-length: 13
date: Fri, 22 Dec 2023 10:13:03 GMT

Hello, world!
```

## Testing the application

Before we end this example, let's see how can we test if it works properly.
This is the testing code that goes into the `src/main.rs` file.

It sets up a fake client and uses that to GET a url (by calling the `get` method.

We then verify that the status code is 200 OK and we compare the received content to the expected content.


```rust
#[cfg(test)]
mod test {
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn hello_world() {
        let client = Client::tracked(super::rocket()).unwrap();
        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), Some("Hello, world!".into()));
    }
}
```


## The full example

![](examples/rocket/hello-world/src/main.rs)

