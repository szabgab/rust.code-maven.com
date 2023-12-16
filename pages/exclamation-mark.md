---
title: Exclamation mark in Rust !
timestamp: 2023-12-06T01:30:01
published: true
description: There are a number of places where exclamation marks are used in Rust
tags:
    - exclamation mark
    - "!"
    - macro
    - not
todo:
    - "!="
---

There are a number of places where we can see exclamation marks `!` in Rust.


## Never type

As the designated return type of a function:

```rust
fn foo() -> ! {
    ...
}
```

[Diverging Functions - functions that never return](/diverging-functions)


## Macros


At the end of "function" names:

```rust
println!("Hello World");
```

[Functions that end with an exclamation mark !](/functions-that-end-with-exclamation-mark)

## Boolean not

In boolean expressions:

```rust
if ! var.is_empty() {
   ...
}
```

[Boolean not !](/boolean-not)


## Not equal !=

Comparing values for lack of equality:

```rust
if answer != 42 {
    ...
}
```


