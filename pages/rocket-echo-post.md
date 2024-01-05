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

The other one is for the page where we'll echo back the text the user sent. It has a single placeholder for a field called `text`.

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

This route will handle any POST request sent to the `/echo` path. The fields that the client sends in are going to be deserialized into
an `InputText` struct. Because that struct has a field called `text` the client is expected to send in a form with a field called `text`.
The value of the field can be any text. The struct is then passed in as the variable `input`.

Then we take the `input.text` field and send it to the template to fill the field `text` via the `context!`.

## How to run it?

```
cargo run
```

Then we can visit http://localhost:8000/ where we'll see the initial page. It has two forms on it. One hase a text field and a submit button.
The other only a submit button. The latter is so we can see what happens if a client send a POST request without the required `text` field:

![](images/post-echo-rocket.png)

If we fill the textbox with "Hello World!" and click on the submit button we get back the following content: "You typed in **Hello World!**"

If we click on the "back" button of the browser and submit the "Bad form" we get a page with the following text:

```
422: Unprocessable Entity

The request was well-formed but was unable to be followed due to semantic errors.
```

## Verify using curl

We can open another terminal and if we have `curl` installed we can check the site:

The main page:

```
$ curl -i http://localhost:8000

HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
server: Rocket
x-content-type-options: nosniff
permissions-policy: interest-cohort=()
x-frame-options: SAMEORIGIN
content-length: 236
date: Fri, 05 Jan 2024 08:18:58 GMT

<h1>Echo</h1>
<form method="POST" action="/echo">
<input name="text">
<input type="submit" value="Echo">
</form>


<h1>Bad form</h1>
Missing the text field.
<form method="POST" action="/echo">
<input type="submit" value="Echo">
</form>
```






## How to test a POST request in Rocket?

![](examples/rocket/echo-post/src/tests.rs)


