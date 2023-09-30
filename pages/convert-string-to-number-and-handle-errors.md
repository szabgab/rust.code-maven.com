---
title: Convert a string to number and handle possible errors
timestamp: 2023-08-30T14:30:01
description: A relatively simple task of checking if a string can be converted to a number and then converting it. Several ways to handle errors.
tags:
    - parse
    - i8
    - u8
    - f32
    - casting
    - turbofish
    - expect
    - unwrap
    - unwrap_or
    - unwrap_or_default
    - match
    - Ok
    - Err
todo:
    - More about turbofish
---

No matter where input comes from (the keyboard, a file, the network) - type-wise it is seen as some kind of a string or series of characters.
Even if the program asked for a number and the user provided a number.

In order to work with the value and use numerical operations on that value in most programming language we need to somehow convert the string into a number type.
In Rust there are a number of [numeric types](https://doc.rust-lang.org/reference/types/numeric.html):

* `u8`, `u16`, `u32`, `u64`, `u128` are unsigned integers using the given number of bits. For example `u8` uses 8 bits (1 byte) and it can store a value between 0 and 2^8-1 (that is between 1 and 255).
* `i8`, `i16`, `i32`, `i64`, `i128` are signed integers using the given number of bits. For example `i8` uses 8 bits (1 byte) and it can store a value between -(2^7) and 2^7-1 (that is between -128 and +127).
* `f32`, `f64` floating point numbers according to the [IEEE 754-2008](https://en.wikipedia.org/wiki/IEEE_754-2008_revision) specification.
* There are also `isize` and `usize`.


In some languages converting a string to a number is called casting, in Rust it is accomplished by calling the [parse()](https://doc.rust-lang.org/std/primitive.str.html#method.parse) method of a string.

## Code example

![](examples/convert-string-to-number/src/main.rs)

Feel free to use this example, enable/disable parts of the code to see the various solutions for error handling.

## Parse

The `parse` method needs to know what numerical type it is supposed to create. There are two ways to accomplish this. In one way we define the type of the variable that we are assigning to, in this case we used `i16`.

```rust
let num: i16 = value.parse().expect("Could not convert");
```

In the second case we use the so-called **turbofish** syntax, where we put the type after the function in `::<>`.

```rust
let num = value.parse::<i16>().expect("Could not convert");
```

## Good results

If we run `cargo run 42` we will see `Converted to 42`. That's the happy path. What about error handling?

## Error handling

An `i16` can hold an integer number between -(2^15) and (2^15)-1 that is between -32768 and +32767. For any string that holds some other value the operation will fail.
So if we try to convert 3.14,  10,000,000, or "abc" and even 32768 will fail. The question then, how can we deal with this in our code.

In the example below I'll stick to the first version, but they could have been done with the turbofish syntax as well.

### expect

The first way we saw in the examples above was to add a call to [expect](https://doc.rust-lang.org/std/result/enum.Result.html#method.expect) to be executed on the result of `parse()`. This is an easy, but not a recommended solution. It is easy as it only requires us to add a few letters, but it is not recommended as this will cause the program to [panic](https://doc.rust-lang.org/std/macro.panic.html) in case the `parse()` fails. That is, if we supply any value that cannot be converted to `i16` our program will die.

For example:

```
cargo run 3.14


thread 'main' panicked at 'Could not convert: ParseIntError { kind: InvalidDigit }', src/main.rs:11:34
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

In the error message you can see the text we included in the call to `expect`.

Unlike **exceptions** in other programming languages, you cannot "catch" a `panic` in Rust. You should probably want to avoid it.

### unwrap

Instead of `expect` we could call [unwrap](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap), but that seems to be worse. It includes a generic error message instead of one we decided upon.

```rust
let num: i16 = value.parse().unwrap();
```

```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }', src/main.rs:14:34
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### unwrap_or

```rust
let num: i16 = value.parse().unwrap_or(42);
```

The call to [unwrap_or](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or) allows us to provide a default value. Is it a good idea to provide a default value?
Some cases it might be. In other cases probably not. Using 42 as the default value for a numerical conversion is clearly not a very good idea
as you can see from the results:

```
$ cargo run 3
Converted to 3

$ cargo run 3.1
Converted to 42
```

### unwrap_or_default

```rust
let num: i16 = value.parse().unwrap_or_default();
```

We could use [unwrap_or_default](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or_default) that, in case of parsing error,
would use the default value of numbers which is **0**.


### match / Ok / Err

Finally let's see the fully coded solution using the [match](https://doc.rust-lang.org/std/keyword.match.html) keyword:

```rust
let num: i16 = match value.parse() {
    Ok(val) => val,
    Err(err) => {
        eprintln!("Could not convert: '{err}'");
        return;
    }
};
```

In this solution we are using `match` to handle the two possible return values of the `parse` function
that can be returned as two different cases of a [Result](https://doc.rust-lang.org/std/result/index.html) type.

The happy case, when the parsing is successful is when get back the parse value wrapped in `Ok`. If `Ok(val)` matches then we are only interested in the value that is in `val`.
(Here `Ok` was the keyword and `val` was just an temporary variable name.)

In case the parsing fails we get back an error object wrapped in an `Err`. In this case we print some error message and then return from this call, which in our case effectively means the program stops. How exactly you deal with the case of failure is of course up to you. This is just one way to handle it.


