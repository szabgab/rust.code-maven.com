# Hello World with Cargo

* cargo

* Cargo is the package management, dependency management, and build system of Rust.
* `cargo new hello-world` creates a new folder called `hello-world`
* With a file called `Cargo.toml`
* A folder called `src` and  a file `src/main.rs` with the hello world code.
* `cargo run` will comple the code and run it.

```
cargo new hello-world
cd hello-world
```

{% embed include file="src/examples/intro/hello-world/src/main.rs" %}
{% embed include file="src/examples/intro/hello-world/Cargo.toml" %}

* It will also create a git repository out of this folder.

```
$ cargo run
   Compiling hello-world v0.1.0 (/home/gabor/work/slides/rust/examples/hello-world)
    Finished dev [unoptimized + debuginfo] target(s) in 0.34s
     Running `target/debug/hello-world`
Hello, world!
```

It also created a file called `Cargo.lock` and a folder called `target`.
* `Cargo.lock` is used to fix the versions of all the dependencies of the project. We have none so far.
* `target` contains the compiled file and all the temporay files that Rust needed for the compilation.


