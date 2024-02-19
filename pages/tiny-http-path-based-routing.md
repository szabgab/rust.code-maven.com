---
title: Tiny HTTP - path based routing
timestamp: 2023-12-15T12:30:01
author: szabgab
published: true
description: Path based routing is the idea that each path in a URL is mapped to a function and that function handles the request.
tags:
    - tiny_http
    - web
    - path
    - match
    - _
todo:
    - handle the more flexible pathes.
---

Part of the [Tiny HTTP series](/tiny-http).

One of the common ways style of web development employed by many we frameworks is the **path-based routing**.

Regardless of where you host your web application it will have URLs like these:

```
/
/newsletter
/about
/user/
/user/joe
/user/jane
...
```

If the request has parameters such as in this case:

```
/api/newsletter?name=Foo
```

we would disregard the parameters and use only

```
/api/newsletter
```

In order to make the code cleaner you could map each such path to a function and let that function handle the request.

So in the above case you will have a function to handle the request to `/`, one to handle the request to `/newsletter` and another one to handle the `/about`.
For the pathes that start with `/user/` you won't be able to implement one function each as there might be any number of such pathes. One for each user of your system.
So you will have a single function that can handle all the users.

In this example we'll see how to handle the fixed pathes (the first 3). Handling pathes in a more flexible way is left as an exercise to the reader. (and as TODO item for myself.)

## Full example

{% include file="examples/tiny-http/path-based-routing/src/main.rs" %}

## Explanation

The first thing we need to do is to extract the path from the url. You might recall the return value of the `url` method starts from the initial `/`
and does not include the hostname. We first try to [split the string into two variables](/split-string-into-two-variables), but for the second variable
we use `_` as we don't actually need that value.

If the `split_once` fails then we know there is no `?` in the string so we can return the whole string.

```rust
let path = if let Some((path, _)) = request.url().split_once('?') {
    path
} else {
    request.url()
};
```

Then we use the `match` pattern matching operator to implement the mapping. Each path is mapped to a function. Each function receives the `request` object as a parameter.
The last one, mapping the underscore `_` to a function called `default` will be triggered if none of the earlier cases matched.


```rust
let html = match path {
    "/" => root_page(&request),
    "/hello" => hello(&request),
    _ => default(&request),
};
```

The functions themselves are rather simple. In this simple case none of them make any use of the `request` parameter, hence we prefixed them with an underscore.

## How to try it?

```
cargo run
```

Will compile the code and run the web server.

* Visit `http://localhost:5000/`
* Click on the link that will lead you to `http://localhost:5000/hello`
* Click on the "home" link.
* Type in `http://localhost:5000/blabla` and you will see "Page not found".

## Conclusion

* The `default` function should probably set the status code to 404 to indicate that the page not found
* We only handle fixed pathes. We should be able to handle pathes in a more flexible way.
* We handle all the requests the same way regardless if they were a GET, POST, HEADER, etc. request.

I am sure there is a lot more to do, but this can already help make the code nicer.


