---
title: Rocket - echo using HTTP POST - form handling
timestamp: 2024-01-02T20:30:01
author: szabgab
published: true
description: Allowing the user to submit forms and handling them properly is the core of web application development.
tags:
    - Rocket
    - web
    - POST
    - Form
    - Template
    - header
    - body
    - ContentType::Form
    - Status::UnprocessableEntity
    - 422
---

Part of the series about the [Rocket web framework](/rocket).

In this example we show an HTML page with a form where the user can type in some text. When the submit button is pressed the text is sent to the server
as an HTTP POST request and the server sends it back in another HTML page.

Very simple, but this is the core of a lot of web applications. If we know this we can already build lots of applications.

## Dependencies

There is nothing new here compared to the previous example.

{% include file="examples/rocket/echo-post/Cargo.toml" %}


## The templates

We are using Tera templates. We need two. One for the front-page showing and HTML form:

{% include file="examples/rocket/echo-post/templates/index.html.tera" %}

The other one is for the page where we'll echo back the text the user sent. It has a single placeholder for a field called `text`.

{% include file="examples/rocket/echo-post/templates/echo.html.tera" %}

The template files are in the `templates` folder.


## The code

{% include file="examples/rocket/echo-post/src/main.rs" %}

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

Sending a POST-request with the text field and the server returns the text we saw on the web page:

```
$ curl -X POST -d "text=Hello World!" -i http://localhost:8000/echo

HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
server: Rocket
x-content-type-options: nosniff
permissions-policy: interest-cohort=()
x-frame-options: SAMEORIGIN
content-length: 32
date: Fri, 05 Jan 2024 08:22:43 GMT

You typed in <b>Hello World!</b>
```


Submitting the form without the `text` field gives us a 415 HTTP error.

```
$ curl -X POST -i http://localhost:8000/echo

HTTP/1.1 415 Unsupported Media Type
content-type: text/html; charset=utf-8
server: Rocket
x-content-type-options: nosniff
permissions-policy: interest-cohort=()
x-frame-options: SAMEORIGIN
content-length: 501
date: Fri, 05 Jan 2024 08:21:22 GMT

<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="color-scheme" content="light dark">
    <title>415 Unsupported Media Type</title>
</head>
<body align="center">
    <div role="main" align="center">
        <h1>415: Unsupported Media Type</h1>
        <p>The request entity has a media type which the server or resource does not support.</p>
        <hr />
    </div>
    <div role="contentinfo" align="center">
        <small>Rocket</small>
    </div>
</body>
</html>
```




Sending a GET request to the `/echo` path will result in a 404 Not Found error. This is not surprising. We have only defined the POST method for the `/echo` path.


```
$ curl -i http://localhost:8000/echo

HTTP/1.1 404 Not Found
content-type: text/html; charset=utf-8
server: Rocket
x-content-type-options: nosniff
permissions-policy: interest-cohort=()
x-frame-options: SAMEORIGIN
content-length: 435
date: Fri, 05 Jan 2024 08:19:53 GMT

<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="color-scheme" content="light dark">
    <title>404 Not Found</title>
</head>
<body align="center">
    <div role="main" align="center">
        <h1>404: Not Found</h1>
        <p>The requested resource could not be found.</p>
        <hr />
    </div>
    <div role="contentinfo" align="center">
        <small>Rocket</small>
    </div>
</body>
</html>
```


## How to test a POST request in Rocket?

Using `curl` is a good way to manually check what the code does, but it is better to have tests that can be execute automatically.

Here are 3 test functions.

The first one checks the main page. It does not check if the returned HTML has some exact content.
That's usually, especially in real applications where the UI changes frequently, is not feasible. We only check if certain elements
appear in the HTML that was returned by the server.

The second function submits a `post` request. It needs the `header` to be set to `ContentType::Form` and we pass the valuse in the `body`.
Here we tested if the returned HTML is exactly as expected, though in a real application we would probably only test if the expected
text is part of the HTML.

Finally we have a test where we send in a POST request but leave out the `body` so there won't be a `text` field.

We verify that the status was set to [Status::UnprocessableEntity](https://api.rocket.rs/v0.5/rocket/http/struct.Status.html#associatedconstant.UnprocessableEntity).
I find it important to have test cases for these automatically generated responses, to protect ourselves from any unplanned changes to the way these cases
are handled.

{% include file="examples/rocket/echo-post/src/tests.rs" %}



