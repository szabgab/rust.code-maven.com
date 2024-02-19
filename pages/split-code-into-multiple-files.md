---
title: Split code into multiple Rust files
timestamp: 2024-01-11T16:50:01
author: szabgab
published: true
description: In order to better organize our code we might want to split it into multiple files.
tags:
    - use
    - crate
---


As our crate grows and has more and more structs, enums, and functions, putting everything in the `src/main.rs` file will start to be confusing.

In order to better organize our code we might want to split it into multiple files.  (and later maybe into multiple crates, but for now let's keep them in the same crate.


# The functions

* It has one function called `in_main` and we can call it without any extra effort.

* In the `src/lib.rs` file we have function called `in_lib`.
    * It is declared with the `pub` keyword to make it public.
    * We can call it by prefixing the name with the name of the crate `app::in_lib();`. (`app` is the name of the crate as defined in the `Cargo.toml` file.)


* In the `src/helper.rs` file there is a function called `in_helper`. In order to make it usable in the main file:
    * We made it public with the `pub` prefix.
    * We also added `pub mod helper;` to the `src/lib.rs` file.
    * Now we can call it with its full name: `app::helper::in_helper();`

* Another function in the `src/helper.rs` called in_imported_helper. 
    * We made it public with `pub`.
    * In `src/lib.rs` we added `pub use helper::in_imported_helper;` (imported it)
    * We can now call it `app::in_imported_helper();` (without the exact location of the function.)

* In order to allow a a function in `src/helper.rs` to call a function in `src/lib.rs`
    * The function must be `pub`.
    * We prefix it with `crate` as in `crate::in_lib()`.


## Cargo.toml

{% include file="examples/split-code/Cargo.toml" %}


## The main.rs

{% include file="examples/split-code/src/main.rs" %}

## lib.rs

{% include file="examples/split-code/src/lib.rs" %}


## helper.rs

{% include file="examples/split-code/src/helper.rs" %}

## tools.rs

{% include file="examples/split-code/src/tools.rs" %}

