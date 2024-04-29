---
title: Serialized - Deserialize enum manually
timestamp: 2024-04-29T09:30:01
author: szabgab
published: true
description: How to create an enum instance from a string and how to convert the instance to a string
tags:
    - enum
    - Debug
    - Display
    - new
todo:
    - implement this using serde or some other crate
    - list all the variants
---

How to create a new instance of an [enum](https://doc.rust-lang.org/std/keyword.enum.html) from a string and how to convert the `enum` instance to a string?


In this example we have a very simple `enum` called `Animal` that has 3 variants:

```rust
enum Animal {
    Cat,
    Dog,
    Fish,
}
```

We derived it from [Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html) using the following line in order to allow us to print the value of an instance
using the `{:?}` debugging format. (Or the `dbg!` macro for that matter.)

```rust
#[derive(Debug)]
```

So we can write this:

```rust
println!("{c:?}");
```


## Converting string to enum variant

We wanted to be able to write something like this and let it return `Animal::Cat`:

```
let c = Animal::new("cat");
```

For this we implemented the `new` method for the `Animal` enum and mapped each string to the appropriate variant.

This mapping allows us to accept alternatives. For example, I included both "dog" and "hound" to be mapped to "Animal::Dog".

As we are using `match` on an `&str` string Rust forces us to have an "arm" for each possible value, so we have to provide a catch-all
default case. We could return "Animal::Dog" or any other variant as the default, but here we elected to `panic!`.

```rust
impl Animal {
    fn new(text: &str) -> Self {
        match text {
            "cat" => Animal::Cat,
            "dog" => Animal::Dog,
            "fish"  => Animal::Fish,

            "hound" => Animal::Dog,

            // default is required
            _ => panic!("No such animal as {text:?}"),
        }
    }
}
```

## Convert enum variant to string

We also want to be able to convert the variant to a string in order to be able to use this:

```rust
println!("{c}");
```

For this we need to implement the [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html) trait
mapping each variant to the string to display:

```
impl std::fmt::Display for Animal {
    fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Animal::Cat => "cat",
            Animal::Dog => "dog",
            Animal::Fish => "fish",
        };
        write!(format, "{text}")
    }
}
```


## Full example

{% include file="examples/serialize-deserialize-enum-manually/src/main.rs" %}




