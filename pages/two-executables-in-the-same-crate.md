---
title: Two or more executables in the same crate
timestamp: 2023-09-12T12:30:01
description:
tags:
    - bin
---

Normally if you run `cargo new name` you will create a crate that has a file called `src/main.rs` which is the main file of the program that will become the executable.
You would be able to compile and run that executable by the `cargo run` command. If you used `cargo build` to create the executable file, its name would be the same name that you used for the crate.

Sometimes, however you would like to create several executables using the same crate.

You can do that by telling cargo that some of the additional files are also executables. Here is an example:

Started by creating the crate:

```
cargo new two-executables
```

I've removed the `src/main.rs` to make the others "symmetric", so we will do the same coding for each one of them.

Instead of that I've added two simple files. Both of them have a `main` function.

![](examples/two-executables/src/one.rs)

![](examples/two-executables/src/two.rs)

Then I've added a `bin` section for each one of the binaries


![](examples/two-executables/Cargo.toml)


This way I can run them a

```
cargo run --bin one_exe
```

and

```
cargo run --bin two
```

As you can see the `name` field does not have to be the same as the name of the file, and the `name` field is the one that determines how this program will be called.

In target/debug/ you will see the two executables called `one_exe` and `two`.

