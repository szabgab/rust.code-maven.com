---
title: Generic types for function parameters in Rust
timestamp: 2023-12-19T09:43:01
published: true
description: Generics allow us to create functions that can work on more than a single fixed type of value.
tags:
    - T
    - PartialOrd
    - E0308
    - E0369
    - error code
todo:
    - Can I limit the generic type to let's say i8 and i16 ? Do we even need such a thing?
    - spearate page explainaing what does T mean
---


## Simple example of the max function

We start with a  simple function that returns the bigger of two values. Basically what the [max function](/min-max) does.

```rust
fn main() {
    let x: i8 = 25;
    let y: i8 = 42;
    println!("{}", bigger(x, y));
}

fn bigger(a: i8, b: i8) -> i8 {
    if a > b {
        a
    } else {
        b
    }
}
```

This works and prints 42 as expected.


##  Trying to use on another type

We add the following to our code:

```rust
    let x: i16 = 25;
    let y: i16 = 42;
    println!("{}", bigger(x, y));
```

So now we have:

```rust
fn main() {
    let x: i8 = 25;
    let y: i8 = 42;
    println!("{}", bigger(x, y));

    let x: i16 = 25;
    let y: i16 = 42;
    println!("{}", bigger(x, y));
}

fn bigger(a: i8, b: i8) -> i8 {
    if a > b {
        a
    } else {
        b
    }
}
```

This will give a compile time error

```
error[E0308]: arguments to this function are incorrect
  --> src/main.rs:8:20
   |
8  |     println!("{}", bigger(x, y));
   |                    ^^^^^^ -  - expected `i8`, found `i16`
   |                           |
   |                           expected `i8`, found `i16`
```

Suggesting to read further about [E0308](https://doc.rust-lang.org/error_codes/E0308.html) 👎


## Solution 1: separate, type specific functions

One solution is to have several implementations for the `bigger` function, one for each type we would like to handle.
Rust 🦀 does not allow us to defined the same function name with different signatures, so we need to create functions with different names.

```rust
fn main() {
    let x: i8 = 25;
    let y: i8 = 42;
    println!("{}", bigger_i8(x, y));

    let x: i16 = 25;
    let y: i16 = 42;
    println!("{}", bigger_i16(x, y));
}

fn bigger_i8(a: i8, b: i8) -> i8 {
    if a > b {
        a
    } else {
        b
    }
}

fn bigger_i16(a: i16, b: i16) -> i16 {
    if a > b {
        a
    } else {
        b
    }
}
```

This already looks bad, but imagine wanting to implement this to other types as well.
The body of the functions is the same so we would just duplicated the code.


## Solution with generics that almost works

```rust
fn main() {
    let x: i8 = 25;
    let y: i8 = 42;
    println!("{}", bigger(x, y));

    let x: i16 = 25;
    let y: i16 = 42;
    println!("{}", bigger(x, y));
}

fn bigger<T>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}
```

Here we have only a single function that uses the proverbial type `T`.
The letter `T` has no special meaning, it is not a reserved word. It is just the symbol commonly used to say "Type" or "generic Type".

In the definition of the function, immediately after the name of the function we declare in angle brackets the name(s) of the type(s)
we are going to use. Once we declare that we use the name `T` for a generic type we can use that type anywhere in the function.
So we use it to say both parameters (`a` and `b`) are going to be of this type `T` and that the return values is also going to be the same type `T`.

However, in the declaration we don't say what real type is this type `T`. It is a **generic type**.

```rust
fn bigger<T>(a: T, b: T) -> T {
```

To emphasize the name `T` is just the name commonly used in Rust 🦀, you could use any name there. This would work exactly the same way:

```rust
fn bigger<Bla: std::cmp::PartialOrd>(a: Bla, b: Bla) -> Bla {
```

and it would make a lot more sense to me 😁.



This code **almost works**. Which is to say it **does NOT work**. This is the error we get:


```
error[E0369]: binary operation `>` cannot be applied to type `T`
  --> src/main.rs:12:10
   |
12 |     if a > b {
   |        - ^ - T
   |        |
   |        T
   |
help: consider restricting type parameter `T`
   |
11 | fn bigger<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
   |            ++++++++++++++++++++++

```

It also points us to [E0369](https://doc.rust-lang.org/error_codes/E0369.html) 👎



## Solution

In this case the solution is to follow the instructions in the error message and add the [PartialOrd](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html)
trait to the definition of the type. So now we have this line:

```
fn bigger<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
```

This means that the function will accept any type that implements the [std::cmp::PartialOrd](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html).

That makes sense. In the function we use the greater-than operator which is implement on each type in the ParialOrd trait, so our function cannot
operate on any type that does not have this trait.

The code now looks like this:

![](examples/generic-bigger/src/main.rs)


## Conclusion

Generics are fun and there is a lot more to learn about them.



