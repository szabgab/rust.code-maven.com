---
title: "Tiny HTTP: Echo using GET request"
timestamp: 2023-12-13T17:10:01
description: A simple GET request, parsing the parameters, returning the text back
tags:
    - tiny_http
    - GET
    - url
    - query_pairs
    - parse
    - web
todo:
    - can we improve the get_url_parameters function?
---

Part of the [Tiny HTTP](/tiny-http) series.

This is a very simple application. When we visit the web site we see a single box where we can type in some text and a button.

![](/images/tiny-http-echo-get-before.png)

When we click the button the server show the same box and button and it also shows **You typed** and the text we typed in.

![](/images/tiny-http-echo-get-response.png)

This already shows a full round-trip processing of HTTP request and response.


In order to make this work we had to add the [url](https://crates.io/crates/url) crate so our `Cargo.toml` now looks like this:

![](examples/tiny-http/echo-get/Cargo.toml)

## The source code

![](examples/tiny-http/echo-get/src/main.rs)


## The explanation

There is a function now called `get_url_parameters` that receives a reference to a [tiny_http::Request](https://docs.rs/tiny_http/latest/tiny_http/struct.Request.html).

The [url](https://docs.rs/tiny_http/latest/tiny_http/struct.Request.html#method.url) method will return the url starting from the initial `/`.
Excluding the protocol that is either `http` or `https`, excluding the hostname and excluding the port number. So if we browse to this address:

`http://localhost:5000/api/echo?text=Hello+World!&name=Foo%20Bar&text=more%20info`

The `url` method will return

`/api/echo?text=Hello+World!&name=Foo%20Bar&text=more%20info`

From this we need to extract the part that is after the `?` that are the parameters.

I could not figure out how to feed this string to the [url](https://crates.io/crates/url) crate so I created a `fake_url`
and passed that to the [url::Url::parse](https://docs.rs/url/latest/url/struct.Url.html#method.parse) method.

Using that we go over the pairs returned by [query_pairs](https://docs.rs/url/latest/url/struct.Url.html#method.query_pairs)
and we build a hash of vectors. Why vectors and not single values? Because one can supply the same key multiple times on the URL.
In our example above we used the "text" field name twice.

I think I could have decided to use the first or the last value  in case there are multiple values, but accepting more than one possible values
seemed like a more correct solution. This of course will mean that the user of this data structure will have to handle one or more values.

Given this request `http://localhost:5000/?text=Hello+World!&name=Foo%20Bar&text=more%20info`, these are the params:

```
{
    "name": ["Foo Bar"],
    "text": ["Hello World!", "more info"]
}
```

Now let's see the body of the `for request in server.incoming_requests()` loop.

* We create the header to be able to set the Content-type to `text/html` as we have already seen in the [Hello World](/tiny-http-hello-world) example.
* The we create a string called `html` with the HTML form. Embedding HTML in our Rust code is a nasty solution, but I did not want to deal with template system at this point. The variable was made mutable so it will be possible to add the response text.
* Then we call `get_url_parameters` described earlier to get the parameters from the URL.
* Then we have the code to add the reply. When we first visit the web site at `http://localhost:5000/` as you can see in the first image, there are no parameters. The params variable will be empty, the `text` field won't exists and thus the block of the `if`-statement will be skipped. Once we type in some text in the box and click on the link the HTML form will be sent to the same address, but this time the URL will also include the `?text=Hello+World!` part as you can see in the second image. This will be parsed by the `get_url_parameters` function and we will have a `text` key in the `params` variable that will contain a vector of strings. We take the first one and append it to the `html` variable with some extra text.
* The last 3 lines in the `main` function returns the html string with the header.



