---
title: Moving from Java to Rust
timestamp: 2025-02-18T09:30:01
author: szabgab
published: false
show_related: true
description:
tags:
    - Rust
todo:
    - TODO
---

This article is for people who would like to move a project from Java to Rust.


In Rust there are no classes in the same sense as there are in Java. Especially there is no inheritence.

In Rust there is no garbage collector. Every piece of data has a single owner and when that owner goes out of scope then the data is removed from memory.

An analogy of the memory model: Java is like you drink coffer and leave your mugs on your desk and then at the end at some random point in time someone comes and cleans all your mugs while you wait staring at the wall.
The Rust ownership model is like when you finised drinking your coffee you clean your mug immediately.

Assignment in Java is copying a reference. In Rust you need to prefix the variable with an `&` sign to `borrow` the data, which is similar to having a reference to the data.

Java has interfaces and generics, Rust has traits.

There is no Null in Rust. Instead you have the Option enum which can be either Some value or None, but you have to check which one it is before you can use the value.

Rust has `match` instead of switch, and you have to handle every potential value or the code won't compile.

In Java there is Maven and Gradle for packaging and dependency management. In Rust there is Cargo.

Rust is stricter and more intentional than Java.


## Resources

Videos by ForrestKnight:

In [My Experience with Rust as a Java Dev](https://www.youtube.com/watch?v=a0LtFp-7T2s) (Dec 28, 2024) he points out a number of differences between Java and Rust he observed.

The video [Why I'm Learning Rust in 2024 (and new dev environment)](https://www.youtube.com/watch?v=3q3OXiyUQk4) (Nov 25, 2024) helps you understand his motivation
learning Rust and his plans before he actually started learning Rust, but you can als skip this or watch it later.

[I Tried to Learn Rust (w/ Advent of Code)](https://www.youtube.com/watch?v=e5YfD4NHlCI) (January 31, 2024) is a long video showing the steps as he was writing some basic Rust code.


