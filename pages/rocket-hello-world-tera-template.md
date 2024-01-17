---
title: Rocket Hello World with Tera Templates
timestamp: 2024-01-02T19:20:01
author: szabgab
published: true
description: The Rocket web framework has integration with the Tera template system
tags:
    - Rocket
    - web
    - Tera
    - rocket_dyn_templates
    - Template
    - context!
    - render
    - attach
    - fairing
---

Part of the series about the [Rocket web framework](/rocket).

Rocket is integrated with two templating systems. In this article we'll see how to use the [Tera](https://crates.io/crates/tera) templating system.

## Dependencies

For this to work, we'll need  the [rocket_dyn_templates](https://crates.io/crates/rocket_dyn_templates) crate with the **tera** feature.

This is how `Cargo.toml` looks like:

![](examples/rocket/hello-world-tera-template/Cargo.toml)

## Template

The template itself has to be placed in the `templates` folder of our crate and it needs to have the `.html.tera` extension.
In this example we use some minimal HTML and a single template variable that we want to substitute. We need that double curly braces around it.

![](examples/rocket/hello-world-tera-template/templates/index.html.tera)


## The code

![](examples/rocket/hello-world-tera-template/src/main.rs)

So what are the interesting parts here?

We need to import both the [Template](https://api.rocket.rs/v0.5/rocket_dyn_templates/struct.Template.html) and the [context](https://api.rocket.rs/v0.5/rocket_dyn_templates/macro.context.html):

```rust
use rocket_dyn_templates::{context, Template};
```

Our route is expected to return a `Template` so we have:

```rust
fn index() -> Template {
```

In the route itself we can pass values for the placeholders of the template using the `context!` macro and we call the [render](https://api.rocket.rs/v0.5/rocket_dyn_templates/struct.Template.html#method.render) method.

```rust
Template::render("index", context! {
    name: "Rocket with Tera"
})
```

Finally, we had to [attach](https://api.rocket.rs/v0.5/rocket/struct.Rocket.html#method.attach) the [Template::fairing](https://api.rocket.rs/v0.5/rocket_dyn_templates/struct.Template.html#method.fairing).

```rust
.attach(Template::fairing())
```

I admit, I haven't heard this word before and I don't really know what it means, but without that Rocket complained:

```
Error: returning `Template` responder without attaching `Template::fairing()`.
   >> To use or query templates, you must attach `Template::fairing()`.
   >> See the `Template` documentation for more information.
```

## Testing

There is nothing new in the test compared to the previous cases. We only changed the expected text.
We still verify that the `Content-Type` is `text/html`.


![](examples/rocket/hello-world-tera-template/src/tests.rs)

```
cargo test
```


## Running


```
cargo run
```


