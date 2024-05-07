---
title: Elapsed time logger
timestamp: 2024-05-07T17:30:01
author: szabgab
published: true
description: How to to print the elapsed time of a function easily
tags:
    - Drop
    - impl
    - new
    - time
    - Instant
---

While working on the [Rust Digger](https://rust-digger.code-maven.com/) I wanted to be able to collect and display the elapsed time of the varous processes.

Partially I waned this to be able to profile the code and make improvements where it will make the most impact, and partially just to show it.

There are a number of code-profilers for Rust and at one point I am going to use them on the project, but after writing about [Drop, the destructor of Rust structs](/drop-the-destructor-of-rust-structs)
I figured I'd like to see if I can use it to make it easier to implement this.

This is my first implementation:

{% include file="examples/elapsed-time-logger/src/main.rs" %}


Here in any function, or for that matter in any block I can create a variable (with a leading underscore as we won't really use the variable)
to be an instance of the `ElapsedTime` struct. The `new` method of the crate (which would be called the constructor or initializer if we used the OOP lingo)
stores the string we passed to it aling with the current time.
It also prints to the screen that we "START" the function.

Then when the variable goes out of scope - when the code leaves the enclosing block - the `drop` method is called
that checks the time again, computes the elapsed time and displays that with the word "ENDED" and the name we stored earlier.

## Logging

Naturally instead of simply pritning to the screen we could use a logger crate to log the value or we could save it in some special file
that can be later analyzed or displayed on a web page.


## Other crates

Once I implemented this I thought I should publish it on [Crates.io](https://crates.io/) and I was wondering what should be the name.

[function-timer](https://crates.io/crates/function-timer) seemed to be the obvious choice, but as you can see it already exists and
if I am not mistaken it tries to solve the same problem in a nicer way. I am going to look into that crate later.
