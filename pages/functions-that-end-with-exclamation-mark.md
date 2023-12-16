---
title: Functions that end with an exclamation mark in Rust !
timestamp: 2023-11-30T09:30:01
published: true
description: There are a number of function-like calls in Rust that end by an exclamation mark. What are those?
tags:
    - exclamation mark
    - "!"
    - macro
todo:
    - Examples writing macros
---

As you read Rust code you will see many function calls that have an exclamation mark `!` at the end.

Some of the most common ones are:

* [print!](https://doc.rust-lang.org/std/macro.print.html)
* [println!](https://doc.rust-lang.org/std/macro.println.html)
* [vec!](https://doc.rust-lang.org/std/macro.vec.html)
* [todo!](https://doc.rust-lang.org/std/macro.todo.html)
* [include_str!](https://doc.rust-lang.org/std/macro.include_str.html)
* [include_bytes!](https://doc.rust-lang.org/std/macro.include_bytes.html)
* [unimplemented!](https://doc.rust-lang.org/std/macro.unimplemented.html)
* [matches!](https://doc.rust-lang.org/std/macro.matches.html)

In reality, these are called [macros](https://doc.rust-lang.org/reference/macros.html).
They are replaced by more Rust code at compile time.

There are a number of [macros in the standard library](https://doc.rust-lang.org/std/#macros)
and there are many [crates sharing additional macros](https://crates.io/keywords/macro).

Macros can be useful to eliminate duplicate code that for some reason we cannot, or do not want to put in a function.

They can also include Rust code that would be executed during the compilation of the code.


* [The Little book of Rust Macros](https://veykril.github.io/tlborm/)
* [Macros tutorial](https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/)


