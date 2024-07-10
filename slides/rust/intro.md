# Introduction to Rust
{id: rust-intro}

## Why Rust?
{id: why-rust}

* The main web site of the [Rust the language](https://www.rust-lang.org/) points out 3:

* Performance
* Reliability
* Productivity

## Why Rust - Performance
{id: rust-performance}

* Roughly at the speed of C and C++ programs
* [ruff](https://github.com/astral-sh/ruff) - A linter for Python
* [speed comparision](https://github.com/niklas-heer/speed-comparison)
* [Ranking programming languages by energy efficiency](https://thenewstack.io/which-programming-languages-use-the-least-electricity/)


## Why Rust - Reliability
{id: rust-reliability}

* type system
* memory safety
* thread-safety

## Why Rust - Productivity
{id: rust-productivity}

* Tooling (IDEs, compilation, package management, dependency management, linter).
* [Documentation](https://doc.rust-lang.org/).
* 3rd party open source libraries in the [Crates registry](https://crates.io/).


## Major features of Rust
{id: major-features-of-rust}

* Shift left programming (eliminate bugs earlier in the development process)

* Speed and Open Source community

* [Corporate support](https://foundation.rust-lang.org/members/), Linux kernel

* Error handling: exceptions VS return values - returning Result
* Eliminate `null` issue.

## Memory allocation
{id: memory-allocation}

* Stack
* Heap
* In the binary

## Rust Books
{id: rust-books}

* [Learn Rust](https://www.rust-lang.org/learn)
* [The Rust book](https://doc.rust-lang.org/book/)
* [Rust by example](https://doc.rust-lang.org/stable/rust-by-example/)
* [Rust for Rustaceans](https://rust-for-rustaceans.com/)
* [Zero To Production In Rust](https://www.zero2prod.com/)

## Crates (3rd party libraries)
{id: crates-io}

* [Crates.io](https://crates.io/) - the registry of all the crates.
* [Docs.rs](https://docs.rs/) - the documentation of all the crates.
* [Lib.rs](https://lib.rs/) - Lightweight, opinionated, curated, unofficial alternative to crates.io.
* [Rust Digger](https://rust-digger.code-maven.com/) - analyzing crates and recommending improvements.

## Rust exercises with feedback
{id: rust-exercises-with-feedback}

* [rustlings](https://rustlings.cool/)
* [Exercism Rust track](https://exercism.org/tracks/rust)
* [Rust users forum](https://users.rust-lang.org/)

## Podcast, newsleter
{id: rust-podcast}

* [This week in Rust](https://this-week-in-rust.org/) - a weekly newsletter
* [Rustacean](https://www.rustaceans.org/)
* [Rustacean station](https://rustacean-station.org/) - a podcast


## Other Rust learning resources
{id: other-rust-learning-resources}

* [Let's Get Rusty](https://letsgetrusty.com/)
* [noboilerplate](https://github.com/0atman/noboilerplate)
* [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/)
* [Rust Atomics and Locks](https://marabos.nl/atomics/) Low-Level Concurrency in Practice by Mara Bos.
* [Learning Rust](https://www.youtube.com/channel/UC7OY971Es9P_AcXGYKms87A) by Ian Walker.
* [teach-rs](https://github.com/tweedegolf/teach-rs) Free material for Rust courses.
* [Easy Rust](https://dhghomon.github.io/easy_rust/)



## Rust in other languages
{id: rust-in-other-languages}

* [From Perl to Rust](https://oylenshpeegul.gitlab.io/from-perl-to-rust/introduction.html)

* [The Rust book in Hebrew](https://github.com/IttayWeiss/rustbook-heb)
* [All the languages](https://doc.rust-lang.org/stable/book/appendix-06-translation.html)

## Articles about Rust
{id: articles-about-rust}

* [Why do Programmers Love Rust?](https://www.youtube.com/watch?v=vBsEF-anSLY) presentation by Dave Rolsky [slides](https://presentations.houseabsolute.com/why-do-programmers-love-rust/) - [source](https://github.com/autarch/presentations/tree/master/why-do-programmers-love-rust)
* [Why the developers who use Rust love it so much](https://stackoverflow.blog/2020/06/05/why-the-developers-who-use-rust-love-it-so-much/)

## Rust community
{id: rust-community}

* [Rust User groups](https://rust.code-maven.com/user-groups)
* [Rust social status](https://rust.code-maven.com/rust-update-2024-06-17)

## What is written in Rust?
{id: what-is-written-in-rust}

* [Pydentic](https://docs.pydantic.dev/), the data validation library for Python.
* [ruff](https://github.com/astral-sh/ruff), a linter for Python.
* [SurrealDB](https://surrealdb.com/), a multi-model database.
* ...

## Demo None handling
{id: demo-none-handling}

![](examples/intro/no-null/src/main.rs)

## Demo None handling with Option
{id: demo-none-handling-with-option}
{i: Option}
{i: match}
{i: Some}
{i: None}

![](examples/intro/demo-none-handling/src/main.rs)


See original idea on [What is Rust and why is it so popular?](https://stackoverflow.blog/2020/01/20/what-is-rust-and-why-is-it-so-popular/)


## Install Rust
{id: rust-installation}

See in the [Rust book](https://doc.rust-lang.org/stable/book/ch01-01-installation.html)

```
rustup --version

rustup 1.26.0 (5af9b9484 2023-04-05)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.70.0 (90c541806 2023-05-31)`
```

```
rustc --version

rustc 1.70.0 (90c541806 2023-05-31)
```


```
$ rustc -vV
rustc 1.79.0 (129f3b996 2024-06-10)
binary: rustc
commit-hash: 129f3b9964af4d4a709d1383930ade12dfe7c081
commit-date: 2024-06-10
host: x86_64-unknown-linux-gnu
release: 1.79.0
LLVM version: 18.1.7
```

## Editor and IDE
{id: rust-editor-and-ide}

* Visual Studio Code with the `rust-analyzer` plugin. [rust-analyzer](https://rust-analyzer.github.io/) is an implementation of [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) for the Rust.
* vim


## Hello World
{id: rust-hello-world}
{i: fn}
{i: main}
{i: println!}

* Rust files must have an extension of `.rs`.
* The main Rust file must have a function called `main`.
* `println!` is a [macro](https://doc.rust-lang.org/book/ch19-06-macros.html) (it looks like function, it generates some Rust code during compilation).

![](examples/intro/hello/src/main.rs)

```
rustc src/main.rs
```

```
./main
```


## Hello World with Cargo
{id: hello-world-with-cargo}
{i: cargo}

* Cargo is the package management system of Rust
* `cargo new hello-world` creates a new folder called `hello-world`
* With a file called `Cargo.toml`
* A folder called `src` and  a file `src/main.rs` with the hello world code.
* `cargo run` will comple the code and run it.

```
cargo new hello-world
cd hello-world
```

![](examples/intro/hello-world/src/main.rs)
![](examples/intro/hello-world/Cargo.toml)

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

## Cargo build
{id: cargo-build}
{i: build}
{i: target}
{i: --release}

* Look in the `target` folder.

```
cargo build
cargo build --release
```

## Run in release mode
{id: cargo-run-release}

```
cargo run --release
```

## Use of macros with parentheses, square brackets, or curly braces
{id: use-of-macros}

* In our first example we used the `println!` macro with parentheses: `println!()`
* We can also use macros with square brackets and curly braces.
* [Macros - the reference](https://doc.rust-lang.org/reference/macros.html)

![](examples/intro/use-macro/src/main.rs)


## Rust and comments
{id: rust-comments}
{i: //}
{i: /* */}

* Both single-line and multi-line comments are available in Rust

* [spec of comments](https://doc.rust-lang.org/reference/comments.html)

![](examples/intro/comments/src/main.rs)

## Rust - Hello Foo
{id: hello-foo}
{i: let}

![](examples/intro/hello-foo/src/main.rs)

![](examples/intro/hello-foo/out.out)

* [format! macro](https://doc.rust-lang.org/std/fmt/)

## Interpolation
{id: string-interpolation}

Since Rust 1.58

![](examples/intro/interpolation/src/main.rs)

![](examples/intro/interpolation/out.out)

## Printing a string
{id: printing-a-string}

![](examples/intro/formatting-required/src/main.rs)

![](examples/intro/formatting-required/out.out)

## Printing a string fixed
{id: printing-a-string-fixed}

* With or without interpolation

![](examples/intro/formatting-required-fixed/src/main.rs)

![](examples/intro/formatting-required-fixed/out.out)

## Debugging print
{id: debugging-print}
{i: dbg!}

* goes to STDERR

![](examples/intro/debugging-print/src/main.rs)

![](examples/intro/debugging-print/out.out)

## Rust and print
{id: rust-and-print}
{i: print!}
{i: println!}
{i: eprint!}
{i: eprintln!}
{i: dbg!}
{i: format!}


* [print!](https://doc.rust-lang.org/std/macro.print.html)    to STDOUT
* [println!](https://doc.rust-lang.org/std/macro.println.html)  to STDOUT

* [eprint!](https://doc.rust-lang.org/std/macro.eprint.html)   to STDERR
* [eprintln!](https://doc.rust-lang.org/std/macro.eprintln.html) to STDERR
* [dbg!](https://doc.rust-lang.org/std/macro.dbg.html)      to STDERR

* [format!](https://doc.rust-lang.org/std/macro.format.html)   returns formatted string

![](examples/intro/print/src/main.rs)
![](examples/intro/print/out.out)


## Exercise: Hello World
{id: exercise-helo-world}

* Make sure you have Rust installed.
* Try `rustc --version`
* Create a new project with Cargo.
* Write a program that prints "Hello World".
* run the program.
* Add comments to the program and run it again.
* Extend the program to have your name in a variable and print that too.

