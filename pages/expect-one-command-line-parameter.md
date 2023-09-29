---
title: Expect one command line parameter in Rust
timestamp: 2023-09-29T14:30:01
description:
tags:
    - std::env::args
    - std::process::exit
    - collect
    - args
    - exit
    - eprintln!
    - STDERR
    - cli
todo:
    - articles about clap
---

I write many command line utilities. Some of those are rather simple, they only expect a single value on the command line. e.g. a file.
Sometimes it becomes a larger tool with more command line options, but at first I like to start out simple.

This is a simple example that does not use any external [crates](https://crates.io/) on how to accept a command line parameter and how to report when it is missing or if the user supplied more than one value. Which probably indicates either a misunderstanding or it is just a plain mistake.


![](examples/one-command-line-parameter/src/main.rs)


I started this example by running `cargo new one-command-line-parameter` so the name of the folder is `one-command-line-parameter`.

## How does it behave?

`cargo run` will print this message: `Usage: target/debug/one-command-line-parameter value`.

`cargo run hello` will print this: `Value: hello`.

If we supply more than 1 value:

`cargo run hello world` we get this message: `Usage: target/debug/one-command-line-parameter value`.

If we put the values in quotes:

`cargo run "hello world"` then Rust will see the whole sting as a single parameter and our code will be happy again  `Value: hello world`.

In the **usage** messages we saw the path to the executable that was run. If we compile a release version of the application using `cargo build --release`
then we can take the binary from `target/release/one-command-line-parameter` move it anywhere and rename it. The usage message will always print the name
of the executable file.

Fore example I moved and renamed it: `mv target/release/one-command-line-parameter qqrq`
and then I ran `./qqrq` and got `Usage: ./qqrq value`.


## How does it work?

[std::env](https://doc.rust-lang.org/std/env/) provides access to the environment of the process. Specifically [std::env::args](https://doc.rust-lang.org/std/env/fn.args.html)
returns the arguments that the program was started with. The first value is the program itself.

We use the [collect](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect) method to transform this iterator to a collection. A vector of strings in this case.

Then we are checking if the length of this vector is different from 2. As the first value of this vector will always be the name of the program, this number will be always one bigger than the number of parameters supplied.

If the number is different from 2 then we use the [eprintln!](https://doc.rust-lang.org/std/macro.eprintln.html) macro to print the usage message to the **Standard Error** channel. (STDERR). In the message we use `argv[0]`, the first element of the `args` vector that contains the name of the program.

Then we call [std::process::exit](https://doc.rust-lang.org/std/process/fn.exit.html) to stop the program with an exit code of 1. (An exit code of 0 would mean success any other number indicates failure. I used 1 here because using 42 would confuse some people.) On the command line you can see this exit code by printing the `$?` variable on Linux/macOS or by printing the `%ERROR_LEVEL%` variable on MS Windows.


If the length was 2 then we don't enter this block and we assign the value from `&argv[1]` to a variable name that we can use later.

That's the whole thing.

