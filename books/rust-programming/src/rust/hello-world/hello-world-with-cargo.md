# Hello World in Rust with Cargo

Earlier we saw how to compile a Rust program using the compiler directly, but in reality Rust developers rarely interact with the compiler directly.
Instead they use `cargo` the all helper tool.

Cargo is the package management, dependency management, and build system of Rust. It is extendable and thus a lot of additional tools are combined with it.

For now let's see how we get started using it:


* `cargo new first-app` creates a new folder called `first-app`
* With a file called `Cargo.toml`
* A folder called `src` and  a file `src/main.rs` with the hello world code.
* `cargo run` will compile the code and run it.

```
cargo new first-app
cd first-app
```

{% embed include file="src/examples/intro/first-app/src/main.rs" %}
{% embed include file="src/examples/intro/first-app/Cargo.toml" %}

* Unless you do this in a folder that is already under git, the `new` command will also create a git repository using this folder.


## Compile and run in one step

Once we are in the folder of our brand new Rust application we can type in `cargo run`. This will compile the code in this
folder and run it. In one step. Just as you would do in any of the dynamic languages. You don't have to deal with the separate
compilation and running step and you don't need to create an maintain a `Makefile` either.

```
$ cargo run
   Compiling first-app v0.1.0 (/home/gabor/work/slides/rust/examples/first-app)
    Finished dev [unoptimized + debuginfo] target(s) in 0.34s
     Running `target/debug/first-app`
Hello, world!
```


The compilation phase created a file called `Cargo.lock` and a folder called `target`.
* `Cargo.lock` is used to freeze the versions of all the dependencies of the project. We have none so far. We'll see this more in detail later on.
* The `target` folder contains sub-folder called `debug` and that contains the compiled file and all the temporary files that Rust needed for the compilation.


---

* cargo


