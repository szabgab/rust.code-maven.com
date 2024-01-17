---
title: Rocket - multi counter using encrypted cookies
timestamp: 2024-01-12T18:00:01
author: szabgab
published: true
description: Using the secret_key and encrypted (private) cookies in this simple counter example.
tags:
    - cookies
    - add_private
    - get_private
    - secret_key
    - Rocket.toml
    - private_cookie
todo:
    - TODO
---

In an earlier example we saw how to create a [counter using cookies](/rocket-multi-counter-using-cookies)  as part of the [Rocket](/rocket) series.

We saw that the user can pretend to have any number in the counter by using `curl` or some other client where it is easy to send in any cookie.

This is another one of the [counter examples](https://code-maven.com/counter) where we'll use encrypted [private cookies](https://rocket.rs/v0.5/guide/requests/#private-cookies).

There are only a few changes from the free-text cookie version.


## Changes

* Include the `secrets` feature.
* Set the `secret_key` in the `Rocket.toml` file. See [explanation about the secret_key](https://rocket.rs/v0.5/guide/requests/#secret-key) and the about the [Rocket configuration](https://rocket.rs/v0.5/guide/configuration/) for more options.
* Use `get_private` and `add_private` instead of `get` and `add`.


## Dependencies in Cargo.toml

![](examples/rocket/multi-counter-using-encrypted-cookies/Cargo.toml)

We added the `secrets` feature


## Secret_key in Rocket.toml

![](examples/rocket/multi-counter-using-encrypted-cookies/Rocket.toml)

In this example we provide a `secret_key` for both the `debug` and the `release` mode.

If we don't provide a `secret_key` for the `debug` mode then it will be generated each time we start the application. That means cookies will not be recognized and accepted after
we restart the web application.

If we don't provide a `secret_key` for the `release` mode then the application will refuse to start.

The secret key can be generated, or one can just type in random characters.


## The code

![](examples/rocket/multi-counter-using-encrypted-cookies/src/main.rs)

Here the difference is that we use `get_private` instead of `get` and `add_private` instead of `add`.

## Running

```
cargo run
```


## Checking manually with two browsers

After launching the application we can visit `http://localhost:8000/` and we'll see `Counter: `.
Every time we reload the page we will get a higher number.

You can open a separate **private** tab or a different browser and visit the same URL. You'll notice the counters increase independently.

If you open the "Web developer tools", the "Storage" tab, Cookies,  you will see a line corresponding to the URL http://localhost:8000/
The name of the cookie "counter" will be visible, but the value will be just some random string. This is the encrypted value of the cookie.

Without the **secret_key** you won't be able to decrypt it. At least not in a reasonable amount of time.

![](images/multi-counter-using-encrypted-cookies.png)

## Checking manually with curl

The fist request goes well. We can see visible part `Counter: 1` and in the header we can see the cookie.

```
$ curl -i http://localhost:8000

HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
set-cookie: counter=cEdonPR4JzPyB0ox85kNcWU7Rmepp+trOcwjqWQ%3D; HttpOnly; SameSite=Strict; Path=/; Expires=Fri, 19 Jan 2024 15:34:11 GMT
server: Rocket
x-frame-options: SAMEORIGIN
permissions-policy: interest-cohort=()
x-content-type-options: nosniff
content-length: 10
date: Fri, 12 Jan 2024 15:34:11 GMT

Counter: 1
```

We can send in the encrypted cookie. We'll get back `Counter: 2` as expected and a new cookie value.

```
$ curl -i --cookie counter=cEdonPR4JzPyB0ox85kNcWU7Rmepp+trOcwjqWQ%3D http://localhost:8000

HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
set-cookie: counter=cJutBaxBcNPnGx17pKGJ6Gw%2FPzvVGkiP38S7BNo%3D; HttpOnly; SameSite=Strict; Path=/; Expires=Fri, 19 Jan 2024 15:35:54 GMT
server: Rocket
x-frame-options: SAMEORIGIN
permissions-policy: interest-cohort=()
x-content-type-options: nosniff
content-length: 10
date: Fri, 12 Jan 2024 15:35:54 GMT

Counter: 2
```

We can repeat the original request and we'll get a new cookie.


We can replace the value of the cookie with any other string. Unless we get lucky and hit one of the possible cookie values we will keep
getting "Counter: 1" as the application will keep disregarding the value.

## Testing with Rust

![](examples/rocket/multi-counter-using-encrypted-cookies/src/tests.rs)

At first I extracted the encrypted cookie string from the cookie and sent that back to the server, but that was not recognized as a correctly
encrypted cookie. I am not sure why.

I found out that the correct way in Rocket to to send encrypted cookies in a test is by calling the `private_cookie` method and providing
the real value there.

Because we are in the same environment, the test also has access to the `Rocket.toml` file where we have the `secret_key`.

## Conclusion

Using private cookies is not more difficult than plain text cookies and they are more secure.



