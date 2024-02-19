---
title: "Rocket: get, set (add), delete cookies - pending cookies"
timestamp: 2024-02-01T10:30:01
author: szabgab
published: true
description: A demo to add a new cookie, update a cookie, delete one and get the value of the cookie in the same route where we set it.
tags:
    - Rocket
    - web
    - get
    - add
    - delete
    - get_pending
---

In this example of the [Rocket web framework](/rocket) we'll see how to set or update (`add`) a cookie.
How to `get` a cookie sent by the user. How to `delete` a cookie from the browser, or at least how to ask the browser to delete it.

Finally we'll deal with `pending cookies`. That is we would like to ensure that in the same route where we make changes to a cookie,
the rest of the code already see the new value of the cookie and not the one we received from the user.

## Dependencies

{% include file="examples/rocket/set-cookie/Cargo.toml" %}

## The code

{% include file="examples/rocket/set-cookie/src/main.rs" %}

## Explanation

We have a function called `get_time` that returns the elapsed time since the epoch. We just needed something easy to have always changing values.
We will set the value returned by this function to be the value of the cookie.

We have 3 routes to set the cookie, delete the cookie and to only look at the cookie (the home page), and they all look the same. So we have a function called
`get_html`. It will `get` the cookie and it will return the shared html that includes the **current time** and the **value of the cookie** which is the timestamp
when we `set` the cookie.

```rust
fn get_html(cookies: &CookieJar<'_>,  current_time: &str) -> content::RawHtml<String> {
    let saved_time: String = match cookies.get("cookie-demo") {
        Some(cookie) => cookie.value().to_owned(),
        None => String::from("No cookie"),
    };

    content::RawHtml(format!(r#"<a href="/">home</a> <a href="/set">set cookie</a> <a href="/delete">delete cookie</a><br>Current time: {current_time}<br>Saved time: {saved_time}<br>"#))
}
```

## Interaction

When you first visit the page you will see (with some other timestamp):

```
Current time: 1706775050281184
Saved time: No cookie
```

That's because we don't have a cookie yet.


If you click on **set cookie** you will see:

```
Current time: 1706775061279344
Saved time: No cookie
```

So despite the fact that we set the cookie it still thinks we don't have a cookie. That's because the `get` method returns the cookie that we received from the client.
In order to get the cookie that we are about to send to the user we can use the [get_pending](https://api.rocket.rs/v0.5/rocket/http/struct.CookieJar.html#method.get_pending) method.

If we now click on the **home** we'll see a new current time, and the saved time is the timestamp from when we clicked on **set**.

```
Current time: 1706775070699980
Saved time: 1706775061279344
```

We can keep refreshing the home page and we'll keep seeing the current time changing while the saved time fixed.

If we click on **delete cookie** it will delete the cookie, but it will still show the values from when we had a cookie.

```
Current time: 1706775167878389
Saved time: 1706775061279344
```

If we click on **home** now we'll see that the cookie is gone:

```
Current time: 1706775203917017
Saved time: No cookie
```

## Use the pendig cookie

If you want to make sure that the page reflects the inew cookie value on the page where we set it, or the lack of cookie on the page where we delete it then instead of
`cookies.get("cookie-demo")` we can call `cookies.get_pending("cookie-demo")` using the [get_pending](https://api.rocket.rs/v0.5/rocket/http/struct.CookieJar.html#method.get_pending)
method.

One would think that it would be better to always se this method, but the documentation warns us of this being slower and recommends that we only use this method when we really need it.

In my exmaple this is not possible as all 3 operations (get/set/delete) uses the same code, but in a real-world application you'd call the `get_pending` only after you called `add` or `delete`
that changes the state of the cookie and you'd called `get` (or `get_private`) in every other case.

