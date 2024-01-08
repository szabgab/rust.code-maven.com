---
title: Rocket - multi-counter using cookies
timestamp: 2024-01-03T14:30:01
published: false
description:
tags:
    - Rust
todo:
    - TODO
---

This is a [Counter example](https://code-maven.com/counter), part of the [Rocket web development framework](/rocket) of Rust series.

Unlike the [Single counter in text file](/rocket-single-counter-in-text-file) example, here we don't use a file on the server to store the counter.
Instead we store the counter a **cookie in the browser**. This means that each brower that access this page will have its own counter.


## Dependencies

For this example to work we only need the Rocket crate:

![](examples/rocket/multi-counter-using-cookies/Cargo.toml)


## The code

![](examples/rocket/multi-counter-using-cookies/src/main.rs)


We use the [rocket::http::CookieJar](https://api.rocket.rs/v0.5/rocket/http/struct.CookieJar.html).

We only have one route and there we expect a parameter `cookies: &CookieJar<'_>`. This tells Rocket to parse the header of the request and
extract the cookie from the header into the `cookies` variable.

There we can call the `get` method passing it the name of the cookie to fetch the value of the cookie the browser sent back.

```
cookies.get("counter")
```

IF the browser does not have that cookie yet it wont send the cookie and this function will return `None`. In order to handle these two cases we use a `match` and
in case we received `None` we set the counter to be 0.

If we received something we still have two possibilities. We either receive a value that can be a proper counter number or we get back some garbage.
If we can `parse` the received value into a `u32` it is a proper value and we set the counter to that.

```rust
Ok(val) => val,
```

If we get a parsing error we need to decide what to do. For that we need to think over how could this happen. One way it can happen if someone manuall sent in some invalid data. (e.g. using `curl`).
There could be also a bug in the browser software the user uses. A very unlikely possibility.
Finally there might be something wrong with the way we set the cookie.

We could return an error to the client, but in this case I though it is better if we print some text to the console on the server and assume the cookie never existed and start from 0.

```rust
Err(_) => {
    eprintln!("Invalid value {} for the 'counter' cookie.", cookie.value());
    0
},
```

Once we have the value of the counter we increment it by 1, set a cookie called "counter" using the new value.
Then we return a string formatted using this number.

## Running

Running the application has nothing unusual:

```
cargo run
```

## Checking manually with two browsers

Once the server is running we can visit `http://localhost:8000/`  and see **Counter: 1**.
Every time we reload the page (e.g. by pressing Ctrl-r) we see the number increase by one.

We can then open another borwser, or we can open a private window on the same browser.


## Checking manually with curl


curl --cookie counter=3 -i http://localhost:8000

## The tests

![](examples/rocket/multi-counter-using-cookies/src/tests.rs)

[cookie](https://api.rocket.rs/v0.5/rocket/local/blocking/struct.LocalRequest.html#method.cookie)

```
cargo test
```
