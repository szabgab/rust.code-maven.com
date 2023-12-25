---
title: Clap - accept string, integer, floating point numbers, booleans, and more
timestamp: 2023-12-25T13:30:01
published: true
description: We can define the expected type for each parameter and Clap will convert the values to the appropriate types.
tags:
    - Clap
    - String
    - i32
    - f32
    - bool
    - PathBuf
    - $?
    - ERROR_LEVEL
---

Part of the series about [Clap](/clap).

In the first article we saw [how to get started with Clap](/clap-simple) and how to accept a single parameter that is expected to be `String`.

In this example we'll see how to accept a number of parameters.

* One of them is a `String`.
* One is an **integer number** of type `i32`.
* Another **integer number** of type `u8`.
* A **floating point number** of type `f32`.
* A **boolean** (`bool`)
* A [std::path::PathBuf](https://doc.rust-lang.org/std/path/struct.PathBuf.html).

To be clear, by default Rust sees all the parameters as strings. What Clap does is it tries to convert the received string to the appropriate type of value
as we describe in the definition of the struct. If the conversion (parsing) fails then Clap will stop the running of the program and provides and error message.
In this case the exit-code of the program will be 2.

## Setup - Cargo.toml

We have the same dependencies as before, the Clap crate with the `derive` feature.

![](examples/clap/numbers-strings-bool/Cargo.toml)


## Source code

![](examples/clap/numbers-strings-bool/src/main.rs)


## Usage:

If we provide all the values as expected, it works fine:

```
$ cargo run -- --host 127.0.0.1 --port 5000 --small 42 --float 3.14 --path .

host:  127.0.0.1
port   5000
small: 42
float: 3.14
debug: false
path:  .
```

Note, for the `path` parameter we had to call `display`, otherwise we get an error:

```
`PathBuf` cannot be formatted with the default formatter; call `.display()` on it
```

## Help

Clap provides a default help message describing how to use the command, what parameters to supply.

```
$ cargo run -- --help

Usage: numbers-strings-bool [OPTIONS] --host <HOST> --port <PORT> --small <SMALL> --float <FLOAT> --path <PATH>

Options:
      --host <HOST>
      --port <PORT>
      --small <SMALL>
      --float <FLOAT>
      --debug
      --path <PATH>
  -h, --help           Print help
  -V, --version        Print version
```

We can take a look at the exit code:

```
$ echo $?

2
```

## Invalid value

If the user supplies a value that cannot be converted to the appropriate type then Clap will provide and error messages and exit with exit-code 2.
For example here were provide 500 to the `small` parameter that is defined as `u8` that means it is expected to be between 0-255.


$ cargo run -- --host 127.0.0.1 --port 5000 --small 500 --float 3.14 --path .

error: invalid value '500' for '--small <SMALL>': 500 is not in 0..=255

For more information, try '--help'.
```


To see the exit code:

```
$ echo $?

2
```

On MS Windows we can see the exit code this way:

```
echo %ERROR_LEVEL%

2
```


In another example we supplied a floating point number to the `port` parameter that was defined as `i32`,
that is an integer.

```
$ cargo run -- --host 127.0.0.1 --port 50.2 --small 23 --float 3.14 --path .

error: invalid value '50.2' for '--port <PORT>': invalid digit found in string

For more information, try '--help'.
```

To see the exit code run:

```
$ echo $?

2
```


