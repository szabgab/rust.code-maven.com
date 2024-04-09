---
title: Guessing MIME-Type, setting Content-Type based on file extension
timestamp: 2024-04-09T14:30:01
author: szabgab
published: true
description: In many cases the extension of the file will dictate the content-type one needs to set when returning in a web application.
tags:
    - mime_guess
    - MIME
    - Content-Type
    - first
---

One of the first web "applications" I wrote in Rust was the [rustatic](https://rustatic.code-maven.com/) a small program that helps me
to run a local web server to look at a static web site. A problem recently encountered was that it sent back every file as HTML.
This broke the AJAX-using JavaScript code I wrote. It did not know what to do with the JSON file that was sent to the client as `text/html`.

At first I patched the code mapping the json and js extensions to their repsective `MIME-Type` so the application will set the `Content-Type` properly,
but then I thougt, surely there is at leas one crate that will do the mapping for me.

That's how I found [mime_guess](https://crates.io/crates/mime_guess).

Here is a small example I put together for myself:


## The dependencies in Cargo.toml

{% include file="examples/guessing-mime-type/Cargo.toml" %}


## The source code

{% include file="examples/guessing-mime-type/src/main.rs" %}


## Explanation

The `first` method will return an [Option](https://doc.rust-lang.org/std/option/enum.Option.html).

If the extension is not recognized, or if there is no extension then it will return `None`.

Otherwise it can be stringified to what needs to be the `Content-Type`.

