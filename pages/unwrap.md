---
title: unwrap, one way to handle errors in Rust
timestamp: 2024-01-08T15:50:01
published: true
description: unwrap is sweeping the error under the carpet untill the garbage burts.
tags:
    - unwrap
    - Result
    - Option
todo:
    - how to avoid unrap with clippy
---

There are several ways to handle errors, or undefined results. One of them is using `unwrap`. Of course some people will say using `unwrap` is not **handling** the error but **sweeping it under the carpet**
in a hope that the garbage won't burst through the carpet.


Some functions in Rust return a value such as a number or a string, but there are many functions that return either a [Result](https://doc.rust-lang.org/std/result/enum.Result.html) or an [Option](https://doc.rust-lang.org/std/option/enum.Option.html).

A `Result` can contain a value (e.g. a number or a string) OR an error message. It is usually returned by functions that might have an error.
For example if we would like to `parse` a string into a `u32` number. If the parsing succeeds we receive a `u32`. However it might fail if the string contains some text or a floating point number. Therefore the [parse](https://doc.rust-lang.org/std/primitive.str.html#method.parse) method returns a `Result<u32, Err>`,

An `Option` is usually returned where the function might have no good value to return.
For example we might want to `find` the location of one string in another string. If it finds the substring it return a `usize` if it does not find the substring it returns a `None`. So the [find](https://doc.rust-lang.org/std/primitive.str.html#method.find) method returns an `Option<usize>`.

For the 4 cases there are 4 functions in the following example. In two cases we try to use `unwrap`. It leads to shorter code, but if there is a failure then our code panics.

In two other cases we use `match` checking the returned value and acting according to the real value.

## Using unwrap on Result

```rust
fn unwrap_on_result(input: &str) {
    let number = input.parse::<u32>().unwrap(); // Result<u32, ParseIntError>
    println!("Input number: {number}");
}
```


If we call it `unwrap_on_result("42");` it works well and prints 42.

If we call it with `unwrap_on_result("4.2");` it panics with `called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }`.


## Using match on Result

```rust
fn match_on_result(input: &str) {
    match input.parse::<u32>() {
        Ok(number) => println!("Input number: {number}"),
        Err(err) => println!("Error: {} when trying to parse '{}'", err, input),
    };
}
```

This function can handle both "42" and "4.2", For the former it will print `Input number: 42`, for the latter it will print `Error: invalid digit found in string when trying to parse '4.2'`.

## Using unwrap on Option

```rust
fn unwrap_option(animal: &str) {
    let text = "The black cat climbed the green tree";
    let location = text.find(animal).unwrap(); // Option<usize>
    println!("Location of {animal}: {location}");
}
```

Calling `unwrap_option("cat");` will print `Location of cat: 10`. Calling `unwrap_option("dog");` will panic with the following message: `called `Option::unwrap()` on a `None` value`.


## Using match on Option

```rust
fn match_on_option(animal: &str) {
    let text = "The black cat climbed the green tree";
    match text.find(animal) {
        Some(location) => println!("Location of {animal}: {location}"),
        None => println!("None received - no {animal} found"),
    };
}
```

Calling `match_on_option("cat");` will print `Location of cat: 10`. Calling `match_on_option("dog");` will print `None received - no dog found`.


## The full code

![](examples/unwrap/src/main.rs)

## Comments

The functions with `unwrap` are shorter, but they might cause panic.

The functions with `match` were just printing some message. In real code they will be probably longer than here.

Besides `unwrap` and using `match` there are many other ways to [handle a Result](https://doc.rust-lang.org/std/result/index.html) and to [handle an Option](https://doc.rust-lang.org/std/option/) that we will
probably discuss in separate posts.




