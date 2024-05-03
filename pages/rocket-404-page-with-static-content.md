---
title: "Rocket: 404 page with static content"
timestamp: 2024-05-02T18:30:01
author: szabgab
published: true
description:
tags:
    - Rocket
    - 404
    - content
    - RawHtml
    - routes!
    - catchers!
    - mount
    - register
    - include_str!
---

If the user visits a path on our [Rocket](/rocket)-based site that does not match any of the routes, by default, Rocket will show a very simple page saying **404: Not Found**
and **The requested resource could not be found.**.

You might want to have a more fancy page.

In this example you can see how to do that.


## Dependencies

We only need [Rocket](/rocket) for this.

{% include file="examples/rocket/http-404-page-with-static-content/Cargo.toml" %}

## The HTML

We could embed the HTML in our Rust file, but it is better to have it in a separate file with `html` extension. That will allow a designer
to make it nice without any interaction with Ruts.

Se we have it in a separate file.

Note: I am not a designer.

{% include file="examples/rocket/http-404-page-with-static-content/src/templates/404.html" %}

## The main code

{% include file="examples/rocket/http-404-page-with-static-content/src/main.rs" %}

## The test

{% include file="examples/rocket/http-404-page-with-static-content/src/tests.rs" %}


## Explanation

We implement the function that will be called by Rocket when all routes were tried and none of them matched.
It looks exactly like a regular route, but it is marked with `catch(404)`.

```rust
#[catch(404)]
fn not_found() -> RawHtml<&'static str> {
    const BODY: &str = include_str!("templates/404.html");
    RawHtml(BODY)
}
```

The [include_str!](https://doc.rust-lang.org/std/macro.include_str.html) embeds the external file into the compiled binary during
compilation.


We also need to register the catchers using the [register](https://api.rocket.rs/v0.5/rocket/struct.Rocket#method.register) method
and the [catchers!](https://api.rocket.rs/v0.5/rocket/macro.catchers) macro:

```rust
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .register("/", catchers![not_found])
}

```

In the tests we can verify that the response status for the main route is [Ok](https://api.rocket.rs/v0.5/rocket/http/struct.Status#associatedconstant.Ok)
and for an arbitrary other path is [NotFound](https://api.rocket.rs/v0.5/rocket/http/struct.Status#associatedconstant.NotFound).

In the main route we checked that the returned page is exactly as we expect.

In the 404 route we demonstrate how to check that some content is in the returned HTML.



