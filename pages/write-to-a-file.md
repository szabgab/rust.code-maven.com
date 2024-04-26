---
title: Write to a file - create file in Rust
timestamp: 2024-04-26T08:30:01
author: szabgab
published: true
description: How to save some text in a file in a Rust program
tags:
    - create
    - writeln!
    - File
    - Write
    - Error
todo:
    - append to a file
---

In order to write to a file - to create a new file and write some text in it - we can do the following.


We use the [File struct](https://doc.rust-lang.org/std/fs/struct.File.html) that represents and open file.
The [create](https://doc.rust-lang.org/std/fs/struct.File.html#method.create) method will create a new file or truncate the content of the file if it already
exists. Meaning that if we don't write anything to the file we'll now have an empty file. Regardless if it existed before and if it had content before.

As this might fail the function returns a [Result](https://doc.rust-lang.org/std/result/enum.Result.html) struct that we need to deal with.
In this example we opted the use of the trailing question mark `?` (See the [Evolution of error handling in Rust](/evolution-of-error-handling) for an explanation.)

For this to work we also had to set up the main function to return a `Result` as explained in [Result returned by main function in Rust](/result-returned-by-main).

We can use the returned value in a [writeln!](https://doc.rust-lang.org/std/macro.writeln.html) to write the content to the file.

There is no need to explicitly  close the file as files are automatically closed in the [drop destructor](/drop-the-destructor-of-rust-structs)  when they go out of scope.

{% include file="examples/files/write-to-file/src/main.rs" %}


One thing that still disturbs me in this example is the fact that we need to have `use std::io::Write;` in the code, but we then don't use the `Write` name.
That's because [Write](https://doc.rust-lang.org/std/io/trait.Write.html) is a trait that adds functionality to the `File` which is used in the `writeln!` macro.


