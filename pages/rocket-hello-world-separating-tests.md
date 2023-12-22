---
title: "Rocket: Hello World with tests in separate file"
timestamp: 2023-12-22T16:40:01
published: true
description: It is better to put the tests in separate files.
tags:
    - Rocker
    - web
    - tests
---

Part of the series about the [Rocket web framework](/rocket).

In the previous article where we saw how to write [Hello World in Web Rocket](/rocket-hello-world) we also included some test.
It was more straight forward to include the tests in the `main.rs`, but in reality I think it is better to have them in separate files.

So in this article we see and example how to do it.

## Dependencies

![](examples/rocket/hello-world-external-test-file/Cargo.toml)

These are the same as in the previous version.

## The tests


```rust
#[cfg(test)]
mod test {
    ...
}
```

The tests from inside the `mod test` were moved to a separate file called `src/tests.rs`.



![](examples/rocket/hello-world-external-test-file/src/tests.rs)


## The code

The curly braces were also removed so this is the code:


![](examples/rocket/hello-world-external-test-file/src/main.rs)

