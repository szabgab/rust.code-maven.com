---
title: Convert Markdown to HTML
timestamp: 2024-06-09T14:00:01
author: szabgab
published: true
description: A simple function using the markdown crate to convert some Markdown text to HTML
tags:
    - markdown
    - HTML
---


We are using the new incarnation of the [markdown crate](https://crates.io/crates/markdown) that has not replaced the old one yet.
The [source code](https://github.com/wooorm/markdown-rs) explains it and referres us to the version number of the most recent
release of the new crate.

{% include file="examples/markdow-to-html/Cargo.toml" %}


## The code

In the code I wrapped the parameters in a new function call. This is how I use the crate in the [Code Maven SSG](https://ssg.code-maven.com/)
the code that generates the Rust Maven web site.

Setting the `allow_dangerous_html` means we can embed HTML tags in the Markdown and they will be passed on without escaping the HTML tags.

{% include file="examples/markdow-to-html/src/main.rs" %}

## Some simple markdown file

The following file was used for the example

{% include file="examples/markdow-to-html/content.md" %}


