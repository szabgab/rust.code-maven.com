---
title: Prompt - read input from Standard Input (STDIN) in Rust
timestamp: 2023-12-31T18:20:01
published: true
description: Python has the input function, other languages have a prompt function. This is how we get some input from the user in Rust.
tags:
    - prompt
    - STDIN
    - STDOUT
    - flush
    - read_line
    - trim_end
    - print!
    - println!
    - lines
    - Write
---

Python has the **input** function, other languages have a **prompt** function. This is how we get some input from the user in Rust.

There are 3 solutions on this page. The first two are simpler but they would `panic!` if there is a problem printing to STDOUT or reading from STDIN.
The 3rd solution handles these case and returns a `Result` in case of any error the caller will have to handle. Use the one that feels appropriate to your situation.

## prompt with read_line

![](examples/prompt/src/main.rs)

Our `prompt` function expects a string as a parameter.

It prints it to the screen. We want to let the user type her answer on the same line where the question was, so instead of `println!`
we use here `print!` that will not add a newline to the output.

Because of that just calling `print!` is not enough.

By default the Standard Output (STDOUT) channel is buffered. The buffer is flushed when we print a newline, but because in this case we don't print a newline
we'll need to call `flush` ourselves. This is why we needed the [std::io::Write](https://doc.rust-lang.org/std/io/trait.Write.html) trait:

```rust
use std::io::Write;
```

and this is why we call to actually flush out the buffer to the screen:

```rust
std::io::stdout().flush().expect("Oups");
```


The reading in from the STDIN is done by these lines:

```rust
let mut response = String::new();
std::io::stdin()
    .read_line(&mut response)
    .expect("Failed to get input");
```

Finally we remove the trailing newline that was added when user pressed ENTER.

```rust
response.trim_end().to_string()
```

## prompt calling lines

An alternative would be the use of the `lines` iterator and getting the first element:


```rust
std::io::stdin().lines().next().unwrap().unwrap()
```

![](examples/prompt2/src/main.rs)

## prompt returning Result


![](examples/prompt-result/src/main.rs)

In this solution the `prompt` function returns `Result<String, std::io::Error>`.

That it, it either returns the string the user typed in or an [std::io::Error](https://doc.rust-lang.org/std/io/struct.Error.html).

The caller has to handle this somehow. For example by using `match` and handling the `Ok` and `Err` cases separately.

Inside the `prompt` function we don't have any explicit error handling. We just added `?` at the end of the calls that might fail.
This means if either of those calls fail, the `prompt` function immediately returns with the same error.

If the calls succeed then the prompt function just goes on.

## Cargo clippy to avoid using expect or unwrap

Clippy is an excellent linter for Rust. Already out of the box it finds lots of things to fix in the code, but it is also configurable.
For example we can tell Clippy that we want to make sure our code does not use `expect` or `unwrap` anywhere.

We can do that by adding a few lines to the `Cargo.toml` file:

![](examples/prompt-result/Cargo.toml)

In this 3rd example `cargo clippy` will run clean, but if we do the same in the other two examples `cargo clippy` will fail.
