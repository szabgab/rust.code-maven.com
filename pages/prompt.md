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
    - trimp_end
    - print!
    - println!
    - lines
---

Python has the **input** function, other languages have a **prompt** function. This is how we get some input from the user in Rust.

![](examples/prompt/src/main.rs)

Our `prompt` function expects a string as a parameter.

It prints it to the screen. We want to let the user type her answer on the same line where the question was, so instead of `println!`
we use here `print!` that will not add a newline to the output.

Becuase of that just calling `print!` is not enough.

By default the Standard Output (STDOUT) channel is buffered. The buffer is flushed when we print a newline, but because in this case we don't print a newline
we'll need to call `flush` ourselves. This is why we needed

```rust
use std::io::Write;
```

and this is why we call to actaully flush out the buffer to the screen:

```rust
std::io::stdout().flush().expect("Oups");
```


The reading in from the STDIN is done by these lines:

```rust
let mut response = String::new();
std::io::stdin()
    .read_line(&mut response)
    .expect("Faild to get input");
```

Finally we remove the trailing newline that was added when user pressed ENTER.

```rust
response.trim_end().to_string()
```

## Alternative reading of STDIN

An alternative would be the use of the `lines` iterator and getting the first element:


```rust
std::io::stdin().lines().next().unwrap().unwrap()
```

