---
title: Turbofish ::<> 🐠
timestamp: 2023-12-13T11:45:01
published: true
description: Turbofish is the name of the syntax that allows us to define a type in the function call.
tags:
    - Turbofish
    - "::<>"
    - parse
    - collect
---

In some code you will see a construct that looks like this:

```rust
parse::<i32>()
```

or this

```rust
collect::<Vec<&str>>()
```

or maybe this:

```rust
collect::<Vec<_>>()
```

These all use the **Turbofish** syntax **::<>**.

## When is it used?

In some cases Rust can deduce the type of the variable automatically, in other cases we explicitly have to say what is the type
of a value and of a variable.

For example when we want to convert a string to a number and call `parse` on the string, it has to know what kind of number are we expecting.
An `i16`, an `f64`?

When taking an iterator and calling `collect` we need to somehow tell Rust what kind of values we are expecting in the resulting vector.

We can do this in the "traditional" way of declaring the type of the variable:

```rust
let val: i32 = input.parse().unwrap();
```

This looks strange to me as this means that the `parse` method has to understand what is expected from it and do its magic based on that.
A more **natural** looking way to write this is as follows:

```rust
let val = input.parse::<i32>().unwrap();
```

In this case we directly told `parse` that we want it to convert the string to an `i32`.


Another case when we have an iterator (e.g. one that `split` returned) and we would like to go over all the elements and `collect` them into a vector.
We either declare the new variable as being a vector:

```rust
let parts: Vec<&str> = text.split(',').collect();
```

or we can use the **turbofish** syntax to tell our expectation to the `collect` method:

```rust
let parts = text.split(',').collect::<Vec<&str>>();
```

In the case of collecting values from an iterator we can even use underscore `_` instead of the internal type and Rust will figure it out.

```rust
let parts: Vec<_> = text.split(',').collect();
let parts = text.split(',').collect::<Vec<_>>();
```

## Full example


![](examples/turbofish/src/main.rs)

## Note

Actually the idea that a function (or method) know what is expected from it and works based on the type of the expected return value is not new to me.
Perl has a similar idea where functions know if we are expecting a single value (a scalar) or multiple values (a list), or maybe nothing (void)
and can work according to that. Some functions even go further and know if we are expecting a boolean value, a number, or a string.

It is called **context** in Perl and in most cases it is rather implicit making it a bit difficult to get used to.

In Rust the closest you might get to be implicit is when you use `_` as part of the type-definition, but even then Rust can deduce the expected type at compile time,
and if you use the [Rust analyzer](https://rust-analyzer.github.io/) in your editor or IDE then it can show you the type even earlier, at "edit-time".

## Origin

See the [Turbofish swimming](https://turbo.fish/) and make sure you look at the [about Turbofish](https://turbo.fish/about) page.

