---
title: Getting started with Loco, the one-person web framework for Rust
timestamp: 2023-12-03T17:30:01
description: Loco is a Rails-like web framework in the making. Let's try it.
tags:
    - Loco
    - web
    - GET
    - POST
    - HTML
    - JSON
---

[Loco](https://loco.rs/) is called the one-person framework for Rust for side-projects and startups. I saw it mentioned in
on of the local Rust groups, so I thought I give it a try. I am using **loco-cli 0.1.3**. By the time you are reading this,
there might be newer versions with other features.

The tour on the web site shows and applications using Postgres, but I don't have it installed, so I will try the simpler version
that does not need a database.

Before we get started let me mention also that in the [GitHub repository](https://github.com/loco-rs/loco) of the project there is a
folder called `examples` where you can find two examples.


## Install Loco

```
cargo install loco-cli
```

## Set up first project

If you don't have one yet, create a folder to hold all of your projects. (I have a folder called `~/work`)

```
mkdir myprojects
cd myprojects
```

Create a new application by running:

```
loco new
```

This will ask for the name of the project (defaulting to myapp) and then it will create a folder using that name.

This will also ask for the type of the project offering you two choices:

* Saas app (with DB and user auth)
* Stateless service (minimal, no db)

For simplicity I went with the default name and picked the second choice that does not need a database.

It created a folder called `myapp` that was an initialized git repository of a Rust Crate with many extra files.


```
$ tree
.
├── Cargo.toml
├── config
│   ├── development.yaml
│   ├── production.yaml
│   └── test.yaml
├── README.md
├── src
│   ├── app.rs
│   ├── bin
│   │   └── main.rs
│   ├── controllers
│   │   ├── foo.rs
│   │   └── mod.rs
│   ├── lib.rs
│   ├── mailers
│   │   ├── auth
│   │   │   ├── forgot
│   │   │   │   ├── html.ejs
│   │   │   │   ├── subject.ejs
│   │   │   │   └── text.ejs
│   │   │   └── welcome
│   │   │       ├── html.t
│   │   │       ├── subject.t
│   │   │       └── text.t
│   │   ├── auth.rs
│   │   └── mod.rs
│   ├── tasks
│   │   ├── example.rs
│   │   └── mod.rs
│   ├── views
│   │   └── mod.rs
│   └── workers
│       ├── downloader.rs
│       └── mod.rs
├── tests
│   ├── mod.rs
│   └── requests
│       ├── foo.rs
│       ├── mod.rs
│       └── snapshots
│           └── can_print_echo.snap
└── test.sh

15 directories, 28 files
```

## Starting the development server

```
cargo loco start
```

This will install a bunch of crates and start the web application on port 3000

Visiting http://localhost:3000 all I could see was the text "Loco".


Searching for that string revealed it appears in 4 places:

```
$ ack Loco

src/controllers/foo.rs
12:    format::text("Loco")

tests/requests/foo.rs
22:        assert_eq!(response.text(), "Loco");
36:            .json(&serde_json::json!({"site": "Loco"}))

tests/requests/snapshots/can_print_echo.snap
7:    "{\"site\":\"Loco\"}",
```

I guess the one in the controllers folder is the one that is being served and the two copies in the `tests` folder are tests.

## Testing

We can run the tests with the following command:

```
cargo test
```

## Main route

In the `src/controllers/foo.rs` file there are 2 functions that are mapped to 2 routes.

This is the `index` function that just returns the text "Loco".

```rust
async fn index(State(_ctx): State<AppContext>) -> Result<String> {
    format::text("Loco")
}
```

This is the `echo` function that returns the content of the requests.

```
pub async fn echo(req_body: String) -> String {
    req_body
}
```

This is how we map two paths (two routes) to the functions that were defined earlier.

```rust
pub fn routes() -> Routes {
    Routes::new().add("/", get(index)).add("/echo", post(echo))
}
```

The following maps the `/` path using the GET HTTP method to the `index` function


```
add("/", get(index)).
```

This one maps the `/echo` path using the POST HTTP method to the `echo` function.

```
add("/echo", post(echo))
```

## Using Curl we can try both:


```
curl -i http://localhost:3000/
```

and:

```
curl -i -X POST -d "Hello Rust" http://localhost:3000/echo
```

The `-i` flag was used to also print the header. It reveals that both responses send with `content-type` `text/plain` and not `text/html`.


## Change the echo function to reverse the string

Given that we have the string `req_body` we can apply transformations to that string, for example we can reverse the string:


```rust
pub async fn echo(req_body: String) -> String {
    req_body.chars().rev().collect()
}
```

In order for these changes to take effect we have to stop the development server using Ctrl-C and then start it again with

```
cargo loco start
```

## Create an API, return JSON

If you'd like to create a web application that has an API, you need to be able to return JSON strings.
Here is a simple examples:

In Cargo.toml, change the dependency on `serde` to include the `derive` feature as well:

```
serde = { version = "1.0", features = ["derive"] }
```

At the top of the `src/controllers/foo.rs` file include

```rust
use axum::Json;
```


Create a struct that will hold the data we will want to return. In this case I just continued the echo example
and created a struct that will hold the original text and the reversed text. We also set the `derive` attribute
to `Serialize` the struct.

```rust
#[derive(Serialize)]
pub struct Echo {
    text: String,
    reversed: String,
}
```

We need to implement a function that will create the struct and call `format::json` to serialize it and return it.
The name of the function is not very original, I just called it api. The return value of the function is a Result
including the Json version of the Echo struct.


```
pub async fn api(req_body: String) -> Result<Json<Echo>>  {
    let data = Echo {
        text: req_body.clone(),
        reversed: req_body.chars().rev().collect(),
    };
    format::json(data)
}
```

Finally I had to update the route, adding a mapping of `/api` path to the `api` function:

```rust
add("/api", post(api))
```

Running `cargo fmt` on the code made it much more readable:


```rust
pub fn routes() -> Routes {
    Routes::new()
        .add("/", get(index))
        .add("/echo", post(echo))
        .add("/api", post(api))
}
```


At this point I could stop and restart the development server and then I could use curl to try the new code.

We got back the json string and the content-type was set to `application/json` as I expected.

```
http://localhost:3000/api


HTTP/1.1 200 OK
content-type: application/json
content-length: 47
date: Sun, 03 Dec 2023 12:31:17 GMT

{"text":"Hello World","reversed":"dlroW olleH"}
```

## Change the main route to return HTML

Finally, let's see how can we return an HTML page and how can we set the <b>Content-Type</b> to be <b>text/html</b>?


At the top of the controller file include the following:

```rust
use axum::{
    response::Html,
    response::Response
};
```

Change the return value of the function to be Result, and use the `Html::from(html).into_response()`
call to return the HTML setting the content type.

```rust
async fn index(State(_ctx): State<AppContext>) -> Response {
    let html = r#"Hello <b>Loco</b>"#;
    Html::from(html).into_response()
}
```


We can use Curl again to verify the content type:


```
$ curl -i http://localhost:3000/

HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 17
date: Sun, 03 Dec 2023 13:10:57 GMT

Hello <b>Loco</b>
```

## Autoreload

When developing a web application it is quite annoying if you have to keep stopping the application with Ctrl-C and restarting it manually.
Luckily, as pointed out by [Elad Kaplan](https://github.com/kaplanelad) the lead developer of Loco, one can use [cargo-watch](https://crates.io/crates/cargo-watch):


Install it:

```
cargo install cargo-watch
```

Run the development server this way:

```
cargo-watch -x check  -s 'cargo loco start'
```

Due to the compilation time of Rust each restart still takes a few second, but at least I don't have to do it manually.

## Conclusion

This is a promising web framework, we'll see if it catches on.

Oh, and it took a while and a picture shared by the authors, to realize that the name "Loco" refers to locomotive (as in train, rails...) and not crazy in Spanish...



