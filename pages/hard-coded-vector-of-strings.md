---
title: Hard-coded vector of Strings in Rust
timestamp: 2024-01-13T18:50:01
author: szabgab
published: true
description: Sometimes, especially for examples, I need a vector of hard coded values that resemble values I read from a file. So String types.
tags:
    - vec!
    - String
    - "&str"
    - into_iter
    - map
    - collect
---


It is easy to create a vector of hard-coded `&str` strings, but sometimes, especially for examples, I need a vector of hard-coded `String` values.
I need that in order to demonstrate how to deal with a vector of values I read from a file that are going to be `String` values.

There are two options. Either create a vector and call `to_string` manually on each one of the elements, or uses an iterator and call `to_string` on each one of them.

## Create a vector of str: `Vec<&str>`
```
let colors = vec!["blue", "red", "green", "yellow"];
```

## Create a vector of String: `Vec<String>` using `to_string` on each on of them

```rust
let colors = vec![
    "blue".to_string(),
    "red".to_string(),
    "green".to_string(),
    "yellow".to_string(),
];
```

## Create a vector of String: `Vec<String>` using `to_string` in a map

```
let colors = vec!["blue", "red", "green", "yellow"]
    .into_iter()
    .map(|str| str.to_string())
    .collect::<Vec<String>>();
```

Using the [Turbofish](/turbofish) syntax to let `collect` know the type of the items.

## The full code

![](examples/hard-coded-vector-of-strings/src/main.rs)

# The output is the same in each case:

```
["blue", "red", "green", "yellow"]
["blue", "red", "green", "yellow"]
["blue", "red", "green", "yellow"]
```


