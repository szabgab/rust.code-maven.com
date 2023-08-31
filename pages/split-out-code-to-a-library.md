---
title: How to refer to code that was split out to a library in Rust?
timestamp: 2023-08-31T16:30:01
description:
tags:
    - cargo
    - pub
    - crate
    - mod
---

When you create a crate that needs to be executable, you use `cargo new NAME` that will automatically create a file called `src/main.rs` with a single `main` function.

In this file I created a new function called `hello_in_main_rs` and called it from the `main` function. Creating function in the main.rs file and calling it is the most simple.

![examples/split-out-code-to-a-library/src/main.rs]


## lib.rs

In crates that are libraries the default is to have file called `src/lib.rs` so we can try to create that and a function called `hello_in_lib_rs` in it.
In order to make it accessible from outside of the `lib.rs` file we also need to use the `pub` prefix in-front of the function declaration.

![examples/split-out-code-to-a-library/src/lib.rs]

Now comes the tricky part. How can we access this function?

The recommended way is to use the name of the crate.

In this example I used the name `split-out-code-to-a-library`. That became the name of the folder `cargo` created and it was also included in the `Cargo.toml` file:

![examples/split-out-code-to-a-library/Cargo.toml]

Those dashes between the words made sense when I created the crate, but we can't use them inside the code. So instead of that we use the same name, but we replace the dashes by underscores. So can call the function this way: `split_out_code_to_a_library::hello_in_lib_rs();` from the `main.rs` file.

The problem I can see with this is that if we rename the crate by changing the `package.name` field in the `Cargo.toml` then we also have to change our code to reflect the new name.

I think, if in a crate that has a `main.rs` we would like to move some code to a separate file purely for better internal code organization, then it is probably better to use filenames other than `lib.rs`.

On the other hand if we would like to let other crates to use our functions then we should probably create a separate crate that has a `lib.rs` but has no `main.rs`.

## other.rs

So what happens if we create another file called `other.rs` with a function called `hello_in_other_rs` in it? How can we call it from `main.rs`?

![examples/split-out-code-to-a-library/src/other.rs]

For this we needed 3 things. In the `other.rs` we had to use the `pub` prefix. (That stand for public and not "place to drink beer".)

We had to add `mod other;` to the `main.rs` and then we could use the name of the file to access the function in two different ways:

`other::hello_in_other_rs();` says where relative to the `main.rs` file the other file and function can be found.

`crate::other::hello_in_other_rs();` says where the `other.rs` file and the function is located starting from the root of the current crate.

In either case we only need to keep aligned with the internal name of the file and the internal directory hierarchy.

We can freely rename the crate to anything.

