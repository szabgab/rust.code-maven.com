---
title: Hello World - Hello Rust
timestamp: 2023-08-27T14:30:01
author: szabgab
published: true
descripton: Two ways to create, compile, and run your first Rust program printing Hellow World! to the screen.
todo:
    - How to install Rust.
    - Using the cmd on Windows.
tags:
    - cargo
    - rustc
    - println!
    - fn
---

I think it is a requirement in every programming language and every technology one learns to create, the "Hello World" example, so we should also start with that.

This areticle assumes that you already have Rust installed on your computer. If not, there should be an article on how to install Rust.

I also assume that you know how to use the command line (the terminal on Linux and on macOS, or the CMD window on Windows).

The installation of **Rust** comes with the **rustc**, the Rust compiler and **cargo** the package manager of Rust.
In the first example we'll see how to use the compiler directly and in the second example we'll use the package manager.

In the most cases, in most projects you'll use the second approach, but seeing the first can help in the understanding.


## Using rustc the Rust compiler directly

Create a file called `hello.rs` (The actual name does not matter. The `rs` extension is not required either, but it is quite useful to stick to the standard.)
I've created it in the `examples` folder.

![](examples/hello.rs)

Then run the compiler:

```
rustc examples/hello.rs
```

This will create a file called `hello`. It is pretty big, it 4,298,488 bytes. but if we run it by typing in `./hello`, it will print "Hello World".


## Hello World using the cargo package manager

On the command line run the following command. (Instead of `hello-rust` you could use any name)

```
cargo new hello-rust
```

Actually I ran `cargo new examples/hello-rust` because I would like to save all of examples in the folder with that name.

This created a folder called `hello-rust` and in the folder the following content:


```
$ tree
.
├── Cargo.toml
└── src
    └── main.rs
```

The Cargo.toml is a configuration file that we disregard for now.

![](examples/hello-rust/Cargo.toml)

The `main.rs` is a default skeleton of a program that prints, surprise!, "Hello, World!". You'd start every Rust application and Rust library from this skeleton.

![](examples/hello-rust/src/main.rs)


At this point you can change directory (`cd`) into the `hello-rust` folder and run the following command:


```
cargo run
```

this will print something like this:

```
   Compiling hello-rust v0.1.0 (/home/gabor/work/rust.code-maven.com/examples/hello-rust)
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/hello-rust`
Hello, world!
```

The first 3 lines telling you about the compilation of the code and the last line being the actual output.

This command also created a file called `Cargo.lock` that we'll keep and discuss later and a folder called `target` that contains many temporary files and the compiled executable.

The compiled executabe was automatically executed, but now that it was created you can run it on your own as well:

```
target/debug/hello-rust
```

It's size is 4,308,272 bytes.


You can now edit the `main.rs` file, change the text, and run `cargo run` again. It will re-compile the code and run the new version.


## Version Control System (VCS)

If you use a VCS , eg. git while doing experiments, you probably should **not** save the `hello` binary from earlyer or the `target` folder from now.


