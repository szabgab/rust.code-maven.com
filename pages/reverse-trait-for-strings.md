---
title: Implement the reverse trait for strings
timestamp: 2024-01-09T19:10:01
author: szabgab
published: true
description: We can extend the functionality of types with additional methods using traits.
tags:
    - trait
    - str
    - self
    - impl
todo:
    - implement reverse for numbers as well, just to show generics
---

I wanted to see how to add a new trait to strings. I thought reversing would be a simple way to demonstrate this. Apparently, the implementation of the string revere I used
will fail on certain Unicode strings. So there is a crate called [unicode_reverse](https://crates.io/crates/unicode_reverse) for the real solution, if you need that.

This article primarily is for demonstrating how to add the trait.

We would like to have a function that can reverse a string and then we would like to turn it into a method. First we would like to have

```rust
println!("{}", reverse_string(text));
```


and then

```rust
println!("{}", text.reverse());
```

## Create a crate

```
cargo new reverse-trait-for-strings
cd reverse-trait-for-strings
```

## print Hello World!

Edit the `src/main.rs` file:


```rust
fn main() {
    let text = "Hello, world!";
    println!("{}", text);
}
```

This works.

## Try to call the reverse method

```rust
fn main() {
    let text = "Hello, world!";
    println!("{}", text);
    println!("{}", text.reverse());
}
```

This will give an error:


**no method named `reverse` found for reference `&str` in the current scope**

## Implement the reverse_string function


```rust
fn main() {
    let text = "Hello, world!";
    println!("{}", text);

    println!("{}", reverse_string(text));
}

fn reverse_string(text: &str) -> String {
    text.chars().rev().collect::<String>()
}
```

```
cargo run

Hello, world!
!dlrow ,olleH
```

## Create the Reverse trait with a function

This is the base implementation of the `reverse` method. It defines the signature of this method.

```rust
trait Reverse {
    fn reverse(&self) -> String;
}
```

## Implement the `reverse` method for `str`

Inside we could have called the `reverse_string` function in order to avoid code duplication, but my point here is that we don't **need** that function for the trait.

```rust
impl Reverse for str {
    fn reverse(&self) -> String {
        self.chars().rev().collect::<String>()
    }
}
```

Now we can already use the `reverse` method:

```rust
fn main() {
    let text = "Hello, world!";
    println!("{}", text);
    println!("{}", text.reverse());
}
```


## The full source code

{% include file="examples/reverse-trait-for-strings/src/main.rs" %}


## Making the trait reusable

It is nice to create a trait for the crate we are working on, but it would be even better if we could make it reusable so other people on our team, in our company, or around the world could use it.
The solution is to create a [crate providing the trait](/reverse-trait-for-strings-in-a-crate).


