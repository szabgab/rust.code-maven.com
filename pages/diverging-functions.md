---
title: Diverging Functions - functions that never return
timestamp: 2023-12-06T00:30:01
description: There are some rare functions that never return, eg. they call exit() or panic!
tags:
    - exit
    - panic!
    - "!"
    - never
    - exclamation mark
    - error code
todo:
    - TODO
---

If we don't say what a function returns, it is expected to return `()`:

```rust
fn say_hi(name: &str) {
    println!("Hello {}", name);
}
```

However, there are function that never return. For example because they call the [panic!](https://doc.rust-lang.org/std/macro.panic.html) macro
or the [std::process::exit](https://doc.rust-lang.org/std/process/fn.exit.html) function.

For these the recommendation is to add an exclamation mark `!` as the designated return type.
This is also called the [never](https://doc.rust-lang.org/std/primitive.never.html) type.

```rust
fn say_hi_an_panic(name: &str) -> ! {
    println!("Hello {}", name);
    panic!("Oh my");
}
```

## A real-world use-case

When would we need to use this?

This is a simplified version of the code I used in the [resize image with error handling](/resize-image) example.

We are expecting a number on the command line that can be represented using a 32-bit unsigned integer.
We would like to provide a **usage** if the user did not supply anything or if the value could not converted to a `u32`.


![](examples/invalid-parameter/src/main.rs)

Here we have a `match` that is expected to return a 32-bit unsigned integer, a `u32`.
However, if the user supplied an invalid argument and we cannot parse and convert the input to a `u32`
we would like to print some error message, a **usage** statement showing how the user should run the program
and then we would like to exit.


```rust
    let number: u32 = match args[1].parse() {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Invalid parameter: '{}'. It must be an integer", err);
            eprintln!("Usage: {} INTEGER", &argv[0]);
            std::process::exit(1);
        }
    };
```

This code works well and even [clippy](https://doc.rust-lang.org/nightly/clippy/) is happy.

However, we have some duplicated code:

```rust
eprintln!("Usage: {} INTEGER", &args[0]);
std::process::exit(1);
````

What happens if we factor this out to a separate function?


## The code with diverging function

![](examples/invalid-parameter-usage/src/main.rs)

Here we need the exclamation mark on the `usage` function.

If we declare the `usage` function without any return value:

```rust
fn usage(name: &str) {
    eprintln!("Usage: {} INTEGER", name);
    std::process::exit(1);
}
```

We will get an error:

```
error[E0308]: mismatched types
  --> src/main.rs:14:21
   |
14 |           Err(err) => {
   |  _____________________^
15 | |             eprintln!("Invalid parameter: '{}'. It must be an integer", err);
16 | |             usage(&args[0]);
17 | |         }
   | |_________^ expected `u32`, found `()`

For more information about this error, try `rustc --explain E0308`.
```

Unfortunately the [explanation of E0308](https://doc.rust-lang.org/error_codes/E0308.html) does not describe this case and does not offer the solution that we needed here.

Luckily the [Rust users forum](https://users.rust-lang.org/) already had an answer: [Function calling other that exits, and branch type mismatch](https://users.rust-lang.org/t/function-calling-other-that-exits-and-branch-type-mismatch/11111) and the [Rust by Example](https://doc.rust-lang.org/rust-by-example/) had an example for [Diverging functions](https://doc.rust-lang.org/rust-by-example/fn/diverging.html).

## Conclusion

As I spent quite some time till I found this solution I though I should explain it. It helps me remember and maybe it will help a few other people avoid this problem or solve it faster.


