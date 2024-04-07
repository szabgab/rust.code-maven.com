---
title: Getting file extension in Rust using PathBuf
timestamp: 2024-04-07T09:20:01
author: szabgab
published: true
description: How to get the file extension in Rust
tags:
    - extension
    - PathBuf
    - OsStr
    - unwrap
    - unwrap_or_default
    - match
    - let
    - Some
    - None
todo:
    - compare file extension to a string (so we can handle different file extensions in different ways, e.g. return content-type
    - remove file extension
    - add file extension
    - change file extension
---

In some applications I need to extract the extension of a file. The part after the dot. We can do that easily using the [extension](https://doc.rust-lang.org/std/path/struct.PathBuf.html#method.extension)
method of `PathBuf`.


## The simple case:

```rust
let path = PathBuf::from("hello.txt");
println!("{:?} {:?}", path, path.extension().unwrap());
```

This will work, however if the path does **not** have an extension, the `extension` method will return `None` and the `unwrap` will generate a panic:

```rust
let path = PathBuf::from("hello");
println!("{:?} {:?}", path, path.extension().unwrap());
```

Will get you this:

```
thread 'main' panicked at src/main.rs:8:50:
called `Option::unwrap()` on a `None` value
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Handle with `match`

One way to handle this problem is to use `match`. It has two arms, one to handle the case when `extension` returns
`Some` value, that's when there was an extension. The other arm is when `extension` returns `None`. In which case
we just report the lack of extension.

The drawback of this approach is that after the closing curly brace of `match` the variable `ext` holding the extension
is not in scope any more. So anything you want to do with the extension must be inside the `match`.

```rust
match path.extension() {
    Some(ext) =>  println!("match: {:?}  {:?}", path, ext),
    None => println!("match: {:?}  has no extension", path),
}
// ext is not in scope here
//println!("{}", ext);
```

## Handle with `let` - `else`

Another way is to use a `let Some()` statement with an `else` part. In the `else` part
we can report the lack of extension and then we need to terminate the current processing.
Because in our example we used a for-loop we terminate the current iteration by calling `continue`.
If this was in a function, we would call `return`.

The big advantage of this is that after the `let` statement we have the `ext` variable in scope:


```rust
let Some(ext) = path.extension() else {
    println!("let:   {:?}  has no extension", path);
    continue;
};
println!("let:   {:?}  {:?}", path, ext);
```


## Handle with `let` - `match`

A third way is to combine the `let` and `match` statements. In this case we can set a default value
to be returned by the `match`. In some cases this is preferable over skipping the rest of the code.
As that `extension` method returns an instance of an [OsStr](https://doc.rust-lang.org/std/ffi/struct.OsStr.html)
we also have to create the default value using that.

let ext = match path.extension() {
    Some(ext) => ext,
    None => OsStr::new(""),
};
println!("let match: {:?}  {:?}", path, ext);


## Handle with `let` - `unwrap_or_default`

The previous `match` statement can be actually simplified to a call to [unwrap_or_default](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_default):

```rust
let ext = path.extension().unwrap_or_default();
println!("default:   {:?}  {:?}", path, ext);
```

## The full code

{% include file="examples/getting-file-extension/src/main.rs" %}

## The output

```
"hello.txt" "txt"

match:     "hello.txt"  "txt"
let match: "hello.txt"  "txt"
default:   "hello.txt"  "txt"
let:       "hello.txt"  "txt"
match:     "hello"  has no extension
let match: "hello"  ""
default:   "hello"  ""
let:       "hello"  has no extension
match:     "main.rs"  "rs"
let match: "main.rs"  "rs"
default:   "main.rs"  "rs"
let:       "main.rs"  "rs"
```


