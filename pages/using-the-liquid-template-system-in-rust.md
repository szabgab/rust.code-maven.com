---
title: Using the liquid template system in Rust
timestamp: 2024-06-26T07:30:01
author: szabgab
published: true
description: The liquid template system was originally developed by Shopify in Ruby, this video is about the Rust implementation of it.
tags:
    - liquid
    - template
    - live
---

{% youtube id="2fGWoo6ZXrA" %}

## An example of an email you might receive...

```
Dear {{fname}},

I am the representative of the late King of Bla. You inherited {{sum}} of money....
```

The original [Liquid in Ruby by Shopify](https://shopify.github.io/liquid/)

The [Liquid crate](https://crates.io/crates/liquid)

## My projects using Liquid

* The [Virtual events site](https://events.code-maven.com/) - [source](https://github.com/szabgab/virtual-events)
* The [Rust Maven site](https://rust.code-maven.com/) uses the [Code Maven Static Site Generator](https://ssg.code-maven.com/) - [source](https://github.com/szabgab/code-maven.rs)
* The [Rust Digger](https://rust-digger.code-maven.com/) - [source](https://github.com/szabgab/rust-digger)

## The slides I used

* [Liquid slides](https://rust.code-maven.com/slides/rust/liquid)
* [Source of the slides](https://github.com/szabgab/rust.code-maven.com/) (in the slides folder)

## Crates

* [liquid](https://crates.io/crates/liquid)
* [liquid-filter-reverse-string](https://crates.io/crates/liquid-filter-reverse-string)
* [liquid-filter-commafy](https://crates.io/crates/liquid-filter-commafy)


