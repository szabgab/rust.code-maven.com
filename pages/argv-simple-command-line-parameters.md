---
title: ARGV - command line parameters for simple CLI program
timestamp: 2023-11-28T12:30:01
author: szabgab
published: true
description:
tags:
    - CLI
    - args
    - ARGV
    - count
    - nth
    - next
todo:
    - clap
---

For applications that need multiple command line parameters it is probably much better to use one of the crates
that can deal with command line parameters well, but for simple cases with 1-2 parameter we might prefer to use
the raw values as seen by Rust.

Using the [std::env::args](https://doc.rust-lang.org/std/env/fn.args.html) function we create a vector of all
the arguments on the command line. This will include the name of the program itself as the first parameter.

If the user does not provide any parameter we'll have one element in this vector.

If the user provides one value on the command line then we'll have a vector with two elements.

Having a program that accepts a single parameter is such a common case (at least for the programs I write)
that I have a separate article for when you [expect one command line parameter](/expect-one-command-line-parameter).

In this example first we create the array we called `argv`.

Then we print the content of the array and the number of elements so you can experiment with it.

Then there are several examples on how do deal with the various cases.

We can iterate over all the parameters excluding the first one which is the name of the program.

We can check if we the number of parameters is less than a certain number (in our case 3)
to make sure we don't try to extract the parameters if the user has not supplied them.

We can assign the content of the argv vector to specific variables to make the code more readable.
(Ok, maybe use names that are more descriptive for your purposes than "first" and "second".)


{% include file="examples/argv-simple-command-line-parameters/src/main.rs" %}

```
cargo run apple banana

["target/debug/argv-simple-command-line-parameters", "apple", "banana"]
Number of elements on the command line 3

apple
banana

First: apple
Second: banana
```

## Alternative without a vector

I am not sure if this is a "better" solution, or in what way would this be a better solution,
but I've seen this used in a number of examples.


{% include file="examples/argv-command-line-parameters-with-count/src/main.rs" %}

Here we use various features of the fact that `args()` returns an iterator.
We can use the `count()` method to get the number of elements,
we can use the `nth()` method to get the nth element and we can use
the `next()` method to get, surprise, surprise, the next element.

The `nth()` method uses 0-based index and thus `nth(1)` is in reality the 2nd element the iterator returns.

I'd probably use `nth(0) to get the first element of the iterator, but [clippy](https://doc.rust-lang.org/nightly/clippy/)
suggested I try `next()` instead so that's what I have in the example.

