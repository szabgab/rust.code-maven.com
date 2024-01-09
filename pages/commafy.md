---
title: Commafy - add a comma after every 3 digits
timestamp: 2023-12-06T18:30:01
published: true
description: "A number like this: 1234567890 is quite unreadable 1,234,567,890 is much better."
tags:
    - commafy
    - assert_eq!
    - into
    - abs
    - thousands
todo:
    - release as a crate
    - use this crate in the liquid commafy example
---


A number like this: 1234567890 is quite unreadable 1,234,567,890 is much better.

Converting a number to this more readable form is sometimes called **commafy**.
Here is an implementation:

![](examples/commafy/src/main.rs)

* We have function called `get_command_line_number` taken from the example [expecting one command line parameter](/expect-one-command-line-parameter).
* We also have a `main` function to make it easier to try the code on the command line.
* We also have a bunch of tests at the bottom of the file. We can run the tests by typing in `cargo test`.
    In each test-case we call the `commafy` function and then compare the resulting string with the expected string on the right hand side.
    We use the `assert_eq!` functions for this.

* The real code is in the `commafy` function.

## Explanation

```rust
number: Integer
```
means the function is expecting a single parameter of type `Integer`.
This name refers to any type that fulfills the requirements in the declaration of the type
right after the name of the function:

```rust
Integer: Into<i128> + Copy + std::fmt::Debug + std::fmt::Display
```

This means that the newly created `Integer` type:
* Must be convertible to `i128` without loss. Basically every integer type and unsigned-integer type fulfills this, except `u128`.
* Has to have the [Copy trait](https://doc.rust-lang.org/std/marker/trait.Copy.html).
* Has the [Debug trait](https://doc.rust-lang.org/std/fmt/trait.Debug.html).
* And the [Display trait](https://doc.rust-lang.org/std/fmt/trait.Display.html).

The function will return a [String](https://doc.rust-lang.org/std/string/struct.String.html).

We only want to work with non-negative numbers, so we work with the absolute value, but the Integer type we created
does not have the `abs` methods so first we need to call the `into` method to convert the value to `i128`.


```rust
let num = number.into().abs();
```

We actually want to work with characters, disregarding the numerical values so we convert the number into a string.

```rust
let num = format!("{num}");
```

It would be hard to add a comma after every 3 character from the right side (the back) of the string, so we split it up into individual characters
(using the [chars](https://doc.rust-lang.org/std/primitive.str.html#method.chars) method,
then reverse the characters using the [rev](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.rev) method.


Then using [map](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map) we go over each character and add
a comma (`,`) after each 3rd character. Actually, we add a comma **in-front** the character whose index is 1 modulo 3 (`ix % 3 == 1`).
This way we can make sure that we convert `123456` to `123,456` and not to `,123,456`.

The we call [collect](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect) and provide the type using the turbofish syntax:
```::<String>`.

The result is a string with all the commas.

The prefix is the `-` for the negative numbers. It is really nice of Rust that `if` returns the expression in it.

```
let prefix = if number.into() < 0 { "-" } else { "" };
```

Finally we need to reverse the string again to get in the correct direction.

```
format!("{}{}", prefix, num.chars().rev().collect::<String>())
```

* [as_bytes](https://doc.rust-lang.org/std/string/struct.String.html#method.as_bytes)


## The thousands crate

After implementing this someone pointed me to the [thousands crate](https://crates.io/crates/thousands) that does this already.


