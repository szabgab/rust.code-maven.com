---
title: Rust error message to fix a typo
timestamp: 2023-08-30T11:52:01
description: Error messages can be annoying, but this one is a really nice recommendation of the Rust compiler to fix a typo.
tags:
    - rustc
    - error code
---

Here is one thing for which I like Rust.

Recently, as I wrote some code, I made a typo in a variable name:

![](examples/code_with_typo.rs)

Running the rust compiler on this file `rustc examples/code_with_typo.rs` I got the following error:

```
rustc examples/code_with_typo.rs
error[E0425]: cannot find value `i` in this scope
 --> examples/code_with_typo.rs:8:8
  |
8 |     if i == 0 {
  |        ^ help: a local variable with a similar name exists: `n`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0425`.
```

So somehow Rust understood that the variable `i` might somehow be releated to the existing name `n`.

Nice.

