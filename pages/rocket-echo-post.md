---
title: Rocket - echo using HTTP POST - form handling
timestamp: 2024-01-02T20:30:01
published: true
description: Allowing the user to submit forms and handling them properly is the core of web application development.
tags:
    - Rocket
    - web
    - POST
---

Part of the series about the [Rocket web framework](/rocket).

In this example we show an HTML page with a form where the user can type in some text. When the submit button is pressed the text is sent to the server
as an HTTP POST request and the server sends it back in another HTML page.

Very simple, but this is the core of a lot of web applications. If we know this we can already build lots of applications.

## Dependencies

There is nothing new here compared to the previous example.

![](examples/rocket/echo-post/Cargo.toml)


## The templates

We are using Tera templates. We need two. One for the front-page showing and HTML form:

![](examples/rocket/echo-post/templates/index.html.tera)

The other one is for the page where we'll echo back the text the user sent.

![](examples/rocket/echo-post/templates/echo.html.tera)

The template files are in the `templates` folder.


## The code

![](examples/rocket/echo-post/src/main.rs)

We have two routes.

The first one mapping the `/` path to a function called `index` show the content of the `index.html.tera` template.
The [context](https://api.rocket.rs/v0.5/rocket_dyn_templates/macro.context.html) macro allows us to send in values to the template, but the front page is static. There is nothing to send in.

```rust
#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}
```

The second route is much more interesting, but we can discuss it, let's see this `struct` at the top of the file.

```rust
#[derive(FromForm)]
struct InputText<'r> {
    text: &'r str,
}
```

This defines the fields we are expecting from the form. In our form we have single field called `text`.

```
#[post("/echo", data = "<input>")]
fn echo(input: Form<InputText<'_>>) -> Template {
    println!("index stdout {:?}", input.text);

    Template::render(
        "echo",
        context! {
            text: input.text
        },
    )
}
```

This route will handle any 





## How to test a POST request in Rocket?

![](examples/rocket/echo-post/src/tests.rs)
