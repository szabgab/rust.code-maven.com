---
title: Result returned by main function in Rust - setting exit code
timestamp: 2024-04-10T14:30:01
author: szabgab
published: true
description: I understand what happens if any regular function returns a Result, but what happens when main returns a Result?
tags:
    - main
    - Result
    - "$?"
    - ERROR_LEVEL
    - "%ERROR_LEVEL%"
    - STDERR
    - Option
    - ok_or
    - None
    - Ok
    - Err
    - Error
    - exit
todo:
    - See at the bottom of the page
---


## Result with plain String as error

The simplest way I found returning a `Result` is to let the `Ok` part be the unit type `()` and `Err` (error) part be a `String`.

If then the code returns `Ok(())` the program exits normally.

```rust
fn main() -> Result<(), String> {
    Ok(())
}
```

This will have no output and the exit code will be set to 0 indicating success:

```
$ cargo run -q
$ echo $?
0
```

On the other hand if we encounter a problem we will want to return a `String` wrapped in an `Err` as in the next case:

```rust
fn main() -> Result<(), String> {
    Err(String::from("A problem"))
}
```

This will print the error text on the Standard Error (STDERR) and then set the exit code (or `ERROR_LEVEL`, if you are on MS Windows), to 1.

```
$ cargo run -q
Error: "A problem"
$ echo $?
1
```

## A Result return an std::error::Error

Instead of making the error a plain string it is probably better to make it a [std::error::Error](https://doc.rust-lang.org/std/error/trait.Error.html).

Returning `Ok` will be the same and will have the same result:


```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
```

Returning error will be a bit more complex:


```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Err(Box::<dyn std::error::Error>::from("Problem"))
}
```

The result is quite the same:

```
$ cargo run -q
Error: "Problem"
$ echo $?
1
```

## IO Error

So what happens if our code experiences some other error:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();

    let filename = args.get(1).ok_or("Missing argument")?;
    let text  = std::fs::read_to_string(filename)?;
    println!("{}", text);

    Ok(())
}
```

Here `args.get(1)` returns an [Option](https://doc.rust-lang.org/std/option/enum.Option.html) that will be `None`
if the args vector does not have a 2nd element meaning the user did not supply any value on the command line.

The [ok_or](https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or) method will convert it to [Result](https://doc.rust-lang.org/std/result/enum.Result.html).
What was `Some(value)` in the `Option` will be turned into an `Ok(value)`. If there was a `None` in the `Option` then the `Result` will be filled with an `Error`
containing the text we supplied as a parameter to the `ok_or` function. Very neat. The question mark `?` at the end will take the `Result`, if it has an `Ok(value)` then
the whole expression will evaluate to the `value` and it gets assigned to the `filename` variable.
If the `Result` has an error an `Err(something)` in then the function will immediately return with that error.
In our case this is the `main` function meaning the program will terminate.

This is what we see  if we don't supply any argument. Then we get this:

```
$ cargo run -q
Error: "Missing argument"
$ echo $?
1
```
If we provide a value on the command line then that value will be assigned to the `filename` variable
and we will get to the next line where we try to read the content of the file using the
[read_to_string](https://doc.rust-lang.org/std/fs/fn.read_to_string.html) function. This returns a `Result`.
If succeeds then it contains `Ok(content_of_the_file)` if fails then it will be some `Err(error_message)`.

The question mark `?` at the end does it's magic again. If `Result` was `Ok` it just assigns the `content_of_the_file`
to the variable called `text`. If there was a problem and the `Result` is some `Err(error_message)` then, just as before,
the function return immediately with that error.

In our case, because this is the `main` function this will print some error message on the screen and terminate the program.

If we supply a path to a file that does not exist we get this error:

```
$ cargo run -q some_file.txt
Error: Os { code: 2, kind: NotFound, message: "No such file or directory" }
$ echo $?
1
```

If we supply the name of a file that exists, but to which we don't have read access we get a different error message:

```
$ cargo run -q /etc/shadow
Error: Os { code: 13, kind: PermissionDenied, message: "Permission denied" }
$ echo $?
1
```

Finally, if we supply an existing file, the program works, it prints out the content of the given files (that we replaced by ... here)
and the exit code (`ERROR_LEVEL`) is set to 0 indicating success.

```
$ cargo run -q Cargo.toml
...
$ echo $?
0
```


## The whole file

{% include file="examples/error-returned-by-main/src/main.rs" %}

## A Note

No question that seeing these error messages is much nice than it would be to see Rust `panic!`, but they still look a bit strange
to a non-programmer. They all set the exit code to 1 and display the real error code which might be good, but it is probably
not what is expected. One day I might figure out how to convert these into the exit codes if the whole program.

I'll write about it when it happens.


* [Rogério Almeida](https://www.linkedin.com/in/rsalmei/) pointed to the [stabilization PR](https://github.com/rust-lang/rust/issues/48453) relevant to this topic.
* [nick42d](https://github.com/nick42d/) pointed to [his solution](https://github.com/nick42d/youtui/blob/main/src%2Fmain.rs)

I need to read those...

