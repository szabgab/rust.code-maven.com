---
title: Rocket - multi-counter using cookies
timestamp: 2024-01-10T09:10:01
author: szabgab
published: true
description: Showing how to create a view-counter based on a cookie that will count the number of visits of that specific browser.
tags:
    - Rocket
    - web
    - rocket::http::CookieJar
    - get
    - value
    - add
---

This is a [Counter example](https://code-maven.com/counter), part of the [Rocket web development framework](/rocket) of Rust series.

Unlike the [Single counter in text file](/rocket-single-counter-in-text-file) example, here we don't use a file on the server to store the counter.
Instead we store the counter a **cookie in the browser**. This means that each browser that access this page will have its own counter.


## Dependencies

For this example to work we only need the Rocket crate:

{% include file="examples/rocket/multi-counter-using-cookies/Cargo.toml" %}


## The code

{% include file="examples/rocket/multi-counter-using-cookies/src/main.rs" %}


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

If we get a parsing error we need to decide what to do. For that we need to think over how could this happen. One way it can happen if someone manually sent in some invalid data. (e.g. using `curl`).
There could be also a bug in the browser software the user uses. A very unlikely possibility.
Finally there might be something wrong with the way we set the cookie.

We could return an error to the client, but in this case I thought it is better if we print some text to the console on the server and assume the cookie never existed and start from 0.
An arbitrary decision, but seemed fine for this example and seemed easier to implement than other solutions.

```rust
Err(_) => {
    eprintln!("Invalid value '{}' for the 'counter' cookie.", cookie.value());
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

We can then open another browser, or we can open a private window on the same browser and look at the same address. The counter there will start from 1.
Each browser (and each private tab) will have its own counter saved in the cookie.

We can now open the developer tools of the browser (eg. pressing Ctrl-Shift-i or F12 in Firefox) and look at the **Storage** tab. In there open the **Cookies** section

You will seem something like this:

![](images/multi-counter-using-cookies.png)

At the top we see the response of the browser. At the bottom we see the cookie as stored by the browser.

We can manually delete the cookie from the browser and if we then reload the page we'll see the counter starting from 1 again.


## Checking manually with curl

We can also look at the behavior using `curl`.

First we send a plain request to the page:

```
curl -i http://localhost:8000
```

and we get back

```
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
set-cookie: counter=1; SameSite=Strict; Path=/
server: Rocket
x-content-type-options: nosniff
x-frame-options: SAMEORIGIN
permissions-policy: interest-cohort=()
content-length: 10
date: Wed, 10 Jan 2024 06:28:37 GMT

Counter: 1
```

On the 3rd line we see `set-cookie`, this is how the web application tells the browser to save the cookie. The browser will then send back that cookie to every request to that site and the given path.

At the bottom of the response we see the content of the web page.

Then we can send a request and include the cookie. However, unlike the in the browser, here we can include an **arbitrary** number:

```
curl --cookie counter=3 -i http://localhost:8000
```

and we get:

```
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
set-cookie: counter=4; SameSite=Strict; Path=/
server: Rocket
x-content-type-options: nosniff
x-frame-options: SAMEORIGIN
permissions-policy: interest-cohort=()
content-length: 10
date: Wed, 10 Jan 2024 06:29:46 GMT

Counter: 4
```

We have more! We are not limited to sending a number in the cookie, we can send any value. For example we can send "bla":

```
curl --cookie counter=bla -i http://localhost:8000
```

The response will be:

```
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
set-cookie: counter=1; SameSite=Strict; Path=/
server: Rocket
x-content-type-options: nosniff
x-frame-options: SAMEORIGIN
permissions-policy: interest-cohort=()
content-length: 10
date: Wed, 10 Jan 2024 06:34:19 GMT

Counter: 1
```

Yes, that's what we implemented. If the client sends us garbage we will just pretend we have not received a cookie and start counting from 1.

We also print a warning on the console, so if you look at the terminal where you ran `cargo run` you will see:

`Invalid value 'bla' for the 'counter' cookie.`


## The tests

{% include file="examples/rocket/multi-counter-using-cookies/src/tests.rs" %}

In the first test-case we send a request without a cookie and check that we received the cookie as expected. It is a field in the `headers`.
We also check that the content of the page (`response.into_string()`) is what we expected.

In the second test-case we send the number 41 as the value of the cookie we set with the [cookie](https://api.rocket.rs/v0.5/rocket/local/blocking/struct.LocalRequest.html#method.cookie) method
and then we verify the newly set cookie and the response both containing 42.

In the third test-case we send "bla" again and we observe the system behaved as if there was no cookie at all.

We can run the tests by typing in

```
cargo test
```


## The extreme case, what if we reach the max value a u32 can store?

If I was still writing Perl or Python I think I would have been very satisfied with the solution, but since I started to write Rust I started to notice more edge-cases.
In this example I started to wonder what would happen if the cookie contained a number higher than [the highest possible number a u32 can contain](/minimum-and-maximum-values-of-numeric-types)
or exactly the highest which is 4294967295.

If it is higher than 4294967295 then the `parse` into `u32` would fail and the request would be treated as if there was no cookie at all.

```
curl -i --cookie counter=4294967299 http://localhost:8000


HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
set-cookie: counter=1; SameSite=Strict; Path=/
server: Rocket
x-frame-options: SAMEORIGIN
permissions-policy: interest-cohort=()
x-content-type-options: nosniff
content-length: 10
date: Wed, 10 Jan 2024 06:51:33 GMT

Counter: 1
```


If it was one before the max, we would get the max number:

```
$ curl -i --cookie counter=4294967294 http://localhost:8000

HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
set-cookie: counter=4294967295; SameSite=Strict; Path=/
server: Rocket
x-frame-options: SAMEORIGIN
permissions-policy: interest-cohort=()
x-content-type-options: nosniff
content-length: 19
date: Wed, 10 Jan 2024 06:51:40 GMT

Counter: 4294967295
```

If we pass the exact `u32::MAX` value which is 4294967295 then the `parse` will succeed but the increment will fail due to an overflow.
This will create a panic which is treated as an internal server error and Rocket will return its default **500 Internal Server Error** page.

```
$ curl -i --cookie counter=4294967295 http://localhost:8000

HTTP/1.1 500 Internal Server Error
content-type: text/html; charset=utf-8
server: Rocket
x-frame-options: SAMEORIGIN
permissions-policy: interest-cohort=()
x-content-type-options: nosniff
content-length: 488
date: Wed, 10 Jan 2024 06:51:44 GMT

<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="color-scheme" content="light dark">
    <title>500 Internal Server Error</title>
</head>
<body align="center">
    <div role="main" align="center">
        <h1>500: Internal Server Error</h1>
        <p>The server encountered an internal error while processing this request.</p>
        <hr />
    </div>
    <div role="contentinfo" align="center">
        <small>Rocket</small>
    </div>
</body>
</html>
```

Actually, if we ran the web application in **release mode** using the `--release` flag

```
cargo run --release
```

then the overflow would just, well, overflow and so the counter will become 0 and there is no panic and no 500-error.

```
$ curl -i --cookie counter=4294967295 http://localhost:8000

HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
set-cookie: counter=0; SameSite=Strict; Path=/
server: Rocket
permissions-policy: interest-cohort=()
x-content-type-options: nosniff
x-frame-options: SAMEORIGIN
content-length: 10
date: Wed, 10 Jan 2024 06:59:29 GMT

Counter: 0
```

However there are other options as well to [treat overflow](/how-to-handle-overflow). One would be to let the counter become 0.
Another one would be to use the `saturating_add` function that will keep the maximum number.

We could also opt to use a `u128` as well. It would not **solve** the problem, but it would allow us to count to a much bigger number.

## Conclusion

Rust forces me to think about more edge-cases. Sometimes by making me [unwrap](/unwrap) results,
sometimes by making me more aware of the limitations of the environment.




