---
title: Simple case of using the clippy::pedantic lints and setting priority
timestamp: 2024-01-18T08:45:01
author: szabgab
published: true
description: Setting the priority of the Rust lints is important. Here is how to do it.
tags:
    - Clippy
    - linter
todo:
    - fix
---

[Clippy](https://doc.rust-lang.org/stable/clippy/index.html) is an excellent linter for Rust. It can help improving your code.

As a starter you would just run `cargo clippy` in the folder of your crate, but later you might want to add additional lints.

The following code has two issues that the `clippy::pedantic` set of rules will flag:

![](examples/uninlined-format-args/src/main.rs)

## Clippy with command line flags

One way to check this is to run clippy with flags. `-D` means we want to **deny** these rules, that clippy won't accept them. This command will show 2 errors:

```
$ cargo clippy --  -Dclippy::pedantic


    Checking uninlined-format-args v0.1.0 (/home/gabor/work/rust.code-maven.com/examples/uninlined-format-args)
error: binding to `_` prefixed variable with no side-effect
 --> src/main.rs:3:5
  |
3 |     let _x = 1;
  |     ^^^^^^^^^^^
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#no_effect_underscore_binding
  = note: `-D clippy::no-effect-underscore-binding` implied by `-D clippy::pedantic`
  = help: to override `-D clippy::pedantic` add `#[allow(clippy::no_effect_underscore_binding)]`

error: variables can be used directly in the `format!` string
 --> src/main.rs:4:5
  |
4 |     println!("Hello, {}!", name);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#uninlined_format_args
  = note: `-D clippy::uninlined-format-args` implied by `-D clippy::pedantic`
  = help: to override `-D clippy::pedantic` add `#[allow(clippy::uninlined_format_args)]`
help: change this to
  |
4 -     println!("Hello, {}!", name);
4 +     println!("Hello, {name}!");
  |

error: could not compile `uninlined-format-args` (bin "uninlined-format-args") due to 2 previous errors
```

Usually in a big code-base you will get tens or hundreds of these that you can't fix immediately. So you might want to exclude the failing ones and then enable them one by one
as I suggested in the [getting started with Clippy on an existing project](/getting-started-with-clippy-on-an-existing-project) article.


```
$ cargo clippy -- -Dclippy::pedantic -Aclippy::uninlined_format_args -Aclippy::no_effect_underscore_binding
```

This one will report that everything is fine as we tuned on (denied) all the **pedantic** lints, but then disabled (allowed) the **uninlined_format_args** and the **no_effect_underscore_binding** lints.


However the order of the flags matter. If we put the `-Dclippy::pedantic` at the end then we would still get the two errors.

```
$ cargo clippy -- -Aclippy::uninlined_format_args -Aclippy::no_effect_underscore_binding -Dclippy::pedantic
```

**So order matters!**


## Configure Clippy in `Cargo.toml`

I don't know where I found that it is possible to configure Clippy in the `Cargo.toml` file, but apparently it is so new that it is not even mentioned in the [documentation of Clippy](https://doc.rust-lang.org/stable/clippy/configuration.html).
See my [issue](https://github.com/rust-lang/rust-clippy/issues/12164). As I see it is mentioned in the [documentation of Cargo](https://doc.rust-lang.org/cargo/reference/manifest.html#the-lints-section).

Anyway, we can configure Clippy by adding the following section to `Cargo.toml`:

```toml
[lints.clippy]
pedantic = "deny"
uninlined_format_args = "allow"
no_effect_underscore_binding = "allow"
```

and then run `cargo clippy` without any extra parameters.

However this will put them in ABC order on the command line, as if we were running this command:

```
cargo clippy --  -Aclippy::uninlined_format_args -Dclippy::pedantic -Aclippy::no_effect_underscore_binding
```

Which means it will still report the **uninlined_format_args** issues as **pedantic** comes after it on the command line.

The right way to configure this order is to set the **priority** of **pedantic** lint-group lower:


```toml
[lints.clippy]
pedantic = { priority = -1, level = "deny" }
uninlined_format_args = "allow"
no_effect_underscore_binding = "allow"
```

then running `cargo clippy` will not complain any more.

Once clippy is happy we can enable the lints that we feel are important and fix the code slowly.

## Configure in `clippy.toml` or `.clippy.toml`

We can also use either of those files to configure Clippy and as I understand the priority setting is clearer there.
Read the documentation on how to [configure Clippy](https://doc.rust-lang.org/stable/clippy/configuration.html).

## Automatic Fix

We could use the `--fix` flag to get Rust to fix our code:

```
cargo clippy --fix
```

I will write about this in a separate article.


## The full `Cargo.toml` file

![](examples/uninlined-format-args/Cargo.toml)


