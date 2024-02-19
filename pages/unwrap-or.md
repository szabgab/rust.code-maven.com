---
title: unwrap, unwrap_or, unwrap_or_default, unwrap_or_else
timestamp: 2024-01-17T16:10:01
published: true
description: Several ways to deal with functions that fail in Rust. The alternatives of calling unwrap.
tags:
    - Result
    - match
    - unwrap
    - unwrap_or
    - unwrap_or_default
    - unwrap_or_else
---

Out of the many ways to handle errors in Rust we have already seen [unwrap](/unwrap), but it has a number of better alternatives that would avoid the
`panic!` if there was an error.


In this example we have a string that we are trying to convert to `u32` number using the [parse](https://doc.rust-lang.org/std/primitive.str.html#method.parse) method
that returns a [Result](https://doc.rust-lang.org/std/result/enum.Result.html) instance. This `Result` either contains


## using match

Probably the most verbose way to handle a `Result` is by using `match`. Here if the operation succeeded the result will be in `val` and first block will be executed.
If the operation fails we get back an `Err` that will be in the `err` variable.

```rust
match input.parse::<u32>() {
    Ok(val) => {
        println!("{}", val);
    },
    Err(err) => {
        eprintln!("Error: {}", err);
    }
};
```

## unwrap, unwrap_or, unwrap_default, unwrap_or_else

With each method, if the operation succeeds we get back the result. They differ in the case when there is an error.


## unwrap

```rust
let number = input.parse::<u32>().unwrap();
```

If the operation fails we get a `panic!`:

```
called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }
```

## unwrap_or


```rust
let number = input.parse::<u32>().unwrap_or(23);
```

If the operation fails we get the value provided there. In this case the arbitrary value of 23.


## unwrap_or_default

```rust
let number = input.parse::<u32>().unwrap_or_default();
```

If the operation fails we get back the default value of the expected type. For `u32` it is 0.

## unwrap_or_else

```rust
let number = input.parse::<u32>().unwrap_or_else(|err| {
    eprintln!("{}", err);
    12
});
```

If the operation fails Rust will call the provided function passing the error to it.
The function can do anything and whatever it returns will be returned to the called.
In our example it is 12.


## The full example


{% include file="examples/unwrap-or/src/main.rs" %}

## Conclusion

Which one you use will depend on the situation.


